use anyhow::Result;
use clap::Parser;
use quincy::cli::Args;
use quincy::config::{FromPath, ServerConfig};
use quincy::server::QuincyServer;
use quincy::utils::enable_tracing;
use tracing::error;

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    match run_server(args).await {
        Ok(_) => {}
        Err(e) => error!("A critical error occurred: {e}"),
    }
}

async fn run_server(args: Args) -> Result<()> {
    let config = ServerConfig::from_path(&args.config_path, &args.env_prefix)?;
    enable_tracing(&config.log.level);

    let server = QuincyServer::new(config).await?;
    server.run().await
}
