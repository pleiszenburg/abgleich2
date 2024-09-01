mod dataset;
mod datasettype;
mod misc;
mod origin;
mod property;
mod rawproperty;
mod stat;
mod zpool;

use crate::zpool::Zpool;

fn main() {

    let zpool = Zpool::from_local();

    println!("len(datasets) == {:?}", zpool.len());

    zpool.print_tree();

    println!("Yay!");

}
