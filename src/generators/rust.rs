use std::io::{Error, stdin, stdout, Write};
use std::io::ErrorKind;

use crate::generators::shared::*;
use crate::string::StringExt;

pub fn rust() {
    println!("Pick a type of code to generate:");
    println!("1) GTK");

    let mut code_type = String::new();

    print!(" > ");
    stdout().flush().unwrap();

    stdin().read_line(&mut code_type).unwrap();

    println!();

    match code_type.trim().parse::<u8>().unwrap() {
        1 => gtk(),
        a => {
            eprintln!("Invalid option {a}!");
            std::process::exit(1);
        }
    }
}

fn gtk() {
    println!("Pick something to generate:");
    println!("1) Button");
    println!("2) Label");
    println!("3) Box");

    print!(" > ");
    stdout().flush().unwrap();

    let mut what_to_generate = String::new();
    stdin().read_line(&mut what_to_generate).unwrap();

    println!();

    match what_to_generate.trim().parse::<u8>().unwrap() {
        1 => {
            let args = get_args(2, &["label", "CSS classes (comma separated)"]);

            println!(
                "gtk4::Button::builder().label({}).css_classes([{}]).build()",
                args[0].replace('"', "").surround('"'),
                parse_css_classes(&args[1])
            );
        }
        2 => {
            let args = get_args(2, &["label", "CSS classes (comma separated)"]);

            println!(
                "gtk4::Label::builder().label({}).css_classes([{}]).build()",
                args[0].replace('"', "").surround('"'),
                parse_css_classes(&args[1])
            );
        }
        3 => {
            let args = get_args(
                2,
                &[
                    "CSS classes (comma separated)",
                    "orientation (vertical or horizontal)",
                ],
            );

            println!(
                "gtk4::Box::builder().css_classes([{}]).orientation({}).build()",
                parse_css_classes(&args[0]),
                parse_orientation(&args[1]).unwrap(),
            )
        }
        a => {
            eprintln!("Invalid argument {a}")
        }
    }
}

fn parse_orientation(arg: &str) -> Result<String, Error> {
    match arg.to_lowercase().replace('"', "").as_str() {
        "horizontal" | "h" => Ok("gtk4::Orientation::HORIZONTAL".to_string()),
        "vertical" | "v" => Ok("gtk4::Orientation::VERTICAL".to_string()),
        x => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Failed to parse orientation {x}!"),
        )),
    }
}
