use clap::{App, Arg};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Config {
    pub left: String,
    pub right: String,
}

pub fn get_args() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("diff")
        .arg(Arg::with_name("left").takes_value(true))
        .arg(Arg::with_name("right").takes_value(true))
        .get_matches();

    let left = matches
        .value_of("left")
        .expect("Expected a left to be provided")
        .to_string();
    let right = matches
        .value_of("right")
        .expect("Expected a left to be provided")
        .to_string();
    Config { left, right }
}
