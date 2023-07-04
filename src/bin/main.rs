//! Cosmic entrypoint

use anyhow::Result;

/// Entrypoint to the CLI.
#[tokio::main]
async fn main() -> Result<()> {
    cosmic::cli::run().await?;
    Ok(())
}
