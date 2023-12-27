use sea_orm_migration::prelude::*;
use tracing;
use tracing_subscriber;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
