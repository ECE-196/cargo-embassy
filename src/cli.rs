pub mod init_args;

use clap::{Parser, Subcommand};
use init_args::InitArgs;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum Cargo {
    #[command(subcommand)]
    Embassy(Embassy),
}

#[derive(Debug, Clone, Subcommand)]
pub enum Embassy {
    #[command(about = "Initializes an Embassy project in the current workspace")]
    Init(InitArgs),
}
