mod cli;
mod modules;
mod plugins;
mod utils;

use clap::{Parser, Subcommand};
use cli::{command::*, deploy, init, keygen, relay, status, zk};
use colored::*;

#[derive(Parser)]
#[command(name = "nonosctl", about = "NØNOS Command-Line Interface", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Relay,
    Deploy,
    Zk,
    Keygen,
    Status,
    Plugins,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init::run().await,
        Commands::Relay => relay::run("relay").await,
        Commands::Deploy => deploy::run("deploy").await,
        Commands::Zk => zk::run("zk").await,
        Commands::Keygen => keygen::generate(),
        Commands::Status => status::check(),
        Commands::Plugins => plugins::manager::run(),
    }
}
