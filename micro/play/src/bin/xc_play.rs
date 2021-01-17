extern crate play;

use std::convert::TryInto;
use std::io::Write;

use byteorder::{ByteOrder, BE};
use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::error::Error as CWError;
use cdrs::frame::Frame;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use grammers_tl_types::Serializable;
use prost::Message;

use play::xc;
use play::xc::FCQueryExecutor;
use shared::aof;

extern crate cdrs;

struct MyCassandraSession {
    session: Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>,
}

impl FCQueryExecutor for &MyCassandraSession {
    fn query_with_values<Q: ToString, V: Into<QueryValues>>(
        &self,
        query: Q,
        values: V,
    ) -> cdrs::error::Result<Frame>
    where
        Self: Sized,
    {
        self.session.query_with_values(query, values)
    }
}
fn main() {
    let node = NodeTcpConfigBuilder::new("185.239.107.163:9042", NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS test_ks WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    no_compression
        .query(create_ks)
        .expect("Keyspace create error");

    let mut m = xc::ChannelMsg {
        channel_id: 345,
        msg_id: 2423,
        pb_data: b"sdfsd".to_vec(),
    };

    // no_compression

    let my_session = MyCassandraSession {
        session: no_compression,
    };

    // let m = m.save(my_session);

    // Loop
    for i in 1..1000 {
        let mut m = xc::ChannelMsg {
            channel_id: i,
            msg_id: i,
            pb_data: format!("msg numver {}", i).to_bytes(),
        };
        m.save(&my_session);
    }
}

fn main1() {}
