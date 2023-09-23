mod compiler;
mod grammar;

use crate::{compiler::Compiler, grammar::File};
use peginator::PegParser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            // check flags
            let input = &args[1];
            match input.as_str() {
                "-h" | "--help" => {
                    println!("Usage: {} <filename>", args[0]);
                    return;
                }
                "-v" | "--version" => {
                    println!("Version: {}", env!("CARGO_PKG_VERSION"));
                    return;
                }
                "-i" | "--interactive" => {
                    println!("Interactive mode not implemented yet");
                    return;
                }
                filename => {
                    let contents = match std::fs::read_to_string(filename) {
                        Ok(c) => c,
                        Err(e) => {
                            println!("Error reading file: {}", e);
                            return;
                        }
                    };
                    let ast = match File::parse(&contents) {
                        Ok(f) => f,
                        Err(e) => {
                            println!("Error parsing file: {}", e);
                            return;
                        }
                    };
                    Compiler::compile(filename, ast);
                }
            }
        }
        _ => {
            println!(
                "\x1b[1m\x1b[31m{:>12}\x1b[0m {} <filename>",
                "Usage", args[0]
            );
            println!("{:>12} {} -h | --help", "", args[0]);
            println!("{:>12} {} -v | --version", "", args[0]);
            println!("{:>12} {} -i | --interactive", "", args[0]);
            println!(
                "\x1b[1m\x1b[31m{:>12}\x1b[0m {} example/main.bds",
                "Example", args[0]
            );
            println!("{:>12} cargo run -- example/main.bds", "");
        }
    }
}
