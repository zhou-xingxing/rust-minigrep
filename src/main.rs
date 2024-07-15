use std::env;
use std::process;

use minigrep::{run, Params};

fn main() {
    // cargo run -- -i -q=query -f=file_path
    let args: Vec<String> = env::args().collect();
    let params = Params::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        println!("A valid command: cargo run -- [-i] -q=<query_string> -f=<file_path>");
        process::exit(1);
    });
    if let Err(err) = run(&params) {
        println!("Application error: {}", err);
        process::exit(1);
    };
}
