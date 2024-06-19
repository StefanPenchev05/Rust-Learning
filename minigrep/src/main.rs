mod lib;

use std::env;
use lib::Args;
use lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();
    config.run().unwrap();
}
