use cdrs_tokio::authenticators::NoneAuthenticatorProvider;
use cdrs_tokio::cluster::session::SessionBuilder;
use cdrs_tokio::cluster::session::TcpSessionBuilder;
use cdrs_tokio::cluster::{session, NodeTcpConfigBuilder, TcpConnectionManager};
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::transport::TransportTcp;
use log::error;
use std::env;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

pub type Session = session::Session<
    TransportTcp,
    TcpConnectionManager,
    RoundRobinLoadBalancingStrategy<TransportTcp, TcpConnectionManager>,
>;

pub async fn connect() -> Session {
    let timeout_duration = Duration::from_secs(5);
    let url = env::var("SCYLLA_URL")
        .map_err(|_| "SCYLLA_URL must be set")
        .unwrap();

    let cluster_config = NodeTcpConfigBuilder::new()
        .with_contact_point(url.into())
        .with_authenticator_provider(Arc::new(NoneAuthenticatorProvider))
        .build()
        .await
        .unwrap();

    let session = match timeout(
        timeout_duration,
        TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), cluster_config).build(),
    )
    .await
    {
        Ok(session) => session,
        Err(_err) => {
            error!("Error connecting to Scylla: Connection timeout");
            exit(0x1);
        }
    };

    match session {
        Ok(session) => session,
        Err(err) => {
            error!("Error connecting to Scylla: {}", err);
            exit(0x1);
        }
    }
}
