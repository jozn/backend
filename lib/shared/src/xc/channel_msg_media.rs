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
pub struct ChannelMsgMedia {
    pub msg_gid: i64,   // msg_gid    clustering  1
    pub channel_cid: i64,   // channel_cid    partition_key  0
    pub media_type_id: i64,   // media_type_id    clustering  0
}

impl ChannelMsgMedia {
    pub fn save(&self, session: impl FCQueryExecutor) -> Result<(),CWError> {
        let mut columns = vec![];
        let mut values :Vec<Value> = vec![];

        
		// partition key and clustering key always must be present
		columns.push("msg_gid");
        values.push(self.msg_gid.clone().into());

		// partition key and clustering key always must be present
		columns.push("channel_cid");
        values.push(self.channel_cid.clone().into());

		// partition key and clustering key always must be present
		columns.push("media_type_id");
        values.push(self.media_type_id.clone().into());


        if columns.len() == 0 {
            return Err(CWError::InvalidCQL)
        }

        let cql_columns = columns.join(", ");
        let mut cql_question = "?,".repeat(columns.len());
        cql_question.remove(cql_question.len()-1);

        let cql_query = format!("INSERT INTO flip.channel_msg_media ({}) VALUES ({})", cql_columns, cql_question);

        println!("{} - {}", &cql_query, &cql_question);

        session.query_with_values(cql_query, values)?;

        Ok(())
    }

    pub fn delete(&self, session: impl FCQueryExecutor) -> Result<(), CWError> {
        let mut deleter = ChannelMsgMedia_Deleter::new();
      
        deleter.channel_cid_eq(self.channel_cid);
    
        deleter.and_msg_gid_eq(self.msg_gid);
        deleter.and_media_type_id_eq(self.media_type_id);

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
pub struct ChannelMsgMedia_Selector {
    wheres: Vec<WhereClause>,
    select_cols: Vec<&'static str>,
    order_by: Vec<&'static str>,
    limit: u32,
    allow_filter: bool,
}

impl ChannelMsgMedia_Selector {
    pub fn new() -> Self {
        ChannelMsgMedia_Selector::default()
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
    pub fn select_msg_gid(&mut self) -> &mut Self {
        self.select_cols.push("msg_gid");
        self
    }
    
    pub fn select_channel_cid(&mut self) -> &mut Self {
        self.select_cols.push("channel_cid");
        self
    }
    
    pub fn select_media_type_id(&mut self) -> &mut Self {
        self.select_cols.push("media_type_id");
        self
    }
    

    pub fn _to_cql(&self) ->  (String, Vec<Value>)  {
        let cql_select = if self.select_cols.is_empty() {
            "*".to_string()
        } else {
            self.select_cols.join(", ")
        };

        let mut cql_query = format!("SELECT {} FROM flip.channel_msg_media", cql_select);

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

    pub fn _get_rows_with_size(&mut self,session: impl FCQueryExecutor, size: i64) -> Result<Vec<ChannelMsgMedia>, CWError>   {

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
            let mut row = ChannelMsgMedia::default();
            
                
            row.msg_gid = db_row.by_name("msg_gid")?.unwrap_or_default();
                
            row.channel_cid = db_row.by_name("channel_cid")?.unwrap_or_default();
                
            row.media_type_id = db_row.by_name("media_type_id")?.unwrap_or_default();

            rows.push(row);
        }

        Ok(rows)
    }

    pub fn get_rows(&mut self, session: impl FCQueryExecutor) -> Result<Vec<ChannelMsgMedia>, CWError>{
        self._get_rows_with_size(session,-1)
    }

    pub fn get_row(&mut self, session: impl FCQueryExecutor) -> Result<ChannelMsgMedia, CWError>{
        let rows = self._get_rows_with_size(session,1)?;

        let opt = rows.get(0);
        match opt {
            Some(row) => Ok(row.to_owned()),
            None => Err(CWError::NotFound)
        }
    }

    
    pub fn order_by_msg_gid_asc(&mut self) -> &mut Self {
		self.order_by.push("msg_gid ASC");
        self
    }

	pub fn order_by_msg_gid_desc(&mut self) -> &mut Self {
		self.order_by.push("msg_gid DESC");
        self
    }

    pub fn order_by_media_type_id_asc(&mut self) -> &mut Self {
		self.order_by.push("media_type_id ASC");
        self
    }

	pub fn order_by_media_type_id_desc(&mut self) -> &mut Self {
		self.order_by.push("media_type_id DESC");
        self
    }


    
    pub fn msg_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    
    pub fn msg_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" msg_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND msg_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR msg_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" media_type_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND media_type_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR media_type_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


}


#[derive(Default, Debug)]
pub struct ChannelMsgMedia_Deleter {
    wheres: Vec<WhereClause>,
    delete_cols: Vec<&'static str>,
}

#[derive(Default, Debug)]
pub struct ChannelMsgMedia_Updater {
    wheres: Vec<WhereClause>,
    updates: HashMap<&'static str, Value>,
}

impl ChannelMsgMedia_Updater {
    pub fn new() -> Self {
        ChannelMsgMedia_Updater::default()
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
            format!("UPDATE flip.channel_msg_media SET {}", cql_update)
        } else {
            format!("UPDATE flip.channel_msg_media SET {} WHERE {}", cql_update, cql_where)
        };

        let query_values = QueryValues::SimpleValues(all_vals);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)
    }

    
    pub fn update_msg_gid(&mut self, val: i64) -> &mut Self {
        self.updates.insert("msg_gid = ?", val.into());
        self
    }

    pub fn update_channel_cid(&mut self, val: i64) -> &mut Self {
        self.updates.insert("channel_cid = ?", val.into());
        self
    }

    pub fn update_media_type_id(&mut self, val: i64) -> &mut Self {
        self.updates.insert("media_type_id = ?", val.into());
        self
    }


    
    pub fn msg_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    
    pub fn msg_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" msg_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND msg_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR msg_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" media_type_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND media_type_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR media_type_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

}

impl ChannelMsgMedia_Deleter {
    pub fn new() -> Self {
        ChannelMsgMedia_Deleter::default()
    }

    //each column delete
    pub fn delete_msg_gid(&mut self) -> &mut Self {
        self.delete_cols.push("msg_gid");
        self
    }
    
    pub fn delete_channel_cid(&mut self) -> &mut Self {
        self.delete_cols.push("channel_cid");
        self
    }
    
    pub fn delete_media_type_id(&mut self) -> &mut Self {
        self.delete_cols.push("media_type_id");
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

        let cql_query = format!("DELETE {} FROM flip.channel_msg_media WHERE {}", del_col, where_str);
        //let cql_query = "DELETE " + del_col + " FROM flip.channel_msg_media WHERE " + where_str ;

        let query_values = QueryValues::SimpleValues(where_arr);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)?;

        Ok(())
    }

    
    pub fn msg_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " msg_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND msg_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR msg_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " media_type_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND media_type_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_lt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_le (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_gt (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_ge (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR media_type_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    
    pub fn msg_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" msg_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND msg_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_gid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR msg_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn media_type_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" media_type_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_media_type_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND media_type_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_media_type_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR media_type_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

}


pub fn get_channel_msg_media_by_channel_cid_and_msg_gid_and_media_type_id(session: impl FCQueryExecutor, p1:i64,p2:i64,p3:i64) -> Result<ChannelMsgMedia,CWError> {
	let m = ChannelMsgMedia_Selector::new()
		.channel_cid_eq(p1)
		.and_msg_gid_eq(p2)
		.and_media_type_id_eq(p3)
		.get_row(session)?;
	Ok(m)
}



