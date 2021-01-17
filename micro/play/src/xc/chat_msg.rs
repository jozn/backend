use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
// use cdrs::query::*;
use cdrs::frame::Frame;
use cdrs::query::{QueryExecutor, QueryValues};

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::ByName;
use std::collections::HashMap;
use std::result::Result; // override prelude Result

//use cdrs::error::{Error as CWError};
use crate::xc::common::*;
use cdrs::frame::frame_error::CDRSError;
use cdrs::Error as DriverError;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ChatMsg {
    pub chat_id: i64, // chat_id    partition_key  0
    pub msg_id: i64,  // msg_id    clustering  0
    pub pb_data: Vec<u8>, // pb_data    regular  -1

                      //_exists: bool,
                      //_deleted: bool,
}

impl ChatMsg {
    /*pub fn deleted(&self) -> bool {
        self._deleted
    }

    pub fn exists(&self) -> bool {
        self._exists
    }*/

    pub fn save(&mut self, session: impl FCQueryExecutor) -> Result<(), CWError> {
        let mut columns = vec![];
        let mut values: Vec<Value> = vec![];

        if self.chat_id != 0i64 {
            columns.push("chat_id");
            values.push(self.chat_id.clone().into());
        }

        if self.msg_id != 0i64 {
            columns.push("msg_id");
            values.push(self.msg_id.clone().into());
        }

        if !self.pb_data.is_empty() {
            columns.push("pb_data");
            values.push(self.pb_data.clone().into());
        }

        if columns.len() == 0 {
            return Err(CWError::InvalidCQL);
        }

        let cql_columns = columns.join(", ");
        let mut cql_question = "?,".repeat(columns.len());
        cql_question.remove(cql_question.len() - 1);

        let cql_query = format!(
            "INSERT INTO msgs.chat_msg ({}) VALUES ({})",
            cql_columns, cql_question
        );

        println!("{} - {}", &cql_query, &cql_question);

        session.query_with_values(cql_query, values)?;

        Ok(())
    }

    pub fn delete(&mut self, session: impl FCQueryExecutor) -> Result<(), CWError> {
        let mut deleter = ChatMsg_Deleter::new();

        deleter.chat_id_eq(self.chat_id);

        deleter.and_msg_id_eq(self.msg_id);

        let res = deleter.delete(session)?;

        Ok(())
    }
}

fn _get_where(wheres: Vec<WhereClause>) -> (String, Vec<Value>) {
    let mut values = vec![];
    let mut where_str = vec![];

    for w in wheres {
        where_str.push(w.condition);
        values.push(w.args)
    }
    let cql_where = where_str.join(" ");

    (cql_where, values)
}

#[derive(Default, Debug)]
pub struct ChatMsg_Selector {
    wheres: Vec<WhereClause>,
    select_cols: Vec<&'static str>,
    order_by: Vec<&'static str>,
    limit: u32,
    allow_filter: bool,
}

impl ChatMsg_Selector {
    pub fn new() -> Self {
        ChatMsg_Selector::default()
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
    pub fn select_chat_id(&mut self) -> &mut Self {
        self.select_cols.push("chat_id");
        self
    }

    pub fn select_msg_id(&mut self) -> &mut Self {
        self.select_cols.push("msg_id");
        self
    }

    pub fn select_pb_data(&mut self) -> &mut Self {
        self.select_cols.push("pb_data");
        self
    }

    pub fn _to_cql(&self) -> (String, Vec<Value>) {
        let cql_select = if self.select_cols.is_empty() {
            "*".to_string()
        } else {
            self.select_cols.join(", ")
        };

        let mut cql_query = format!("SELECT {} FROM msgs.chat_msg", cql_select);

        let (cql_where, where_values) = _get_where(self.wheres.clone());

        if where_values.len() > 0 {
            cql_query.push_str(&format!(" WHERE {}", &cql_where));
        }

        if self.order_by.len() > 0 {
            let cql_orders = self.order_by.join(", ");
            cql_query.push_str(&format!(" ORDER BY {}", &cql_orders));
        };

        if self.limit != 0 {
            cql_query.push_str(&format!(" LIMIT {} ", self.limit));
        };

        if self.allow_filter {
            cql_query.push_str(" ALLOW FILTERING");
        };

        (cql_query, where_values)
    }

    pub fn _get_rows_with_size(
        &mut self,
        session: impl FCQueryExecutor,
        size: i64,
    ) -> Result<Vec<ChatMsg>, CWError> {
        let (cql_query, query_values) = self._to_cql();

        println!("{} - {:?}", &cql_query, &query_values);

        let query_result = session
            .query_with_values(cql_query, query_values)?
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
            }
            None => return Err(CWError::NotFound),
        };

        let mut rows = vec![];

        for db_row in db_raws {
            let mut row = ChatMsg::default();

            row.chat_id = db_row.by_name("chat_id")?.unwrap_or_default();

            row.msg_id = db_row.by_name("msg_id")?.unwrap_or_default();

            row.pb_data = db_row
                .by_name::<Blob>("pb_data")?
                .unwrap_or(Blob::new(vec![]))
                .into_vec();

            rows.push(row);
        }

        Ok(rows)
    }

    pub fn get_rows(&mut self, session: impl FCQueryExecutor) -> Result<Vec<ChatMsg>, CWError> {
        self._get_rows_with_size(session, -1)
    }

    pub fn get_row(&mut self, session: impl FCQueryExecutor) -> Result<ChatMsg, CWError> {
        let rows = self._get_rows_with_size(session, 1)?;

        let opt = rows.get(0);
        match opt {
            Some(row) => Ok(row.to_owned()),
            None => Err(CWError::NotFound),
        }
    }

    pub fn order_by_msg_id_asc(&mut self) -> &mut Self {
        self.order_by.push("msg_id ASC");
        self
    }

    pub fn order_by_msg_id_desc(&mut self) -> &mut Self {
        self.order_by.push("msg_id DESC");
        self
    }

    pub fn chat_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" chat_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND chat_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR chat_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" msg_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND msg_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR msg_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }
}

#[derive(Default, Debug)]
pub struct ChatMsg_Deleter {
    wheres: Vec<WhereClause>,
    delete_cols: Vec<&'static str>,
}

#[derive(Default, Debug)]
pub struct ChatMsg_Updater {
    wheres: Vec<WhereClause>,
    updates: HashMap<&'static str, Value>,
}

impl ChatMsg_Updater {
    pub fn new() -> Self {
        ChatMsg_Updater::default()
    }

    pub fn update(&mut self, session: impl FCQueryExecutor) -> cdrs::error::Result<Frame> {
        if self.updates.is_empty() {
            return Err(cdrs::error::Error::General("empty".to_string()));
        }

        // Update columns building
        let mut all_vals = vec![];
        let mut col_updates = vec![];

        for (col, val) in self.updates.clone() {
            all_vals.push(val);
            col_updates.push(col);
        }
        let cql_update = col_updates.join(",");

        // Where columns building
        let mut where_str = vec![];

        for w in self.wheres.clone() {
            where_str.push(w.condition);
            all_vals.push(w.args)
        }
        let cql_where = where_str.join(" ");

        // Build final query
        let mut cql_query = if self.wheres.is_empty() {
            format!("UPDATE msgs.chat_msg SET {}", cql_update)
        } else {
            format!(
                "UPDATE msgs.chat_msg SET {} WHERE {}",
                cql_update, cql_where
            )
        };

        let query_values = QueryValues::SimpleValues(all_vals);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)
    }

    pub fn update_chat_id(&mut self, val: i64) -> &mut Self {
        self.updates.insert("chat_id = ?", val.into());
        self
    }

    pub fn update_msg_id(&mut self, val: i64) -> &mut Self {
        self.updates.insert("msg_id = ?", val.into());
        self
    }

    pub fn update_pb_data(&mut self, val: &Vec<u8>) -> &mut Self {
        self.updates
            .insert("pb_data = ?", Blob::new(val.clone()).into());
        self
    }

    pub fn chat_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" chat_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND chat_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR chat_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" msg_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND msg_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR msg_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }
}

impl ChatMsg_Deleter {
    pub fn new() -> Self {
        ChatMsg_Deleter::default()
    }

    //each column delete
    pub fn delete_chat_id(&mut self) -> &mut Self {
        self.delete_cols.push("chat_id");
        self
    }

    pub fn delete_msg_id(&mut self) -> &mut Self {
        self.delete_cols.push("msg_id");
        self
    }

    pub fn delete_pb_data(&mut self) -> &mut Self {
        self.delete_cols.push("pb_data");
        self
    }

    pub fn delete(&mut self, session: impl FCQueryExecutor) -> Result<(), CWError> {
        let del_col = self.delete_cols.join(", ");

        let mut where_str = vec![];
        let mut where_arr = vec![];

        for w in self.wheres.clone() {
            where_str.push(w.condition);
            where_arr.push(w.args)
        }

        let where_str = where_str.join(" ");

        let cql_query = format!("DELETE {} FROM msgs.chat_msg WHERE {}", del_col, where_str);
        //let cql_query = "DELETE " + del_col + " FROM msgs.chat_msg WHERE " + where_str ;

        let query_values = QueryValues::SimpleValues(where_arr);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)?;

        Ok(())
    }

    pub fn chat_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " chat_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND chat_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR chat_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " msg_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND msg_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR msg_id >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn chat_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" chat_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_chat_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND chat_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_chat_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR chat_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn msg_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" msg_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_msg_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND msg_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_msg_id_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR msg_id IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }
}
