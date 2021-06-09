// DO NOT MODIFY AUTO-GENERATED BY PB-WALKER
use mysql_async::prelude::*;
use mysql_async::{FromRowError, OptsBuilder, Params, Row, Pool};
use mysql_common::row::ColumnIndex;

use mysql_common::value::Value;
use crate::mysql_common::*;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Profile  { // profile
    pub profile_id: u32,
    pub pb_data: Vec<u8>,
    pub debug_data: String,
}

impl FromRow for Profile {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        Ok(Profile  {
            profile_id: row.get(0).unwrap_or_default(),
            pb_data: row.get(1).unwrap_or_default(),
            debug_data: row.get(2).unwrap_or_default(),
        })
    }
}

impl Profile {
    pub async fn insert(&self, spool: &SPool) -> Result<Profile,MyError> {
        let mut conn = spool.pool.get_conn().await?;

        let query = format!(r"INSERT INTO {:}.profile (profile_id, pb_data, debug_data) VALUES (?, ?, ?)",&spool.database);
        let p = Params::Positional(vec![self.profile_id.clone().into(), self.pb_data.clone().into(), self.debug_data.clone().into()]);

        conn.exec_iter(
            query, p
        ).await?;

        let cp = self.clone();
        Ok(cp)
    }

    // [[ for template: this code is copy of insert with 'insert' changed to 'replace' ]]
    pub async fn replace(&self, spool: &SPool) -> Result<Profile,MyError> {
        let mut conn = spool.pool.get_conn().await?;

        let query = format!(r"REPLACE INTO {:}.profile (profile_id, pb_data, debug_data) VALUES (?, ?, ?)",&spool.database);
        let p = Params::Positional(vec![self.profile_id.clone().into(), self.pb_data.clone().into(), self.debug_data.clone().into()]);

        conn.exec_iter(
            query, p
        ).await?;

        let cp = self.clone();
        Ok(cp)
    }

    pub async fn update(&self, spool: &SPool) -> Result<(),MyError> {
        let mut conn = spool.pool.get_conn().await?;
        let query = format!(r"UPDATE `{:}`.profile` SET pb_data = ?, debug_data = ? WHERE profile_id = ? ", &spool.database);
        let p = Params::Positional(vec![self.pb_data.clone().into(), self.debug_data.clone().into(),  self.profile_id.clone().into() ]);

        let qr = conn.exec_iter(
            query, p
        ).await?;

        Ok(())
    }

    pub async fn delete(&self, spool: &SPool) -> Result<(),MyError> {
        let mut conn = spool.pool.get_conn().await?;

        let query = format!(r"DELETE FROM `{:}`.profile` WHERE profile_id = ? ", &spool.database);
        let p = Params::Positional(vec![self.profile_id.clone().into()]);

        conn.exec_drop(
            query, p
        ).await?;

        Ok(())
    }
}


#[derive(Default, Debug)]
pub struct ProfileSelector {
    q: TQuery
}

impl ProfileSelector {
    pub fn new() -> Self {
        ProfileSelector{
            q: TQuery{
                table: "profile",
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
    pub fn select_profile_id(&mut self) -> &mut Self {
        self.q.select_cols.push("profile_id");
        self
    }
    
    pub fn select_pb_data(&mut self) -> &mut Self {
        self.q.select_cols.push("pb_data");
        self
    }
    
    pub fn select_debug_data(&mut self) -> &mut Self {
        self.q.select_cols.push("debug_data");
        self
    }
    

    pub async fn _get_rows_with_size(&mut self, session: &SPool, size: i64) -> Result<Vec<Profile>, MyError>   {
        let mut conn = session.pool.get_conn().await?;
        let(cql_query, query_values) = self.q._to_sql_selector(&session.database);

        println!("{} - {:?}", &cql_query, &query_values);

        let p = Params::Positional(query_values);

        let query_result = conn
            .exec_map(
                cql_query,p,
                |obj: Profile| obj
            ).await?;

        Ok(query_result)
    }

    pub async fn get_rows(&mut self, session: &SPool) -> Result<Vec<Profile>, MyError>{
        self._get_rows_with_size(session,-1).await
    }

    pub async fn get_row(&mut self, session: &SPool) -> Result<Profile, MyError>{
        let rows = self._get_rows_with_size(session,1).await?;

        let opt = rows.get(0);
        match opt {
            Some(row) => Ok(row.to_owned()),
            None => Err(MyError::NotFound)
        }
    }

    // Modifiers
    
    pub fn order_by_profile_id_asc(&mut self) -> &mut Self {
		self.q.order_by.push("profile_id ASC");
        self
    }

	pub fn order_by_profile_id_desc(&mut self) -> &mut Self {
		self.q.order_by.push("profile_id DESC");
        self
    }

    pub fn order_by_debug_data_asc(&mut self) -> &mut Self {
		self.q.order_by.push("debug_data ASC");
        self
    }

	pub fn order_by_debug_data_desc(&mut self) -> &mut Self {
		self.q.order_by.push("debug_data DESC");
        self
    }

    
    pub fn profile_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    
    pub fn profile_id_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" profile_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_profile_id_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND profile_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_profile_id_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR profile_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn debug_data_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" debug_data IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_debug_data_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND debug_data IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_debug_data_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR debug_data IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

}

// Updater

#[derive(Default, Debug)]
pub struct ProfileUpdater {
    q: TQuery
}

impl ProfileUpdater {
    pub fn new() -> Self {
        ProfileUpdater{
            q: TQuery{
                table: "profile",
                ..Default::default()
            }
        }
    }

    pub fn limit(&mut self, size: u32) -> &mut Self {
        self.q.limit = size;
        self
    }

    //each column delete
    pub fn set_profile_id(&mut self, val :u32) -> &mut Self {
        self.q.updates.insert("profile_id",val.into());
        self
    }
    
    pub fn set_pb_data(&mut self, val :&Vec<u8>) -> &mut Self {
        self.q.updates.insert("pb_data",val.into());
        self
    }
    
    pub fn set_debug_data(&mut self, val :&str) -> &mut Self {
        self.q.updates.insert("debug_data",val.into());
        self
    }
    

    pub async fn update(&mut self, session: &SPool) -> Result<(),MyError> {
        update_rows(&self.q, session).await
    }

    
    pub fn order_by_profile_id_asc(&mut self) -> &mut Self {
		self.q.order_by.push("profile_id ASC");
        self
    }

	pub fn order_by_profile_id_desc(&mut self) -> &mut Self {
		self.q.order_by.push("profile_id DESC");
        self
    }

    pub fn order_by_debug_data_asc(&mut self) -> &mut Self {
		self.q.order_by.push("debug_data ASC");
        self
    }

	pub fn order_by_debug_data_desc(&mut self) -> &mut Self {
		self.q.order_by.push("debug_data DESC");
        self
    }

    
    pub fn profile_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    
    pub fn profile_id_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" profile_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_profile_id_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND profile_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_profile_id_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR profile_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn debug_data_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" debug_data IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_debug_data_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND debug_data IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_debug_data_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR debug_data IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

}

// Deleter

#[derive(Default, Debug)]
pub struct ProfileDeleter {
    q: TQuery
}

impl ProfileDeleter {
    pub fn new() -> Self {
        ProfileDeleter{
            q: TQuery{
                table: "profile",
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

    
    pub fn order_by_profile_id_asc(&mut self) -> &mut Self {
		self.q.order_by.push("profile_id ASC");
        self
    }

	pub fn order_by_profile_id_desc(&mut self) -> &mut Self {
		self.q.order_by.push("profile_id DESC");
        self
    }

    pub fn order_by_debug_data_asc(&mut self) -> &mut Self {
		self.q.order_by.push("debug_data ASC");
        self
    }

	pub fn order_by_debug_data_desc(&mut self) -> &mut Self {
		self.q.order_by.push("debug_data DESC");
        self
    }

    
    pub fn profile_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn profile_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: " profile_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_profile_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND profile_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_eq (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_lt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_le (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_gt (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_profile_id_ge (&mut self, val: u32 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR profile_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn debug_data_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: " debug_data >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_debug_data_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "AND debug_data >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_eq (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_lt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_le (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_gt (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_debug_data_ge (&mut self, val: &str ) -> &mut Self {
        let w = WhereClause{
            condition: "OR debug_data >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    
    pub fn profile_id_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" profile_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_profile_id_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND profile_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_profile_id_in (&mut self, val: Vec<u32> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR profile_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn debug_data_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" debug_data IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_debug_data_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND debug_data IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_debug_data_in (&mut self, val: Vec<&str> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR debug_data IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

}

pub async fn profile_mass_insert(arr :&Vec<Profile>, spool: &SPool) -> Result<(),MyError> {
    let mut conn = spool.pool.get_conn().await?;
    if arr.len() == 0 {
        return Err(MyError::EmptySql)
    }
     let mut insert_fields = "(profile_id, pb_data, debug_data)";

     let mut vals_str = "(?, ?, ?), ".repeat(arr.len());
    let vals_str = &vals_str[0..(vals_str.len()-2)];

    let query = format!(r"INSERT INTO {:}.tweet {:} VALUES {:}", &spool.database, insert_fields, vals_str);

    let mut arr_vals = vec![];
    for ar in arr {
        arr_vals.push(ar.profile_id.clone().into());
        arr_vals.push(ar.pb_data.clone().into());
        arr_vals.push(ar.debug_data.clone().into());}

    let p = Params::Positional(arr_vals);

    println!("{} - {:?}", &query, &p);

    let qr = conn.exec_iter(
        query, p
    ).await?;

    Ok(())
}

// Index
pub async fn get_profile(profile_id: u32, spool: &SPool) -> Result<Profile,MyError> {
	let m = ProfileSelector::new()
		.profile_id_eq(profile_id)
		.get_row(spool).await?;
	Ok(m)
}
