use chrono::NaiveDateTime;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod listen;

fn extract_time(input: &str) -> Result<String, &str> {
    let foo = input.split(|e| e == '[' || e == ']');
    for bar in foo {
        match NaiveDateTime::parse_from_str(bar, "%Y-%m-%dT%H:%M:%S") {
            Ok(a) => return Ok(a.to_string()),
            Err(_) => {}
        }
        match bar.trim().parse::<f64>() {
            Ok(a) => return Ok(a.to_string()),
            Err(_) => {}
        }
    }

    Err(input)
}

fn handle_line(line: &str) {
    match extract_time(line) {
        Ok(a) => println!("{}", a),
        Err(a) => eprintln!("{}", a),
    }
}

fn main() {
    log4rs::init_file("src/log.yaml", Default::default()).unwrap();

    info!("Hello, World!");

    let files = vec!["data/a", "data/b", "data/c", "data/d"];

    listen::listen(files);
}
