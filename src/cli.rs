use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::settings::Settings;
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

    },

    /// create snapshots of changed datasets for backups
    #[command(arg_required_else_help = true)]
    Snap {

        /// configuration file
        #[arg(required = true)]
        config: PathBuf,

    },

}

pub fn cli() {

    let args = Cli::parse();

    match args.command {

        Commands::Tree { config } => {

            let settings = Settings::from_configfile(config.to_str().unwrap());

            let zpool = Zpool::from_cmd(&settings.source.host, &settings.source.root);
            zpool.print_tree();

        },

        Commands::Snap { config } => {

            let settings = Settings::from_configfile(config.to_str().unwrap());

            let zpool = Zpool::from_cmd(&settings.source.host, &settings.source.root);
            let transactions = zpool.get_snapshot_transaction(
                settings.always_changed,
                settings.written_threshold,
                settings.check_diff,
                &settings.suffix,
                &settings.ignore,
            );
            transactions.print_table();

        },

    }

}
