extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // [1] => Search Term, [2] => File
     let mut config: Config = Config { query: String::new(), filename: String::new() };
    {
        let args: Vec<String> = env::args().collect();
        config = Config::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    } // Ainda consigo usar o config por causa do lifetime static

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
