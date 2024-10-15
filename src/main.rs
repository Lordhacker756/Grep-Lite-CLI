use grep_lite::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error in parsing input args:: {}", err);
        process::exit(0);
    });

    if let Err(e) = run(config) {
        println!("Error while searching in file:: {}", e);
        process::exit(0);
    };
}
