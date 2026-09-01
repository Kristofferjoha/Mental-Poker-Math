// $env:RUST_LOG = "info"

use backend::program;

#[tokio::main]
async fn main() {
    // Initializing logging/tracing
    tracing_subscriber::fmt::init();

    if let Err(e) = program::run().await {
        tracing::error!("Application error: {}", e);
    }
}
