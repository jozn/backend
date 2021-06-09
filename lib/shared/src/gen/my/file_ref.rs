// DO NOT MODIFY AUTO-GENERATED BY PB-WALKER
use mysql_async::prelude::*;
use mysql_async::{FromRowError, OptsBuilder, Params, Row, Pool};
use mysql_common::row::ColumnIndex;

use mysql_common::value::Value;
use crate::mysql_common::*;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct FileRef  { // file_ref
    pub file_gid: u64,
    pub ref_id: u64,
}

impl FromRow for FileRef {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        Ok(FileRef  {
            file_gid: row.get(0).unwrap_or_default(),
            ref_id: row.get(1).unwrap_or_default(),
        })
    }
}

impl FileRef {
    pub async fn insert(&self, spool: &SPool) -> Result<FileRef,MyError> {
        let mut conn = spool.pool.get_conn().await?;

        let query = format!(r"INSERT INTO {:}.file_ref (file_gid, ref_id) VALUES (?, ?)",&spool.database);
        let p = Params::Positional(vec![self.file_gid.clone().into(), self.ref_id.clone().into()]);

        conn.exec_iter(
            query, p
        ).await?;

        let cp = self.clone();
        Ok(cp)
    }

    // [[ for template: this code is copy of insert with 'insert' changed to 'replace' ]]
    pub async fn replace(&self, spool: &SPool) -> Result<FileRef,MyError> {
        let mut conn = spool.pool.get_conn().await?;

        let query = format!(r"REPLACE INTO {:}.file_ref (file_gid, ref_id) VALUES (?, ?)",&spool.database);
        let p = Params::Positional(vec![self.file_gid.clone().into(), self.ref_id.clone().into()]);

        conn.exec_iter(
            query, p
        ).await?;

        let cp = self.clone();
        Ok(cp)
    }

    pub async fn update(&self, spool: &SPool) -> Result<(),MyError> {
        let mut conn = spool.pool.get_conn().await?;
        let query = format!(r"UPDATE `{:}`.file_ref` SET  WHERE file_gid = ? AND ref_id = ? ", &spool.database);
        let p = Params::Positional(vec![ self.file_gid.clone().into(), self.ref_id.clone().into() ]);

        let qr = conn.exec_iter(
            query, p
        ).await?;

        Ok(())
    }

    pub async fn delete(&self, spool: &SPool) -> Result<(),MyError> {
        let mut conn = spool.pool.get_conn().await?;

        let query = format!(r"DELETE FROM `{:}`.file_ref` WHERE file_gid = ? AND ref_id = ? ", &spool.database);
        let p = Params::Positional(vec![self.file_gid.clone().into(), self.ref_id.clone().into()]);

        conn.exec_drop(
            query, p
        ).await?;

        Ok(())
    }
}


#[derive(Default, Debug)]
pub struct FileRefSelector {
    q: TQuery
}

impl FileRefSelector {
    pub fn new() -> Self {
        FileRefSelector{
            q: TQuery{
                table: "file_ref",
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
    pub fn select_file_gid(&mut self) -> &mut Self {
        self.q.select_cols.push("file_gid");
        self
    }
    
    pub fn select_ref_id(&mut self) -> &mut Self {
        self.q.select_cols.push("ref_id");
        self
    }
    

    pub async fn _get_rows_with_size(&mut self, session: &SPool, size: i64) -> Result<Vec<FileRef>, MyError>   {
        let mut conn = session.pool.get_conn().await?;
        let(cql_query, query_values) = self.q._to_sql_selector(&session.database);

        println!("{} - {:?}", &cql_query, &query_values);

        let p = Params::Positional(query_values);

        let query_result = conn
            .exec_map(
                cql_query,p,
                |obj: FileRef| obj
            ).await?;

        Ok(query_result)
    }

    pub async fn get_rows(&mut self, session: &SPool) -> Result<Vec<FileRef>, MyError>{
        self._get_rows_with_size(session,-1).await
    }

    pub async fn get_row(&mut self, session: &SPool) -> Result<FileRef, MyError>{
        let rows = self._get_rows_with_size(session,1).await?;

        let opt = rows.get(0);
        match opt {
            Some(row) => Ok(row.to_owned()),
            None => Err(MyError::NotFound)
        }
    }

    // Modifiers
    
    pub fn order_by_file_gid_asc(&mut self) -> &mut Self {
		self.q.order_by.push("file_gid ASC");
        self
    }

	pub fn order_by_file_gid_desc(&mut self) -> &mut Self {
		self.q.order_by.push("file_gid DESC");
        self
    }

    pub fn order_by_ref_id_asc(&mut self) -> &mut Self {
		self.q.order_by.push("ref_id ASC");
        self
    }

	pub fn order_by_ref_id_desc(&mut self) -> &mut Self {
		self.q.order_by.push("ref_id DESC");
        self
    }

    
    pub fn file_gid_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    
    pub fn file_gid_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" file_gid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_file_gid_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND file_gid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_file_gid_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR file_gid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn ref_id_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" ref_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_ref_id_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND ref_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_ref_id_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR ref_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

}

// Updater

#[derive(Default, Debug)]
pub struct FileRefUpdater {
    q: TQuery
}

impl FileRefUpdater {
    pub fn new() -> Self {
        FileRefUpdater{
            q: TQuery{
                table: "file_ref",
                ..Default::default()
            }
        }
    }

    pub fn limit(&mut self, size: u32) -> &mut Self {
        self.q.limit = size;
        self
    }

    //each column delete
    pub fn set_file_gid(&mut self, val :u64) -> &mut Self {
        self.q.updates.insert("file_gid",val.into());
        self
    }
    
    pub fn set_ref_id(&mut self, val :u64) -> &mut Self {
        self.q.updates.insert("ref_id",val.into());
        self
    }
    

    pub async fn update(&mut self, session: &SPool) -> Result<(),MyError> {
        update_rows(&self.q, session).await
    }

    
    pub fn order_by_file_gid_asc(&mut self) -> &mut Self {
		self.q.order_by.push("file_gid ASC");
        self
    }

	pub fn order_by_file_gid_desc(&mut self) -> &mut Self {
		self.q.order_by.push("file_gid DESC");
        self
    }

    pub fn order_by_ref_id_asc(&mut self) -> &mut Self {
		self.q.order_by.push("ref_id ASC");
        self
    }

	pub fn order_by_ref_id_desc(&mut self) -> &mut Self {
		self.q.order_by.push("ref_id DESC");
        self
    }

    
    pub fn file_gid_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    
    pub fn file_gid_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" file_gid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_file_gid_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND file_gid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_file_gid_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR file_gid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn ref_id_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" ref_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_ref_id_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND ref_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_ref_id_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR ref_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

}

// Deleter

#[derive(Default, Debug)]
pub struct FileRefDeleter {
    q: TQuery
}

impl FileRefDeleter {
    pub fn new() -> Self {
        FileRefDeleter{
            q: TQuery{
                table: "file_ref",
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

    
    pub fn order_by_file_gid_asc(&mut self) -> &mut Self {
		self.q.order_by.push("file_gid ASC");
        self
    }

	pub fn order_by_file_gid_desc(&mut self) -> &mut Self {
		self.q.order_by.push("file_gid DESC");
        self
    }

    pub fn order_by_ref_id_asc(&mut self) -> &mut Self {
		self.q.order_by.push("ref_id ASC");
        self
    }

	pub fn order_by_ref_id_desc(&mut self) -> &mut Self {
		self.q.order_by.push("ref_id DESC");
        self
    }

    
    pub fn file_gid_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn file_gid_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " file_gid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_file_gid_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND file_gid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_file_gid_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR file_gid >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn ref_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: " ref_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn and_ref_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "AND ref_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_eq (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id = ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_lt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id < ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_le (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id <= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_gt (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id > ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    pub fn or_ref_id_ge (&mut self, val: u64 ) -> &mut Self {
        let w = WhereClause{
            condition: "OR ref_id >= ?".to_string(),
            args: val.into(),
        };
        self.q.wheres.push(w);
        self
    }

    
    pub fn file_gid_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" file_gid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_file_gid_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND file_gid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_file_gid_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR file_gid IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn ref_id_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!(" ref_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn and_ref_id_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("AND ref_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

    pub fn or_ref_id_in (&mut self, val: Vec<u64> ) -> &mut Self {
		let len = val.len();
        if len == 0 {
            return self
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len()-1);

		let arr = val.iter().map(|v|v.into()).collect();

        let w = WhereInClause{
			condition: format!("OR ref_id IN ({})", marks),
            args: arr,
        };
        self.q.wheres_ins.push(w);
        self
    }

}

pub async fn file_ref_mass_insert(arr :&Vec<FileRef>, spool: &SPool) -> Result<(),MyError> {
    let mut conn = spool.pool.get_conn().await?;
    if arr.len() == 0 {
        return Err(MyError::EmptySql)
    }
     let mut insert_fields = "(file_gid, ref_id)";

     let mut vals_str = "(?, ?), ".repeat(arr.len());
    let vals_str = &vals_str[0..(vals_str.len()-2)];

    let query = format!(r"INSERT INTO {:}.tweet {:} VALUES {:}", &spool.database, insert_fields, vals_str);

    let mut arr_vals = vec![];
    for ar in arr {
        arr_vals.push(ar.file_gid.clone().into());
        arr_vals.push(ar.ref_id.clone().into());}

    let p = Params::Positional(arr_vals);

    println!("{} - {:?}", &query, &p);

    let qr = conn.exec_iter(
        query, p
    ).await?;

    Ok(())
}

// Index
pub async fn get_file_ref(file_gid: u64,ref_id: u64, spool: &SPool) -> Result<FileRef,MyError> {
	let m = FileRefSelector::new()
		.file_gid_eq(file_gid)
		.and_ref_id_eq(ref_id)
		.get_row(spool).await?;
	Ok(m)
}
