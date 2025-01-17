use std::io::{stdin, stdout, Write};

mod generators;
mod string;

fn main() {
    println!("Pick a language to generate code for:");
    println!("1) Python");
    println!("2) Rust");

    let mut lang = String::new();

    print!(" > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut lang).unwrap();

    println!();

    match lang.trim().parse::<u8>().unwrap() {
        1 => {
            generators::python::python();
        }
        2 => {
            generators::rust::rust();
        }
        a => {
            eprintln!("Invalid option {a}!");
            std::process::exit(1);
        }
    }
}
