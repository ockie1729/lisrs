use std::io::{stdout, Write};

fn main() {
    print!("lis.rs> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    let answer = input.trim().to_string();

    println!("{}", answer);
}
