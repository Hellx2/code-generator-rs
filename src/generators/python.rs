use std::io::{Error, stdin, stdout, Write};
use std::io::ErrorKind;

use crate::generators::shared::*;
use crate::string::StringExt;

pub fn python() {
    println!("Pick a type of code to generate:");
    println!("1) GTK");
    println!("2) Basic Terminal Games");

    let mut code_type = String::new();

    print!(" > ");
    stdout().flush().unwrap();

    stdin().read_line(&mut code_type).unwrap();

    println!();

    match code_type.trim().parse::<u8>().unwrap() {
        1 => gtk(),
        2 => basic_terminal_games(),
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
                "Gtk.Button(label={}, css_classes=[{}])",
                args[0].replace('"', "").surround('"'),
                parse_css_classes(&args[1])
            );
        }
        2 => {
            let args = get_args(2, &["label", "CSS classes (comma separated)"]);

            println!(
                "Gtk.Label(label={}, css_classes=[{}])",
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
                "Gtk.Box(css_classes=[{}], orientation={})",
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
        "horizontal" | "h" => Ok("Gtk.Orientation.HORIZONTAL".to_string()),
        "vertical" | "v" => Ok("Gtk.Orientation.VERTICAL".to_string()),
        x => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Failed to parse orientation {x}!"),
        )),
    }
}

fn basic_terminal_games() {
    println!("Pick a game to generate:");
    println!("1) High-Low Guessing Game");
    println!("2) Pig Dice Game");

    'tr: loop {
        print!(" > ");

        stdout().flush().unwrap();

        let mut game = String::new();

        stdin().read_line(&mut game).unwrap();

        println!();

        match game.trim().parse::<u8>().unwrap() {
            1 => {
                guessing_game();
                break 'tr;
            }
            a => {
                eprintln!("'{a}' is not an option.");
            }
        }
    }
}

fn guessing_game() {
    print!("Should the guessing game ask the user to set the bounds (Y/n)? ");
    stdout().flush().unwrap();

    let mut ask_user = String::new();

    stdin().read_line(&mut ask_user).unwrap();

    let ask = parse_user_option(ask_user);

    print!("Should the guessing game say if it was lower or higher (Y/n)? ");
    stdout().flush().unwrap();

    let mut ask_user = String::new();

    stdin().read_line(&mut ask_user).unwrap();

    let use_low_high = parse_user_option(ask_user);

    print!("Should there be documentation comments (Y/n)? ");
    stdout().flush().unwrap();

    let mut ask_user = String::new();

    stdin().read_line(&mut ask_user).unwrap();

    let give_docs = parse_user_option(ask_user);

    println!();

    println!("# Put this at the top of the file");
    if give_docs {
        println!("# Import the random library")
    }
    println!("import random");
    println!();
    println!("# Put this somewhere below the 'import random'.");
    if give_docs {
        println!("# High-low guessing game")
    }
    println!("def guessing_game():");
    if ask {
        if give_docs {
            with_tabs("# Get the user to input the lower and upper number", 1)
        }
        with_tabs("lower_bound = int(input('Enter the lower number: '))", 1);
        with_tabs("upper_bound = int(input('Enter the upper number: '))", 1);
    } else {
        if give_docs {
            with_tabs("# The lower and upper number", 1)
        }
        with_tabs("lower_bound = 1", 1);
        with_tabs("upper_bound = 100", 1);
    }
    println!();
    with_tabs(
        "secret_number = random.randint(lower_bound, upper_bound)",
        1,
    );
    println!();
    with_tabs(
        "print(f'Guess a number between {{lower_bound}} and {{upper_bound}}')",
        1,
    );
    println!();
    with_tabs("while True:", 1);
    with_tabs("guess = int(input('Enter your guess: '))", 2);
    println!();
    if use_low_high {
        with_tabs("if guess > secret_number: print('Too high!')", 2);
        with_tabs("elif guess < secret_number: print('Too low!')", 2);
        with_tabs("else:", 2);
        with_tabs("print('You win!')", 3);
        with_tabs("break", 3);
    } else {
        with_tabs("match abs(guess - secret_number):", 2);
        with_tabs("case 0:", 3);
        with_tabs("print('You win!')", 4);
        with_tabs("break", 4);
        with_tabs("case v if v in range(1, 6): print('Boiling!')", 3);
        with_tabs("case v if v in range(6, 11): print('Hot!')", 3);
        with_tabs("case v if v in range(11, 21): print('Warm.')", 3);
        with_tabs("case v if v in range(21, 51): print('Cold.')", 3);
        with_tabs("case _: print('Freezing.')", 3);
        println!()
    }
}

/**
 * Parses yes or no options.
 */
fn parse_user_option(answer: String) -> bool {
    match answer.to_lowercase().trim() {
        "y" | "yes" => true,
        "n" | "no" => false,
        a => {
            eprintln!("Invalid option '{a}'!");
            std::process::exit(1);
        }
    }
}

fn with_tabs(string: &str, tabcount: usize) {
    let mut return_value = "    ".repeat(tabcount);
    return_value.push_str(string);
    println!("{return_value}");
}
