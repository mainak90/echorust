extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("echorust")
        .version("0.0.1")
        .author("Mainak Dhar <mainak90@gmail.com>")
        .about("Unix echo in Rustlang")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}