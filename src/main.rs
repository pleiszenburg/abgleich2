mod dataset;
mod datasettype;
mod misc;
mod origin;
mod property;
mod rawproperty;
mod stat;
mod table;
mod zpool;

use std::path::PathBuf;

use crate::zpool::Zpool;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "abgleich")]
#[command(about = "abgleich, zfs sync tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// show dataset tree
    #[command(arg_required_else_help = true)]
    Tree {
        /// configuration file
        #[arg(required = true)]
        config: PathBuf,
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Tree { config } => {
            println!("Tree! {}", config.to_str().unwrap());
            let zpool = Zpool::from_local();
            zpool.print_tree();
        }
    }
}
