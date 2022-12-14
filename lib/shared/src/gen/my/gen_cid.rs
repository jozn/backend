// DO NOT MODIFY AUTO-GENERATED BY PB-WALKER
use mysql_async::prelude::*;
use mysql_async::{FromRowError, OptsBuilder, Params, Row, Pool};
use mysql_common::row::ColumnIndex;

use mysql_common::value::Value;
use crate::mysql_common::*;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct GenCid  { // gen_cid
    pub cid: u32,
    pub intent: String,
}

impl FromRow for GenCid {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        Ok(GenCid  {
            cid: row.get(0).unwrap_or_default(),
            intent: row.get(1).unwrap_or_default(),
        })
    }
}

impl GenCid {
    pub async fn insert(&self, spool: &SPool) -> Result<GenCid,MyError> {
        let mut conn = spool.pool.get_conn().await?;

        let query = format!(r"INSERT INTO {:}.gen_cid (intent) VALUES (?)",&spool.database);
        let p = Params::Positional(vec![self.intent.clone().into()]);

        let qr = conn.exec_iter(
            query, p
        ).await?;

        let mut cp = self.clone();
        cp.cid = qr.last_insert_id().unwrap() as u32;
        Ok(cp)
    }


    // [[ for template: this code is copy of insert with 'insert' changed to 'replace' ]]
    pub async fn replace(&self, spool: &SPool) -> Result<GenCid,MyError> {
        let mut conn = spool.pool.get_conn().await?;

        let query = format!(r"REPLACE INTO {:}.gen_cid (intent) VALUES (?)",&spool.database);
        let p = Params::Positional(vec![self.intent.clone().into()]);

        let qr = conn.exec_iter(
            query, p
        ).await?;

        let mut cp = self.clone();
        cp.cid = qr.last_insert_id().unwrap() as u32;
        Ok(cp)
    }


    pub async fn update(&self, spool: &SPool) -> Result<(),MyError> {
        let mut conn = spool.pool.get_conn().await?;
        let query = format!(r"UPDATE `{:}`.gen_cid` SET intent = ? WHERE cid = ? ", &spool.database);
        let p = Params::Positional(vec![self.intent.clone().into(),  self.cid.clone().into() ]);

        let qr = conn.exec_iter(
            query, p
        ).await?;

        Ok(())
    }

    pub async fn delete(&self, spool: &SPool) -> Result<(),MyError> {
        let mut conn = spool.pool.get_conn().await?;

        let query = format!(r"DELETE FROM `{:}`.gen_cid` WHERE cid = ? ", &spool.database);
        let p = Params::Positional(vec![self.cid.clone().into()]);

        conn.exec_drop(
            query, p
        ).await?;

        Ok(())
    }
}


#[derive(Default, Debug)]
pub struct GenCidSelector {
    q: TQuery
}

impl GenCidSelector {
    pub fn new() -> Self {
        GenCidSelector{
            q: TQuery{
                table: "gen_cid",
                ..Default::default()
            }
        }
    }

    pub fn limit(&mut self, size: u32) -> &mut Self {
        self.q.limit = size;
        self
    }

    pub fn offset(&mut self, size: u32) -> &mut Self {
        self.q.offset = size;
        self
    }

    pub fn select_all(&mut self) -> &mut Self {
        // Default is select *
        self
    }

    //each column select
    pub fn select_cid(&mut self) -> &mut Self {
        self.q.select_cols.push("cid");
        self
    }
    
    pub fn select_intent(&mut self) -> &mut Self {
        self.q.select_cols.push("intent");
        self
    }
    

    pub async fn _get_rows_with_size(&mut self, session: &SPool, size: i64) -> Result<Vec<GenCid>, MyError>   {
        let mut conn = session.pool.get_conn().await?;
        let(cql_query, query_values) = self.q._to_sql_selector(&session.database);

        println!("{} - {:?}", &cql_query, &query_values);

        let p = Params::Positional(query_values);

        let query_result = conn
            .exec_map(
                cql_query,p,
                |obj: GenCid| obj
            ).await?;

        Ok(query_result)
    }

    pub async fn get_rows(&mut self, session: &SPool) -> Result<Vec<GenCid>, MyError>{
        self._get_rows_with_size(session,-1).await
    }

    pub async fn get_row(&mut self, session: &SPool) -> Result<GenCid, MyError>{
        let rows = self._get_rows_with_size(session,1).await?;

        let opt = rows.get(0);
        match opt {
            Some(row) => Ok(row.to_owned()),
            None => Err(MyError::NotFound)
        }
    }

    // Modifiers
    
    pub fn order_by_cid_asc(&mut self) -> &mut Self {
		self.q.order_by.push("cid ASC");
        self
    }

	pub fn order_by_cid_desc(&mut self) -> &mut Self {
		self.q.order_by.push("cid DESC");
        self
    }

    pub fn order_by_intent_asc(&mut self) -> &mut Self {
		self.q.order_by.push("intent ASC");
        self
    }

	pub fn order_by_intent_desc(&mut self) -> &mut Self {
		self.q.order_by.push("intent DESC");
        self
    }

    
    pub fn cid_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    
    pub fn cid_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" cid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_cid_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND cid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_cid_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR cid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn intent_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" intent IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_intent_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND intent IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_intent_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR intent IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

}

// Updater

#[derive(Default, Debug)]
pub struct GenCidUpdater {
    q: TQuery
}

impl GenCidUpdater {
    pub fn new() -> Self {
        GenCidUpdater{
            q: TQuery{
                table: "gen_cid",
                ..Default::default()
            }
        }
    }

    pub fn limit(&mut self, size: u32) -> &mut Self {
        self.q.limit = size;
        self
    }

    //each column delete
    pub fn set_cid(&mut self, val :u32) -> &mut Self {
        self.q.updates.insert("cid",val.into());
        self
    }
    
    pub fn set_intent(&mut self, val :&str) -> &mut Self {
        self.q.updates.insert("intent",val.into());
        self
    }
    

    pub async fn update(&mut self, session: &SPool) -> Result<(),MyError> {
        update_rows(&self.q, session).await
    }

    
    pub fn order_by_cid_asc(&mut self) -> &mut Self {
		self.q.order_by.push("cid ASC");
        self
    }

	pub fn order_by_cid_desc(&mut self) -> &mut Self {
		self.q.order_by.push("cid DESC");
        self
    }

    pub fn order_by_intent_asc(&mut self) -> &mut Self {
		self.q.order_by.push("intent ASC");
        self
    }

	pub fn order_by_intent_desc(&mut self) -> &mut Self {
		self.q.order_by.push("intent DESC");
        self
    }

    
    pub fn cid_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    
    pub fn cid_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" cid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_cid_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND cid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_cid_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR cid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn intent_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" intent IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_intent_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND intent IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_intent_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR intent IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

}

// Deleter

#[derive(Default, Debug)]
pub struct GenCidDeleter {
    q: TQuery
}

impl GenCidDeleter {
    pub fn new() -> Self {
        GenCidDeleter{
            q: TQuery{
                table: "gen_cid",
                ..Default::default()
            }
        }
    }

    pub fn limit(&mut self, size: u32) -> &mut Self {
        self.q.limit = size;
        self
    }

    pub async fn delete(&mut self, session: &SPool) -> Result<(),MyError> {
        delete_rows(&self.q, session).await
    }

    
    pub fn order_by_cid_asc(&mut self) -> &mut Self {
		self.q.order_by.push("cid ASC");
        self
    }

	pub fn order_by_cid_desc(&mut self) -> &mut Self {
		self.q.order_by.push("cid DESC");
        self
    }

    pub fn order_by_intent_asc(&mut self) -> &mut Self {
		self.q.order_by.push("intent ASC");
        self
    }

	pub fn order_by_intent_desc(&mut self) -> &mut Self {
		self.q.order_by.push("intent DESC");
        self
    }

    
    pub fn cid_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn cid_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " cid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_cid_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND cid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_cid_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR cid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn intent_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " intent >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_intent_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND intent >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_intent_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR intent >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    
    pub fn cid_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" cid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_cid_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND cid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_cid_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR cid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn intent_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" intent IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_intent_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND intent IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_intent_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR intent IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

}

pub async fn gen_cid_mass_insert(arr :&Vec<GenCid>, spool: &SPool) -> Result<(),MyError> {
    let mut conn = spool.pool.get_conn().await?;
    if arr.len() == 0 {
        return Err(MyError::EmptySql)
    }
    let mut insert_fields = "(intent)";

    let mut vals_str = "(?), ".repeat(arr.len());
   
    let vals_str = &vals_str[0..(vals_str.len()-2)];

    let query = format!(r"INSERT INTO {:}.tweet {:} VALUES {:}", &spool.database, insert_fields, vals_str);

    let mut arr_vals = vec![];
    for ar in arr {
                
                arr_vals.push(ar.intent.clone().into());
       }

    let p = Params::Positional(arr_vals);

    println!("{} - {:?}", &query, &p);

    let qr = conn.exec_iter(
        query, p
    ).await?;

    Ok(())
}

// Index
pub async fn get_gen_cid(cid: u32, spool: &SPool) -> Result<GenCid,MyError> {
	let m = GenCidSelector::new()
		.cid_eq(cid)
		.get_row(spool).await?;
	Ok(m)
}
