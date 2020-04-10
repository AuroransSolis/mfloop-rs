mod clap_handler;
mod gpu72_runtime;
mod primenet_runtime;
mod util;

use clap_handler::app::{Options, request_from_args};
use primenet_runtime::primenet_runtime;

fn main() {
    match request_from_args() {
        Ok(o) => {
            // println!("Successfully parsed command line arguments:\n{:?}", o);
            println!("Successfully parsed command line arguments.");
            match o {
                Options::Primenet(primenet_options) => {
                    primenet_runtime(primenet_options).expect("Primenet runtime returned an error:\n")
                }
                Options::Gpu72(_) => println!("GPU to 72 not supported yet."),
            }
        }
        Err(e) => println!("{}", e),
    }
}
