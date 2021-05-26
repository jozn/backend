use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::frame::Frame;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::{QueryExecutor, QueryValues};
use shared::xc::FCQueryExecutor;
use shared::{pb, xc};
use std::ops::Deref;

pub struct FlipCassandraSession {
    session: Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>,
}

impl FCQueryExecutor for &FlipCassandraSession {
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

pub fn get_session() -> FlipCassandraSession {
    let node = NodeTcpConfigBuilder::new("185.239.107.163:9042", NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    let my_session = FlipCassandraSession {
        session: no_compression,
    };

    my_session
}
