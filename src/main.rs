mod cli;
mod dataset;
mod datasettype;
mod meta;
mod misc;
mod origin;
mod property;
mod rawproperty;
mod settings;
mod snapshot;
mod stat;
mod table;
mod zpool;

use cli::cli;

fn main() {
    cli();
}
