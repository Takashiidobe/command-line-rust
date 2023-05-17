use clap::{App, Arg};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Config {
    pub binary: String,
}

pub fn get_args() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("ldd")
        .arg(Arg::with_name("binary").takes_value(true))
        .get_matches();

    let binary = matches
        .value_of("binary")
        .expect("Expected a binary to be provided")
        .to_string();
    Config { binary }
}
