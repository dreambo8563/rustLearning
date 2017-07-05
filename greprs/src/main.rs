 #![allow(unused_variables)]
extern crate greprs;
use std::env;
use std::process;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let stderr = io::stderr();

    let config = greprs::Config::new(&args).unwrap_or_else(|err| {
        stderr
            .lock()
            .write_fmt(format_args!("Problem parsing arguments: {}", err))
            .expect("Could not write to stderr");
        process::exit(1)
    });
    // println!("args: {:?}", args);
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        // println!("Application error: {}", e);
        stderr
            .lock()
            .write_fmt(format_args!("Application error: {}", e))
            .expect("Could not write to stderr");
        process::exit(1)
    }
}
