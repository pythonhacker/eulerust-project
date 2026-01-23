// Lattice paths
use std::env;
use euler_lib::{npaths_lattice_r, npaths_lattice_i};

fn main() {
    // Uncomment below line for recursive solution
    // Warning: Its very computational intensive and takes a while to run.

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "-r" {
        println!("Using recursion");
        println!("{}", npaths_lattice_r(0,0,20,20));
    } else {
        println!("{}", npaths_lattice_i(20));
    }
}
