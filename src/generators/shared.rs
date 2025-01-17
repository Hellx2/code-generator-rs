use std::io::{stdin, stdout, Write};

use crate::string::StringExt;

pub fn parse_css_classes(arg: &str) -> String {
    arg.split(',')
        .map(|x| x.trim().replace('"', "").surround('"'))
        .collect::<Vec<String>>()
        .join(", ")
}
pub fn get_args(arg_count: usize, arg_names: &[&str]) -> Vec<String> {
    let mut args = vec![];

    for i in 1..=arg_count {
        print!("Enter the {}: ", arg_names[i - 1]);

        stdout().flush().unwrap();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        args.push(input.trim().to_string());
    }

    println!();

    args
}
