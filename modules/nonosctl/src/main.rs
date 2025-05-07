mod cli;
mod modules;
mod plugins;
mod utils;

use clap::{Parser, Subcommand};
use cli::{deploy, init, keygen, relay, status, zk};

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            init::run().await;
            Ok(())
        }
        Commands::Relay => {
            relay::run("relay").await;
            Ok(())
        }
        Commands::Deploy => deploy::run("deploy").await,
        Commands::Zk => {
            zk::run("zk").await;
            Ok(())
        }
        Commands::Keygen => {
            keygen::generate();
            Ok(())
        }
        Commands::Status => {
            status::check();
            Ok(())
        }
        Commands::Plugins => {
            plugins::manager::run();
            Ok(())
        }
    }
}
