extern crate minigrep;

use std::process;
use minigrep::Config;

fn main() {
    // [1] => Search Term, [2] => File
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
