use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
// use cdrs::query::*;
use cdrs::query::{QueryValues,QueryExecutor};
use cdrs::frame::Frame;
use cdrs::types::value::ValueType;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::ByName;
use std::collections::HashMap;
use std::result::Result; // override prelude Result

//use cdrs::error::{Error as CWError};
use cdrs::frame::frame_error::CDRSError;
use cdrs::Error as DriverError;
use crate::xc::common::*;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Chat {
    pub chat_gid: i64,   // chat_gid    clustering  0
    pub profile_cid: i64,   // profile_cid    partition_key  0
    pub pb_data: Vec<u8>,   // pb_data    regular  -1
}

impl Chat {
    pub fn save(&self, session: impl FCQueryExecutor) -> Result<(),CWError> {
        let mut columns = vec![];
        let mut values :Vec<Value> = vec![];

        
		// partition key and clustering key always must be present
		columns.push("chat_gid");
        values.push(self.chat_gid.clone().into());

		// partition key and clustering key always must be present
		columns.push("profile_cid");
        values.push(self.profile_cid.clone().into());

		if !self.pb_data.is_empty() {
            let val = Value{
                body: self.pb_data.clone(),
                value_type: ValueType::Normal(self.pb_data.len() as i32)
            };

            columns.push("pb_data");
            values.push(val);
       	}


        if columns.len() == 0 {
            return Err(CWError::InvalidCQL)
        }

        let cql_columns = columns.join(", ");
        let mut cql_question = "?,".repeat(columns.len());
        cql_question.remove(cql_question.len()-1);

        let cql_query = format!("INSERT INTO flip.chat ({}) VALUES ({})", cql_columns, cql_question);

        println!("{} - {}", &cql_query, &cql_question);

        session.query_with_values(cql_query, values)?;

        Ok(())
    }

    pub fn delete(&self, session: impl FCQueryExecutor) -> Result<(), CWError> {
        let mut deleter = Chat_Deleter::new();
      
        deleter.profile_cid_eq(self.profile_cid);
    
        deleter.and_chat_gid_eq(self.chat_gid);

        let res = deleter.delete(session)?;

        Ok(())
    }

}

fn _get_where(wheres: Vec<WhereClause>) ->  (String, Vec<Value>) {
    let mut values = vec![];
    let  mut where_str = vec![];

    for w in wheres {
        where_str.push(w.condition);
        values.push(w.args)
    }
    let cql_where = where_str.join(" ");

    (cql_where, values)
}

#[derive(Default, Debug)]
pub struct Chat_Selector {
    wheres: Vec<WhereClause>,
    select_cols: Vec<&'static str>,
    order_by: Vec<&'static str>,
    limit: u32,
    allow_filter: bool,
}

impl Chat_Selector {
    pub fn new() -> Self {
        Chat_Selector::default()
    }

    pub fn limit(&mut self, size: u32) -> &mut Self {
        self.limit = size;
        self
    }

    pub fn allow_filtering(&mut self, allow: bool) -> &mut Self {
        self.allow_filter = allow;
        self
    }

    pub fn select_all(&mut self) -> &mut Self {
        // Default is select *
        self
    }

    //each column select
    pub fn select_chat_gid(&mut self) -> &mut Self {
        self.select_cols.push("chat_gid");
        self
    }
    
    pub fn select_profile_cid(&mut self) -> &mut Self {
        self.select_cols.push("profile_cid");
        self
    }
    
    pub fn select_pb_data(&mut self) -> &mut Self {
        self.select_cols.push("pb_data");
        self
    }
    

    pub fn _to_cql(&self) ->  (String, Vec<Value>)  {
        let cql_select = if self.select_cols.is_empty() {
            "*".to_string()
        } else {
            self.select_cols.join(", ")
        };

        let mut cql_query = format!("SELECT {} FROM flip.chat", cql_select);

        let (cql_where, where_values) = _get_where(self.wheres.clone());

        if where_values.len() > 0 {
            cql_query.push_str(&format!(" WHERE {}",&cql_where));
        }

        if self.order_by.len() > 0 {
            let cql_orders = self.order_by.join(", ");
            cql_query.push_str( &format!(" ORDER BY {}", &cql_orders));
        };

        if self.limit != 0  {
            cql_query.push_str(&format!(" LIMIT {} ", self.limit));
        };

        if self.allow_filter  {
            cql_query.push_str(" ALLOW FILTERING");
        };

        (cql_query, where_values)
    }

    pub fn _get_rows_with_size(&mut self,session: impl FCQueryExecutor, size: i64) -> Result<Vec<Chat>, CWError>   {

        let(cql_query, query_values) = self._to_cql();

        println!("{} - {:?}", &cql_query, &query_values);

        let query_result = session
            .query_with_values(cql_query,query_values)?
            .get_body()?
            .into_rows();

        let db_raws = match query_result {
            Some(rs) => {
                if size > 0 {
                    if rs.len() == size as usize {
                        rs
                    } else {
                        let min = (size as usize).min(rs.len());
                        rs[0..min].to_vec()
                    }
                } else {
                    rs
                }
            },
            None => return Err(CWError::NotFound)
        };

        let mut rows = vec![];

        for db_row in db_raws {
            let mut row = Chat::default();
            
                
            row.chat_gid = db_row.by_name("chat_gid")?.unwrap_or_default();
                
            row.profile_cid = db_row.by_name("profile_cid")?.unwrap_or_default();
                
            row.pb_data = db_row.by_name::<Blob>("pb_data")?.unwrap_or(Blob::new(vec![])).into_vec();

            rows.push(row);
        }

        Ok(rows)
    }

    pub fn get_rows(&mut self, session: impl FCQueryExecutor) -> Result<Vec<Chat>, CWError>{
        self._get_rows_with_size(session,-1)
    }

    pub fn get_row(&mut self, session: impl FCQueryExecutor) -> Result<Chat, CWError>{
        let rows = self._get_rows_with_size(session,1)?;

        let opt = rows.get(0);
        match opt {
            Some(row) => Ok(row.to_owned()),
            None => Err(CWError::NotFound)
        }
    }

    
    pub fn order_by_chat_gid_asc(&mut self) -> &mut Self {
		self.order_by.push("chat_gid ASC");
        self
    }

	pub fn order_by_chat_gid_desc(&mut self) -> &mut Self {
		self.order_by.push("chat_gid DESC");
        self
    }


    
    pub fn chat_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    
    pub fn chat_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" chat_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND chat_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR chat_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


}


#[derive(Default, Debug)]
pub struct Chat_Deleter {
    wheres: Vec<WhereClause>,
    delete_cols: Vec<&'static str>,
}

#[derive(Default, Debug)]
pub struct Chat_Updater {
    wheres: Vec<WhereClause>,
    updates: HashMap<&'static str, Value>,
}

impl Chat_Updater {
    pub fn new() -> Self {
        Chat_Updater::default()
    }

    pub fn update(&mut self,session: impl FCQueryExecutor) -> cdrs::error::Result<Frame>  {
        if self.updates.is_empty() {
            return Err(cdrs::error::Error::General("empty".to_string()));
        }

        // Update columns building
        let mut all_vals = vec![];
        let mut col_updates = vec![];

        for (col,val) in self.updates.clone() {
            all_vals.push(val);
            col_updates.push(col);
        }
        let cql_update = col_updates.join(",");

        // Where columns building
        let  mut where_str = vec![];

        for w in self.wheres.clone() {
            where_str.push(w.condition);
            all_vals.push(w.args)
        }
        let cql_where = where_str.join(" ");

        // Build final query
        let mut cql_query = if self.wheres.is_empty() {
            format!("UPDATE flip.chat SET {}", cql_update)
        } else {
            format!("UPDATE flip.chat SET {} WHERE {}", cql_update, cql_where)
        };

        let query_values = QueryValues::SimpleValues(all_vals);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)
    }

    
    pub fn update_chat_gid(&mut self, val: i64) -> &mut Self {
        self.updates.insert("chat_gid = ?", val.into());
        self
    }

    pub fn update_profile_cid(&mut self, val: i64) -> &mut Self {
        self.updates.insert("profile_cid = ?", val.into());
        self
    }

    pub fn update_pb_data(&mut self, val: &Vec<u8>) -> &mut Self {
        self.updates.insert("pb_data = ?", Blob::new(val.clone()).into());
        self
    }


    
    pub fn chat_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    
    pub fn chat_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" chat_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND chat_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR chat_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

}

impl Chat_Deleter {
    pub fn new() -> Self {
        Chat_Deleter::default()
    }

    //each column delete
    pub fn delete_chat_gid(&mut self) -> &mut Self {
        self.delete_cols.push("chat_gid");
        self
    }
    
    pub fn delete_profile_cid(&mut self) -> &mut Self {
        self.delete_cols.push("profile_cid");
        self
    }
    
    pub fn delete_pb_data(&mut self) -> &mut Self {
        self.delete_cols.push("pb_data");
        self
    }
    

    pub fn delete(&mut self, session: impl FCQueryExecutor) -> Result<(),CWError> {
        let del_col = self.delete_cols.join(", ");

        let  mut where_str = vec![];
        let mut where_arr = vec![];

        for w in self.wheres.clone() {
            where_str.push(w.condition);
            where_arr.push(w.args)
        }

        let where_str = where_str.join(" ");

        let cql_query = format!("DELETE {} FROM flip.chat WHERE {}", del_col, where_str);
        //let cql_query = "DELETE " + del_col + " FROM flip.chat WHERE " + where_str ;

        let query_values = QueryValues::SimpleValues(where_arr);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)?;

        Ok(())
    }

    
    pub fn chat_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " chat_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND chat_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR chat_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    
    pub fn chat_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" chat_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND chat_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR chat_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

}


pub fn get_chat_by_profile_cid_and_chat_gid(session: impl FCQueryExecutor, p1:i64,p2:i64) -> Result<Chat,CWError> {
	let m = Chat_Selector::new()
		.profile_cid_eq(p1)
		.and_chat_gid_eq(p2)
		.get_row(session)?;
	Ok(m)
}



