use clap::{Arg, Command};

mod examples;
pub mod utils;

use examples::{first, second, third};

fn main() {
    let matches = Command::new("Ray Tracing Examples")
        .version("1.0")
        .author("Vikram Saraph <vikram.saraph.22@gmail.com>")
        .about("Runs different ray tracing examples")
        .arg(
            Arg::new("example")
                .short('e')
                .long("example")
                .help("Specifies which example to run")
                .required(true),
        )
        .get_matches();

    let example = matches.get_one::<String>("example").unwrap().as_str();

    match example {
        "first" => {
            first::run();
        }
        "second" => {
            second::run();
        }
        "third" => {
            third::run();
        }
        _ => {
            println!("Unknown example: {}", example);
            println!("Available examples are: basic, lighting, shadows");
        }
    }
}
