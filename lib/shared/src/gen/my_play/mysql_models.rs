// DO NOT MODIFY AUTO-GENERATED BY PB-WALKER
use mysql_async::prelude::*;
use mysql_async::{FromRowError, OptsBuilder, Params, Row, Pool};
use mysql_common::row::ColumnIndex;

use mysql_common::value::Value;

//use crate::xc::CWError;

// Every Table Must Have Primary Keys to Be Included In This Output
// Primiay Keys must be one column (no compostion types yet)
// Primiay Keys can be 1) Auto Increment 2) Other self Inserted

// Implemention is simple NOT many features is suported in Rust version:
// Keep mysql data types in int, bigint, text, varchar, bool, blob
// No signed integer is supported
// For now Primary key should only be numbers
// Not fully ORM is supported: limited to CRUD on rows + Indexes querys



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tweet  { // tweet
    pub tweet_id: u64,
    pub user_id: u32,
    pub created_time: u64,
    pub text_body: String,
}

impl FromRow for Tweet {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        Ok(Tweet  {
            tweet_id: row.get(0).unwrap_or_default(),
            user_id: row.get(1).unwrap_or_default(),
            created_time: row.get(2).unwrap_or_default(),
            text_body: row.get(3).unwrap_or_default(),
        })
    }
}

impl Tweet {
    pub async fn insert(&self, pool: &Pool) -> Result<Tweet,MyError> {
        let mut conn = pool.get_conn().await?;

        let query = r"INSERT INTO `twitter`.`tweet` (user_id, created_time, text_body) VALUES (?, ?, ?)";
        let p = Params::Positional(vec![self.user_id.clone().into(), self.created_time.clone().into(), self.text_body.clone().into()]);

        let qr = conn.exec_iter(
            query, p
        ).await?;

        let mut cp = self.clone();
        cp.tweet_id = qr.last_insert_id().unwrap() as u64;
        Ok(cp)
    }


    pub async fn replace_dep(&self, pool: &Pool) -> Result<Tweet,MyError> {
        let mut conn = pool.get_conn().await?;

        let query = r"INSERT INTO `twitter`.`tweet` (user_id, created_time, text_body) VALUES (?, ?, ?)";
        let p = Params::Positional(vec![self.user_id.clone().into(), self.created_time.clone().into(), self.text_body.clone().into()]);

        let qr = conn.exec_iter(
            query, p
        ).await?;

        let mut cp = self.clone();
        cp.tweet_id = qr.last_insert_id().unwrap() as u64;

       Ok(cp)
    }

    pub async fn update(&self, pool: &Pool) -> Result<(),MyError> {
        let mut conn = pool.get_conn().await?;
        let query = r"UPDATE `twitter`.`tweet` SET user_id = ?, created_time = ?, text_body = ? WHERE  ";
        let p = Params::Positional(vec![self.user_id.clone().into(), self.created_time.clone().into(), self.text_body.clone().into(),  self.tweet_id.clone().into() ]);

        let qr = conn.exec_iter(
            query, p
        ).await?;

        Ok(())
    }

    pub async fn delete(&self, pool: &Pool) -> Result<(),MyError> {
        let mut conn = pool.get_conn().await?;

        let query = r"DELETE FROM `twitter`.`tweet` WHERE  ";
        let p = Params::Positional(vec![self.tweet_id.clone().into()]);

        conn.exec_drop(
            query, p
        ).await?;

        Ok(())
    }
}


#[derive(Default, Debug)]
pub struct TweetSelector {
    wheres: Vec<WhereClause>,
    select_cols: Vec<&'static str>,
    order_by:  Vec<&'static str>,
    limit: u32,
    offset: u32,
}

impl TweetSelector {
    pub fn new() -> Self {
        TweetSelector::default()
    }

    pub fn limit(&mut self, size: u32) -> &mut Self {
        self.limit = size;
        self
    }

    pub fn offset(&mut self, size: u32) -> &mut Self {
        self.offset = size;
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
    
    pub fn select_created_time(&mut self) -> &mut Self {
        self.select_cols.push("created_time");
        self
    }
    
    pub fn select_text_body(&mut self) -> &mut Self {
        self.select_cols.push("text_body");
        self
    }
    

    fn _to_cql(&self) ->  (String, Vec<Value>)  {
        let cql_select = if self.select_cols.is_empty() {
            "*".to_string()
        } else {
            self.select_cols.join(", ")
        };

        let mut cql_query = format!("SELECT {} FROM `twitter`.`tweet`", cql_select);

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

        if self.offset != 0  {
            cql_query.push_str(&format!(" OFFSET {} ", self.offset));
        };

        (cql_query, where_values)
    }

    pub async fn _get_rows_with_size(&mut self, session: &Pool, size: i64) -> Result<Vec<Tweet>, MyError>   {
        let mut conn = session.get_conn().await?;
        let(cql_query, query_values) = self._to_cql();

        println!("{} - {:?}", &cql_query, &query_values);

        let p = Params::Positional(query_values);

        let query_result = conn
            .exec_map(
                cql_query,p,
                |obj: Tweet| obj
            ).await?;

        Ok(query_result)
    }

    pub async fn get_rows(&mut self, session: &Pool) -> Result<Vec<Tweet>, MyError>{
        self._get_rows_with_size(session,-1).await
    }

    pub async fn get_row(&mut self, session: &Pool) -> Result<Tweet, MyError>{
        let rows = self._get_rows_with_size(session,1).await?;

        let opt = rows.get(0);
        match opt {
            Some(row) => Ok(row.to_owned()),
            None => Err(MyError::NotFound)
        }
    }

    // Modifiers

    
    pub fn order_by_tweet_id_asc(&mut self) -> &mut Self {
		self.order_by.push("tweet_id ASC");
        self
    }

	pub fn order_by_tweet_id_desc(&mut self) -> &mut Self {
		self.order_by.push("tweet_id DESC");
        self
    }

    pub fn order_by_user_id_asc(&mut self) -> &mut Self {
		self.order_by.push("user_id ASC");
        self
    }

	pub fn order_by_user_id_desc(&mut self) -> &mut Self {
		self.order_by.push("user_id DESC");
        self
    }

    pub fn order_by_created_time_asc(&mut self) -> &mut Self {
		self.order_by.push("created_time ASC");
        self
    }

	pub fn order_by_created_time_desc(&mut self) -> &mut Self {
		self.order_by.push("created_time DESC");
        self
    }

    pub fn order_by_text_body_asc(&mut self) -> &mut Self {
		self.order_by.push("text_body ASC");
        self
    }

	pub fn order_by_text_body_desc(&mut self) -> &mut Self {
		self.order_by.push("text_body DESC");
        self
    }


    
    pub fn tweet_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn tweet_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " tweet_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn tweet_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " tweet_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn tweet_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " tweet_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn tweet_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " tweet_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND tweet_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND tweet_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND tweet_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND tweet_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR tweet_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR tweet_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR tweet_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR tweet_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR tweet_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn user_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_user_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_user_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR user_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn created_time_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " created_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn created_time_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " created_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn created_time_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " created_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn created_time_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " created_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn created_time_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " created_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_created_time_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND created_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_created_time_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND created_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_created_time_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND created_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_created_time_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND created_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_created_time_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND created_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_created_time_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR created_time = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_created_time_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR created_time < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_created_time_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR created_time <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_created_time_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR created_time > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_created_time_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR created_time >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn text_body_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " text_body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn text_body_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " text_body < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn text_body_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " text_body <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn text_body_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " text_body > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn text_body_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " text_body >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_text_body_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND text_body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_text_body_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND text_body < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_text_body_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND text_body <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_text_body_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND text_body > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_text_body_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND text_body >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_text_body_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR text_body = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_text_body_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR text_body < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_text_body_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR text_body <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_text_body_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR text_body > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_text_body_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR text_body >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }


    //{* .GetRustWhereInsTmplOut *}

}



///////////////// SHARED CODE ///////////
#[derive(Debug, Clone)]
pub struct WhereClause {
    // pub condition: &'static str,
    pub condition: String,
    pub args: Value,
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

#[derive(Debug)]
pub enum MyError { // MySQL Error
    NotFound,
    MySqlError(mysql_async::Error),
}

impl From<mysql_async::Error> for MyError{
    fn from(err: mysql_async::Error) -> Self {
        MyError::MySqlError(err)
    }
}

