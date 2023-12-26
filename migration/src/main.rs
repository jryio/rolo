use sea_orm_migration::prelude::*;
use tracing;
use tracing_subscriber;

#[async_std::main]
async fn main() {
    println!("JRY DEBUG - migration/cli");
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();
    cli::run_cli(migration::Migrator).await;
}
