use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
// use cdrs::query::*;
use cdrs::query::{QueryValues,QueryExecutor};
use cdrs::frame::Frame;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::ByName;
use std::collections::HashMap;
use std::result::Result; // override prelude Result

use crate::xc::common::*;


#[derive(Default, Clone, Debug, PartialEq)]
pub struct Tweet {
    pub tweet_id: String,   // tweet_id    clustering  0
    pub user_id: i64,   // user_id    partition_key  0
    pub body: String,   // body    regular  -1
    pub create_time: i32,   //G create_time    regular  -1
    
    _exists: bool,
    _deleted: bool,
}

impl Tweet {
    pub fn deleted(&self) -> bool {
        self._deleted
    }

    pub fn exists(&self) -> bool {
        self._exists
    }

    pub fn save(&mut self, session: &CurrentSession) -> Result<(),CWError> {
        let mut columns = vec![];
        let mut values :Vec<Value> = vec![];

        
		if !self.tweet_id.is_empty() {
            columns.push("tweet_id");
            values.push(self.tweet_id.clone().into());
       	}

		if self.user_id != 0i64 {
            columns.push("user_id");
            values.push(self.user_id.clone().into());
       	}

		if !self.body.is_empty() {
            columns.push("body");
            values.push(self.body.clone().into());
       	}

		if self.create_time != 0i32 {
            columns.push("create_time");
            values.push(self.create_time.clone().into());
       	}


        if columns.len() == 0 {
            return Err(CWError::InvalidCQL)
        }

        let cql_columns = columns.join(", ");
        let mut cql_question = "?,".repeat(columns.len());
        cql_question.remove(cql_question.len()-1);

        let cql_query = format!("INSERT INTO twitter.tweet ({}) VALUES ({})", cql_columns, cql_question);

        println!("{} - {}", &cql_query, &cql_question);

        session.query_with_values(cql_query, values)?;

        Ok(())
    }

    pub fn delete(&mut self, session: &CurrentSession) -> Result<(), CWError> {
        let mut deleter = Tweet_Deleter::new();
      
        deleter.user_id_eq(self.user_id);
    
        deleter.and_tweet_id_eq(&self.tweet_id);

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
pub struct Tweet_Selector {
    wheres: Vec<WhereClause>,
    select_cols: Vec<&'static str>,
    order_by: Vec<&'static str>,
    limit: u32,
    allow_filter: bool,
}

impl Tweet_Selector {
    pub fn new() -> Self {
        Tweet_Selector::default()
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
    pub fn select_tweet_id(&mut self) -> &mut Self {
        self.select_cols.push("tweet_id");
        self
    }
    
    pub fn select_user_id(&mut self) -> &mut Self {
        self.select_cols.push("user_id");
        self
    }
    
    pub fn select_body(&mut self) -> &mut Self {
        self.select_cols.push("body");
        self
    }
    
    pub fn select_create_time(&mut self) -> &mut Self {
        self.select_cols.push("create_time");
        self
    }
    

    pub fn _to_cql(&self) ->  (String, Vec<Value>)  {
        let cql_select = if self.select_cols.is_empty() {
            "*".to_string()
        } else {
            self.select_cols.join(", ")
        };

        let mut cql_query = format!("SELECT {} FROM twitter.tweet", cql_select);

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

    pub fn _get_rows_with_size(&mut self,session: &CurrentSession, size: i64) -> Result<Vec<Tweet>, CWError>   {

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
            let mut row = Tweet::default();
            
            row.tweet_id = db_row.by_name("tweet_id")?.unwrap_or_default();
            row.user_id = db_row.by_name("user_id")?.unwrap_or_default();
            row.body = db_row.by_name("body")?.unwrap_or_default();
            row.create_time = db_row.by_name("create_time")?.unwrap_or_default();

            rows.push(row);
        }

        Ok(rows)
    }

    pub fn get_rows(&mut self, session: &CurrentSession) -> Result<Vec<Tweet>, CWError>{
        self._get_rows_with_size(session,-1)
    }

    pub fn get_row(&mut self, session: &CurrentSession) -> Result<Tweet, CWError>{
        let rows = self._get_rows_with_size(session,1)?;

        let opt = rows.get(0);
        match opt {
            Some(row) => Ok(row.to_owned()),
            None => Err(CWError::NotFound)
        }
    }

    
    pub fn order_by_tweet_id_asc(&mut self) -> &mut Self {
		self.order_by.push("tweet_id ASC");
        self
    }

	pub fn order_by_tweet_id_desc(&mut self) -> &mut Self {
		self.order_by.push("tweet_id DESC");
        self
    }


    
    pub fn tweet_id_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn body_eq_filtering (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_body_eq_filtering (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_body_eq_filtering (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_eq_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_lt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_le_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_gt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_ge_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_eq_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_lt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_le_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_gt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_ge_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_eq_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_lt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_le_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_gt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_ge_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    
    pub fn tweet_id_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" tweet_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND tweet_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR tweet_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" user_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND user_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR user_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn body_in_filtering (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" body IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_body_in_filtering (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND body IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_body_in_filtering (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR body IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_in_filtering (&mut self, val: Vec<i32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" create_time IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_in_filtering (&mut self, val: Vec<i32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND create_time IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_in_filtering (&mut self, val: Vec<i32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR create_time IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


}


#[derive(Default, Debug)]
pub struct Tweet_Deleter {
    wheres: Vec<WhereClause>,
    delete_cols: Vec<&'static str>,
}

#[derive(Default, Debug)]
pub struct Tweet_Updater {
    wheres: Vec<WhereClause>,
    updates: HashMap<&'static str, Value>,
}

impl Tweet_Updater {
    pub fn new() -> Self {
        Tweet_Updater::default()
    }

    pub fn update(&mut self,session: &CurrentSession) -> cdrs::error::Result<Frame>  {
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
            format!("UPDATE twitter.tweet SET {}", cql_update)
        } else {
            format!("UPDATE twitter.tweet SET {} WHERE {}", cql_update, cql_where)
        };

        let query_values = QueryValues::SimpleValues(all_vals);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)
    }

    
    pub fn update_tweet_id(&mut self, val: &str) -> &mut Self {
        self.updates.insert("tweet_id = ?", val.into());
        self
    }

    pub fn update_user_id(&mut self, val: i64) -> &mut Self {
        self.updates.insert("user_id = ?", val.into());
        self
    }

    pub fn update_body(&mut self, val: &str) -> &mut Self {
        self.updates.insert("body = ?", val.into());
        self
    }

    pub fn update_create_time(&mut self, val: i32) -> &mut Self {
        self.updates.insert("create_time = ?", val.into());
        self
    }


    
    pub fn tweet_id_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn body_eq_filtering (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_body_eq_filtering (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_body_eq_filtering (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_eq_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_lt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_le_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_gt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_ge_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_eq_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_lt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_le_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_gt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_ge_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_eq_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_lt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_le_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_gt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_ge_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    
    pub fn tweet_id_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" tweet_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND tweet_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR tweet_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" user_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND user_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR user_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn body_in_filtering (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" body IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_body_in_filtering (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND body IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_body_in_filtering (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR body IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_in_filtering (&mut self, val: Vec<i32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" create_time IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_in_filtering (&mut self, val: Vec<i32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND create_time IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_in_filtering (&mut self, val: Vec<i32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR create_time IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

}

impl Tweet_Deleter {
    pub fn new() -> Self {
        Tweet_Deleter::default()
    }

    //each column delete
    pub fn delete_tweet_id(&mut self) -> &mut Self {
        self.delete_cols.push("tweet_id");
        self
    }
    
    pub fn delete_user_id(&mut self) -> &mut Self {
        self.delete_cols.push("user_id");
        self
    }
    
    pub fn delete_body(&mut self) -> &mut Self {
        self.delete_cols.push("body");
        self
    }
    
    pub fn delete_create_time(&mut self) -> &mut Self {
        self.delete_cols.push("create_time");
        self
    }
    

    pub fn delete(&mut self, session: &CurrentSession) -> Result<(),CWError> {
        let del_col = self.delete_cols.join(", ");

        let  mut where_str = vec![];
        let mut where_arr = vec![];

        for w in self.wheres.clone() {
            where_str.push(w.condition);
            where_arr.push(w.args)
        }

        let where_str = where_str.join(" ");

        let cql_query = format!("DELETE {} FROM twitter.tweet WHERE {}", del_col, where_str);
        //let cql_query = "DELETE " + del_col + " FROM twitter.tweet WHERE " + where_str ;

        let query_values = QueryValues::SimpleValues(where_arr);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)?;

        Ok(())
    }

    
    pub fn tweet_id_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_eq (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_lt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_le_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_gt_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_ge_filtering (&mut self, val: i64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn body_eq_filtering (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_body_eq_filtering (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_body_eq_filtering (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_eq_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_lt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_le_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_gt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_ge_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: " create_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_eq_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_lt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_le_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_gt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_ge_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND create_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_eq_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_lt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_le_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_gt_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_ge_filtering (&mut self, val: i32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR create_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    
    pub fn tweet_id_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" tweet_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND tweet_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR tweet_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" user_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND user_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_in (&mut self, val: Vec<i64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR user_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn body_in_filtering (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" body IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_body_in_filtering (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND body IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_body_in_filtering (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR body IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn create_time_in_filtering (&mut self, val: Vec<i32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!(" create_time IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_create_time_in_filtering (&mut self, val: Vec<i32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("AND create_time IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_create_time_in_filtering (&mut self, val: Vec<i32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);
        let w = WhereClause{
			condition: format!("OR create_time IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

}




