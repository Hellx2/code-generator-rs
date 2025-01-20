use std::collections::BTreeMap;
use std::io::ErrorKind;
use std::io::{stdin, stdout, Error, Write};

use crate::generators::shared::*;

pub struct RustGTKGenerator {
    pub typename: String,
    pub args: BTreeMap<&'static str, (String, Option<fn(&str) -> String>)>,
}

impl RustGTKGenerator {
    pub fn new(
        typename: &str,
        args: BTreeMap<&'static str, (String, Option<fn(&str) -> String>)>,
    ) -> Self {
        Self {
            typename: typename.to_string(),
            args,
        }
    }

    pub fn generate(&self, args: Vec<String>) -> String {
        let mut return_value = format!("gtk4::{}::builder()", self.typename);

        for (i, (&arg, _)) in self.args.iter().enumerate() {
            if let Some((_, Some(arg_fn))) = self.args.get(arg) {
                return_value.push_str(&format!(".{}", arg));
                return_value.push_str(&format!("({})", arg_fn(args[i].as_str())));
            } else {
                return_value.push_str(&format!(".{}", arg));
                return_value.push_str(&format!("(\"{}\")", args[i]));
            }
        }

        return_value + ".build()"
    }
}

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

pub fn gtk() {
    // let args = get_args(2, &["label", "CSS classes (comma separated)"]);
    let css_classes_parser: fn(&str) -> String = parse_css_classes;
    let css_classes = (
        "css_classes",
        (
            "CSS Classes (comma separated)".to_owned(),
            Some(css_classes_parser),
        ),
    );
    let label = ("label", ("label".to_owned(), None));

    let generator_list = vec![
        RustGTKGenerator::new("Button", map([label.clone(), css_classes.clone()])),
        RustGTKGenerator::new("Label", map([label.clone(), css_classes.clone()])),
        RustGTKGenerator::new(
            "Application",
            map([("application_id", ("App ID".to_owned(), None))]),
        ),
        RustGTKGenerator::new(
            "Box",
            map([
                css_classes,
                (
                    "orientation",
                    (
                        "orientation (horizontal or vertical)".to_owned(),
                        Some(|x| parse_orientation(x).unwrap()),
                    ),
                ),
            ]),
        ),
    ];

    println!("Pick something to generate:");
    for (i, j) in generator_list.iter().enumerate() {
        println!("{}) {}", i + 1, j.typename);
    }
    print!(" > ");
    stdout().flush().unwrap();

    let mut what_to_generate = String::new();
    stdin().read_line(&mut what_to_generate).unwrap();

    println!();

    let gen = &generator_list[what_to_generate.trim().parse::<u8>().unwrap() as usize - 1];
    let x = gen
        .args
        .iter()
        .map(|(_, (name, _))| name.as_str())
        .collect::<Vec<&str>>();
    println!("{}", gen.generate(get_args(x.len(), &x)));
}

pub fn parse_orientation(arg: &str) -> Result<String, Error> {
    match arg.to_lowercase().replace('"', "").as_str() {
        "horizontal" | "h" => Ok("gtk4::Orientation::HORIZONTAL".to_string()),
        "vertical" | "v" => Ok("gtk4::Orientation::VERTICAL".to_string()),
        x => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Failed to parse orientation {x}!"),
        )),
    }
}

pub fn map<K: Ord, V, const N: usize>(props: [(K, V); N]) -> BTreeMap<K, V> {
    BTreeMap::from(props)
}
