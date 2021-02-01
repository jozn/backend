use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
// use cdrs::query::*;
use cdrs::frame::Frame;
use cdrs::query::{QueryExecutor, QueryValues};
use cdrs::types::value::ValueType;

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
pub struct ChannelSubscriberSort {
    pub profile_cid: i64,    // profile_cid    clustering  1
    pub channel_cid: i64,    // channel_cid    partition_key  0
    pub subscriber_gid: i64, // subscriber_gid    clustering  0
}

impl ChannelSubscriberSort {
    pub fn save(&self, session: impl FCQueryExecutor) -> Result<(), CWError> {
        let mut columns = vec![];
        let mut values: Vec<Value> = vec![];

        // partition key and clustering key always must be present
        columns.push("profile_cid");
        values.push(self.profile_cid.clone().into());

        // partition key and clustering key always must be present
        columns.push("channel_cid");
        values.push(self.channel_cid.clone().into());

        // partition key and clustering key always must be present
        columns.push("subscriber_gid");
        values.push(self.subscriber_gid.clone().into());

        if columns.len() == 0 {
            return Err(CWError::InvalidCQL);
        }

        let cql_columns = columns.join(", ");
        let mut cql_question = "?,".repeat(columns.len());
        cql_question.remove(cql_question.len() - 1);

        let cql_query = format!(
            "INSERT INTO flip.channel_subscriber_sort ({}) VALUES ({})",
            cql_columns, cql_question
        );

        println!("{} - {}", &cql_query, &cql_question);

        session.query_with_values(cql_query, values)?;

        Ok(())
    }

    pub fn delete(&self, session: impl FCQueryExecutor) -> Result<(), CWError> {
        let mut deleter = ChannelSubscriberSort_Deleter::new();

        deleter.channel_cid_eq(self.channel_cid);

        deleter.and_profile_cid_eq(self.profile_cid);
        deleter.and_subscriber_gid_eq(self.subscriber_gid);

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
pub struct ChannelSubscriberSort_Selector {
    wheres: Vec<WhereClause>,
    select_cols: Vec<&'static str>,
    order_by: Vec<&'static str>,
    limit: u32,
    allow_filter: bool,
}

impl ChannelSubscriberSort_Selector {
    pub fn new() -> Self {
        ChannelSubscriberSort_Selector::default()
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
    pub fn select_profile_cid(&mut self) -> &mut Self {
        self.select_cols.push("profile_cid");
        self
    }

    pub fn select_channel_cid(&mut self) -> &mut Self {
        self.select_cols.push("channel_cid");
        self
    }

    pub fn select_subscriber_gid(&mut self) -> &mut Self {
        self.select_cols.push("subscriber_gid");
        self
    }

    pub fn _to_cql(&self) -> (String, Vec<Value>) {
        let cql_select = if self.select_cols.is_empty() {
            "*".to_string()
        } else {
            self.select_cols.join(", ")
        };

        let mut cql_query = format!("SELECT {} FROM flip.channel_subscriber_sort", cql_select);

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
    ) -> Result<Vec<ChannelSubscriberSort>, CWError> {
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
            let mut row = ChannelSubscriberSort::default();

            row.profile_cid = db_row.by_name("profile_cid")?.unwrap_or_default();

            row.channel_cid = db_row.by_name("channel_cid")?.unwrap_or_default();

            row.subscriber_gid = db_row.by_name("subscriber_gid")?.unwrap_or_default();

            rows.push(row);
        }

        Ok(rows)
    }

    pub fn get_rows(
        &mut self,
        session: impl FCQueryExecutor,
    ) -> Result<Vec<ChannelSubscriberSort>, CWError> {
        self._get_rows_with_size(session, -1)
    }

    pub fn get_row(
        &mut self,
        session: impl FCQueryExecutor,
    ) -> Result<ChannelSubscriberSort, CWError> {
        let rows = self._get_rows_with_size(session, 1)?;

        let opt = rows.get(0);
        match opt {
            Some(row) => Ok(row.to_owned()),
            None => Err(CWError::NotFound),
        }
    }

    pub fn order_by_profile_cid_asc(&mut self) -> &mut Self {
        self.order_by.push("profile_cid ASC");
        self
    }

    pub fn order_by_profile_cid_desc(&mut self) -> &mut Self {
        self.order_by.push("profile_cid DESC");
        self
    }

    pub fn order_by_subscriber_gid_asc(&mut self) -> &mut Self {
        self.order_by.push("subscriber_gid ASC");
        self
    }

    pub fn order_by_subscriber_gid_desc(&mut self) -> &mut Self {
        self.order_by.push("subscriber_gid DESC");
        self
    }

    pub fn profile_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" subscriber_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND subscriber_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR subscriber_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }
}

#[derive(Default, Debug)]
pub struct ChannelSubscriberSort_Deleter {
    wheres: Vec<WhereClause>,
    delete_cols: Vec<&'static str>,
}

#[derive(Default, Debug)]
pub struct ChannelSubscriberSort_Updater {
    wheres: Vec<WhereClause>,
    updates: HashMap<&'static str, Value>,
}

impl ChannelSubscriberSort_Updater {
    pub fn new() -> Self {
        ChannelSubscriberSort_Updater::default()
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
            format!("UPDATE flip.channel_subscriber_sort SET {}", cql_update)
        } else {
            format!(
                "UPDATE flip.channel_subscriber_sort SET {} WHERE {}",
                cql_update, cql_where
            )
        };

        let query_values = QueryValues::SimpleValues(all_vals);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)
    }

    pub fn update_profile_cid(&mut self, val: i64) -> &mut Self {
        self.updates.insert("profile_cid = ?", val.into());
        self
    }

    pub fn update_channel_cid(&mut self, val: i64) -> &mut Self {
        self.updates.insert("channel_cid = ?", val.into());
        self
    }

    pub fn update_subscriber_gid(&mut self, val: i64) -> &mut Self {
        self.updates.insert("subscriber_gid = ?", val.into());
        self
    }

    pub fn profile_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" subscriber_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND subscriber_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR subscriber_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }
}

impl ChannelSubscriberSort_Deleter {
    pub fn new() -> Self {
        ChannelSubscriberSort_Deleter::default()
    }

    //each column delete
    pub fn delete_profile_cid(&mut self) -> &mut Self {
        self.delete_cols.push("profile_cid");
        self
    }

    pub fn delete_channel_cid(&mut self) -> &mut Self {
        self.delete_cols.push("channel_cid");
        self
    }

    pub fn delete_subscriber_gid(&mut self) -> &mut Self {
        self.delete_cols.push("subscriber_gid");
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

        let cql_query = format!(
            "DELETE {} FROM flip.channel_subscriber_sort WHERE {}",
            del_col, where_str
        );
        //let cql_query = "DELETE " + del_col + " FROM flip.channel_subscriber_sort WHERE " + where_str ;

        let query_values = QueryValues::SimpleValues(where_arr);
        println!("{} - {:?}", &cql_query, &query_values);

        session.query_with_values(cql_query, query_values)?;

        Ok(())
    }

    pub fn profile_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR profile_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_lt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_le_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_gt_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_ge_filtering(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR channel_cid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: " subscriber_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "AND subscriber_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_eq(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid = ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_lt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid < ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_le(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid <= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_gt(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid > ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_ge(&mut self, val: i64) -> &mut Self {
        let w = WhereClause {
            condition: "OR subscriber_gid >= ?".to_string(),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn profile_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_profile_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_profile_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR profile_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn channel_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_channel_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_channel_cid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR channel_cid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn subscriber_gid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!(" subscriber_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_subscriber_gid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("AND subscriber_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_subscriber_gid_in(&mut self, val: Vec<i64>) -> &mut Self {
        let len = val.len();
        if len == 0 {
            return self;
        }

        let mut marks = "?,".repeat(len);
        marks.remove(marks.len() - 1);
        let w = WhereClause {
            condition: format!("OR subscriber_gid IN ({})", marks),
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }
}

pub fn get_channel_subscriber_sort_by_channel_cid_and_profile_cid_and_subscriber_gid(
    session: impl FCQueryExecutor,
    p1: i64,
    p2: i64,
    p3: i64,
) -> Result<ChannelSubscriberSort, CWError> {
    let m = ChannelSubscriberSort_Selector::new()
        .channel_cid_eq(p1)
        .and_profile_cid_eq(p2)
        .and_subscriber_gid_eq(p3)
        .get_row(session)?;
    Ok(m)
}
