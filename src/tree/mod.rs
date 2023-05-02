use clap::{App, Arg};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct Config {
    pub dir: String,
}

pub fn get_args() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("tree")
        .arg(Arg::with_name("dir").takes_value(true).default_value("."))
        .get_matches();

    let dir = matches
        .value_of("dir")
        .expect("Expected a dir to be provided")
        .to_string();
    Config { dir }
}
