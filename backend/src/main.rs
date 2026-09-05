use backend::program;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = program::run().await {
        tracing::error!("Application error: {}", e);
    }
}
