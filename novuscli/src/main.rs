use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::env;

mod help;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        help::help_menu();
    }
}