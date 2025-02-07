use futures::future::join_all;
use notary_client::{Accepted, NotarizationRequest, NotaryClient};
use tlsn_common::config::ProtocolConfig;
use tlsn_prover::{Prover, ProverConfig};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

// Number of Prover instances to run simultaneously
const INSTANCES: usize = 16;

// Setting of the application server
const SERVER_DOMAIN: &str = "discord.com";

// Notary server
const NOTARY_HOST: &str = "127.0.0.1";
const NOTARY_PORT: u16 = 7047;

// Maximum number of bytes that can be sent from prover to server
const MAX_SENT_DATA: usize = 1 << 12;
// Maximum number of bytes that can be received by prover from server
const MAX_RECV_DATA: usize = 1 << 14;

#[tracing::instrument()]
async fn instance(id: usize) {
    // Build a client to connect to the notary server.
    let notary_client = NotaryClient::builder()
        .host(NOTARY_HOST)
        .port(NOTARY_PORT)
        // WARNING: Always use TLS to connect to notary server, except if notary is running locally
        // e.g. this example, hence `enable_tls` is set to False (else it always defaults to True).
        .enable_tls(false)
        .build()
        .unwrap();

    // Send requests for configuration and notarization to the notary server.
    let notarization_request = NotarizationRequest::builder()
        .max_sent_data(MAX_SENT_DATA)
        .max_recv_data(MAX_RECV_DATA)
        .build()
        .unwrap();

    let Accepted {
        io: notary_connection,
        id: _session_id,
        ..
    } = notary_client
        .request_notarization(notarization_request)
        .await
        .expect("Could not connect to notary. Make sure it is running.");

    // Set up protocol configuration for prover.
    let protocol_config = ProtocolConfig::builder()
        .max_sent_data(MAX_SENT_DATA)
        .max_recv_data(MAX_RECV_DATA)
        .build()
        .unwrap();

    // Create a new prover and set up the MPC backend.
    let prover_config = ProverConfig::builder()
        .server_name(SERVER_DOMAIN)
        .protocol_config(protocol_config)
        .build()
        .unwrap();

    let _prover = Prover::new(prover_config)
        .setup(notary_connection.compat())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env()
        .unwrap();

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let mut futures = Vec::new();

    for id in 0..INSTANCES {
        futures.push(instance(id));
    }

    join_all(futures).await;
}
