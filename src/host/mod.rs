use clap::{App, Arg};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Config {
    pub hostname: String,
}

pub fn get_args() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("host")
        .arg(Arg::with_name("hostname").takes_value(true))
        .get_matches();

    let hostname = matches
        .value_of("hostname")
        .expect("Expected a hostname to be provided")
        .to_string();
    Config { hostname }
}
