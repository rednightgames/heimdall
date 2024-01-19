use cdrs_tokio::authenticators::NoneAuthenticatorProvider;
use cdrs_tokio::cluster::session::SessionBuilder;
use cdrs_tokio::cluster::session::TcpSessionBuilder;
use cdrs_tokio::cluster::{session, NodeTcpConfigBuilder, TcpConnectionManager};
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::transport::TransportTcp;
use std::env;
use std::sync::Arc;

pub type Session = session::Session<
    TransportTcp,
    TcpConnectionManager,
    RoundRobinLoadBalancingStrategy<TransportTcp, TcpConnectionManager>,
>;

pub async fn connect() -> Session {
    let url = env::var("SCYLLA_URL")
        .map_err(|_| "SCYLLA_URL must be set")
        .unwrap();

    let cluster_config = NodeTcpConfigBuilder::new()
        .with_contact_point(url.into())
        .with_authenticator_provider(Arc::new(NoneAuthenticatorProvider))
        .build()
        .await
        .unwrap();

    TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), cluster_config)
        .build()
        .await
        .unwrap()
}
