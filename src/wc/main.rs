use command_line_rust::wc;

fn main() {
    if let Err(e) = wc::get_args().and_then(wc::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
