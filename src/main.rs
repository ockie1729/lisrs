mod parse;
mod token;

use std::io;

use parse::parse;

fn main() {
    let mut input_str: String = String::new();

    println!("input lisp expressoin:");
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");

    match parse(&input_str) {
        Ok(expr) => println!("{expr:?}"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
