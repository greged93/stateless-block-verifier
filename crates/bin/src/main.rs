//! Stateless Block Verifier

#[macro_use]
extern crate sbv;

use clap::Parser;
use sbv::core::HardforkConfig;

#[cfg(feature = "dev")]
use tracing_subscriber::EnvFilter;

mod commands;

mod utils;

#[derive(Parser)]
#[command(version, about = "Stateless Block Verifier")]
struct Cli {
    #[command(subcommand)]
    commands: commands::Commands,
    /// Curie block number, defaults to be determined by chain id
    #[arg(short, long)]
    curie_block: Option<u64>,
    /// Start metrics server
    #[cfg(feature = "metrics")]
    #[arg(long)]
    metrics: bool,
    /// Metrics server address
    #[cfg(feature = "metrics")]
    #[arg(long, default_value = "127.0.0.1:9090")]
    metrics_addr: std::net::SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Install the tracing subscriber that will listen for events and filters. We try to use the
    // `RUST_LOG` environment variable and default to RUST_LOG=info if unset.
    #[cfg(feature = "dev")]
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cmd = Cli::parse();

    #[cfg(feature = "metrics")]
    if cmd.metrics {
        sbv::utils::metrics::start_metrics_server(cmd.metrics_addr);
    }

    let get_fork_config = move |chain_id: u64| {
        let mut config = HardforkConfig::default_from_chain_id(chain_id);

        dev_info!("Using hardfork config: {:?}", config);
        if let Some(curie_block) = cmd.curie_block {
            config.set_curie_block(curie_block);
        }
        config
    };

    cmd.commands.run(get_fork_config).await?;

    Ok(())
}
