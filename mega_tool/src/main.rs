use clap::Parser;

use mega_tool::{
    command::{Cli, Commands},
    crate_to_repo::convert_crate_to_repo,
    handle_repo::add_and_push_to_remote,
    incremental_update::incremental_update,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    match args.command {
        Commands::Upload => {
            add_and_push_to_remote(args.workspace).await;
        }
        Commands::Crate => {
            convert_crate_to_repo(args.workspace).await;
        }
        Commands::Incremental => {
            incremental_update().await;
        }
    }
}
