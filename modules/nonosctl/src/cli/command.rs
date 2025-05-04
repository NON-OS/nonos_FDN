use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nonosctl", about = "NONOS Command-Line Interface")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands, 
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Relay,
    Deploy,
    Zk,
    Keygen,
    Status,
    Plugins,
}
