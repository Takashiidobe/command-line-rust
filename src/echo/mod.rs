use clap::{App, Arg, ArgMatches};

pub fn matches() -> ArgMatches {
    App::new("echo")
        .version("0.1.0")
        .author("Takashi Idobe <mail@takashiidobe.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input Text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches()
}
