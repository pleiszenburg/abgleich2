mod cli;
mod dataset;
mod datasettype;
mod misc;
mod origin;
mod property;
mod rawproperty;
mod stat;
mod table;
mod zpool;

use cli::cli;

fn main() {
    cli();
}
