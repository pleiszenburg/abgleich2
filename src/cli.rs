use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::zpool::Zpool;

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

pub fn cli() {

    let args = Cli::parse();

    match args.command {
        Commands::Tree { config } => {

            let zpool = Zpool::from_local();
            zpool.print_tree();

        }
    }

}
