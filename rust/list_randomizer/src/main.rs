use std::io::{self, Write};
use rand::prelude::*;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    match io::stdout().flush() {
        Ok(_flush_successful) => {},
        Err(e) => println!("Error on stdout flush: {}", e),
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(e) => println!("Error on read line: {}", e),
    }
    input.trim().to_string()
}

fn main() {
    println!("List of Movies separated by ',':");
    let input = get_input("──> ");
    let movies: Vec<String> = input
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    if movies.len() == 0 {
        println!("No movie added!");
        return;
    }

    let num = rand::thread_rng().gen_range(0, movies.len());

    println!("The movie is: {}", movies.get(num).unwrap());
}
