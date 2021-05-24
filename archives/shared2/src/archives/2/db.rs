
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
#[macro_use]
extern crate maplit;

use std::collections::HashMap;

use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<StaticPasswordAuthenticator>>>;

mod xc;
mod pb;


fn main_run() {
    let user = "user";
    let password = "password";
    let auth = StaticPasswordAuthenticator::new(&user, &password);
    let node = NodeTcpConfigBuilder::new("localhost:9042", auth).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression: CurrentSession =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    //delete_struct(&no_compression);

    let mut d = xc::Tweet_Deleter::new();
    d.user_id_eq(25).and_tweet_id_eq("asdfghjklqwert");;
    d.delete(&no_compression);

    let mut d = xc::Tweet_Deleter::new();
    d.delete_body().delete_create_time().user_id_eq(25).and_tweet_id_in(vec!["aaa","bbb","ccc"]).delete(&no_compression);


    let mut d = xc::Tweet_Selector::new();
    let res =d.user_id_eq(2).limit(1).get_row(&no_compression);

    println!("{:?}", res);

    for i in 1..50 {
        let mut m = xc::Tweet::default();
        m.tweet_id = format!("id_{}",i);
        m.user_id = i;
        m.body = format!("My sample tweet {}",i);

        let res = m.save(&no_compression);
        println!("{:?}", res);
    }

    for i in 10..50 {
        let mut m = xc::Tweet::default();
        m.tweet_id = format!("id_{}",i);
        m.user_id = i;
        m.body = format!("My sample tweet {}",i);

        let res = m.delete(&no_compression);
        println!("{:?}", res);
    }

}

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct Tweet {
    pub tweet_id: String,   // tweet_id    clustering  0
    pub user_id: i64,   // user_id    partition_key  0
    pub body: String,   // body    regular  -1
    pub create_time: i32,   // create_time    regular  -1

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

    pub fn limit(&mut self, size: u32) -> &Self {
        self.limit = size;
        self
    }

    pub fn allow_filtering(&mut self, allow: bool) -> &Self {
        self.allow_filter = allow;
        self
    }

    // each col
    pub fn select_body(&mut self, allow: bool) -> &Self {
        self.select_cols.push("body");
        self
    }

    //each column orders //just ints
    pub fn orderby_tweet_id_desc(&mut self, allow: bool) -> &Self {
        self.order_by.push(" tweet_id desc");
        self
    }

    pub fn orderby_tweet_id_asc(&mut self, allow: bool) -> &Self {
        self.order_by.push(" tweet_id asc");
        self
    }

    pub fn select(&mut self, allow: bool) {

    }

}

#[derive(Debug)]
struct WhereClause {
    condition: &'static str,
    args: Value,
}

#[derive(Default, Debug)]
pub struct Tweet_Deleter {
    wheres: Vec<WhereClause>,
    delete_cols: Vec<&'static str>,
}

impl Tweet_Deleter {
    pub fn new() -> Self {
        Tweet_Deleter::default()
    }

    // each col
    pub fn delete_body(&mut self, allow: bool) -> &Self {
        self.delete_cols.push("body");
        self
    }

    pub fn tweet_id_eq(&mut self, val: &str) ->&Self {
        let w = WhereClause{
            condition: "tweet_id =?",
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn and_tweet_id_eq(&mut self, val: &str) ->&Self {
        let w = WhereClause{
            condition: "AND tweet_id =?",
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_eq(&mut self, val: &str) ->&Self {
        let w = WhereClause{
            condition: "OR tweet_id =?",
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

    pub fn or_tweet_id_ge(&mut self, val: &str) ->&Self {
        let w = WhereClause{
            condition: "OR tweet_id >= ?",
            args: val.into(),
        };
        self.wheres.push(w);
        self
    }

}

#[derive(Default, Debug)]
struct TweetSelection {

}

fn insert_struct2(session: &CurrentSession) {
    let insert_struct_cql = "INSERT INTO test_ks.my_test_table \
                             (?, ?, ?, ?) VALUES (?, ?, ?, ?)";

    let v :Vec<u8> = Vec::new();
    use cdrs::types::value::Value;
    use cdrs::query::QueryValues;
    let mut values: Vec<Value> = Vec::new();
    values.push(5.into());
    values.push("sdf".into());
    values.push(v.into());

    let query_values = QueryValues::SimpleValues(values);

    session.query_with_values(insert_struct_cql, query_values );
}


/*pub fn delete(&mut self, session: &CurrentSession) {

    let del_col = self.delete_cols.join(", ");

    let  mut where_str = vec![];
    let mut where_arr = vec![];

    for i in self.wheres {
        where_str.push(i.condition);
        where_arr.push(i.args)
    }

    let where_str = where_str.join("");

    let cql_query = "DELETE " + del_col + " FROM {{.TableSchemeOut}} WHERE " + where_str ;

    let query_values = QueryValues::SimpleValues(where_arr);

    session.query_with_values(cql_query, query_values);

}*/


































////////////// Archives /////////////////////////
fn main_old() {
    let user = "user";
    let password = "password";
    let auth = StaticPasswordAuthenticator::new(&user, &password);
    let node = NodeTcpConfigBuilder::new("localhost:9042", auth).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression: CurrentSession =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    create_keyspace(&no_compression);
    create_udt(&no_compression);
    create_table(&no_compression);
    insert_struct(&no_compression);
    select_struct(&no_compression);
    update_struct(&no_compression);
    //delete_struct(&no_compression);
}

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct RowStruct {
    key: i32,
    user: User,
    map: HashMap<String, User>,
    list: Vec<User>,
}

#[derive(Clone, Debug, PartialEq)]
struct Row {
    key: i32,
    vec: Vec<u8>,
    // time: bool,
}

fn p1() {
    let r = Row {
        key: 0,
        vec: vec![],
        // time: "".to_string(),
    };
}


/*
:= &xc.Tweet {
	tweet_id: "".to_string(),
	user_id: 0i62,
	body: "".to_string(),
	create_time: 0i32,
*/
#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct User2 {
    pub user_id: i32,   // user_id    partition_key  0
    pub created_time: i64,   // created_time    regular  -1
    pub full_name: String,   // full_name    regular  -1
    pub user_name: String,   // user_name    regular  -1

    _exists: bool,
    _deleted: bool,
}

impl RowStruct {
    fn into_query_values(self) -> QueryValues {
        query_values!("key" => self.key, "user" => self.user, "map" => self.map, "list" => self.list)
    }
}

#[derive(Debug, Clone, PartialEq, IntoCDRSValue, TryFromUDT)]
struct User {
    username: String,
}

fn create_keyspace(session: &CurrentSession) {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS test_ks WITH REPLICATION = { \
                                   'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    session.query(create_ks).expect("Keyspace creation error");
}

fn create_udt(session: &CurrentSession) {
    let create_type_cql = "CREATE TYPE IF NOT EXISTS test_ks.user (username text)";
    session
        .query(create_type_cql)
        .expect("Keyspace creation error");
}

fn create_table(session: &CurrentSession) {
    let create_table_cql =
        "CREATE TABLE IF NOT EXISTS test_ks.my_test_table (key int PRIMARY KEY, \
     user frozen<test_ks.user>, map map<text, frozen<test_ks.user>>, list list<frozen<test_ks.user>>);";
    session
        .query(create_table_cql)
        .expect("Table creation error");
}

fn insert_struct(session: &CurrentSession) {
    let row = RowStruct {
        key: 3i32,
        user: User {
            username: "John".to_string(),
        },
        map: hashmap! { "John".to_string() => User { username: "John".to_string() } },
        list: vec![User {
            username: "John".to_string(),
        }],
    };

    let insert_struct_cql = "INSERT INTO test_ks.my_test_table \
                             (key, user, map, list) VALUES (?, ?, ?, ?)";
    session
        .query_with_values(insert_struct_cql, row.into_query_values())
        .expect("insert");
}

fn select_struct(session: &CurrentSession) {
    let select_struct_cql = "SELECT * FROM test_ks.my_test_table";
    let rows = session
        .query(select_struct_cql)
        .expect("query")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    for row in rows {
        // let my_row: RowStruct = RowStruct::try_from_row(row);//.expect("into RowStruct");
        let my_row = RowStruct::try_from_row(row); //.expect("into RowStruct");
        println!("struct got: {:?}", my_row);
    }
}

fn update_struct(session: &CurrentSession) {
    let update_struct_cql = "UPDATE test_ks.my_test_table SET user = ? WHERE key = ?";
    let upd_user = User {
        username: "Marry".to_string(),
    };
    let user_key = 1i32;
    session
        .query_with_values(update_struct_cql, query_values!(upd_user, user_key))
        .expect("update");
}

fn delete_struct(session: &CurrentSession) {
    let delete_struct_cql = "DELETE FROM test_ks.my_test_table WHERE key = ?";
    let user_key = 1i32;
    session
        .query_with_values(delete_struct_cql, query_values!(user_key))
        .expect("delete");
}
