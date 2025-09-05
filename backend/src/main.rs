// $env:RUST_LOG = "info"
use tracing;

mod poker_logic;
mod utils;
mod program;
mod problems;

#[tokio::main]
async fn main() {
     // Initializing logging/tracing 
    tracing_subscriber::fmt::init();

    if let Err(e) = program::run().await {
        tracing::error!("Application error: {}", e);
    }
}
