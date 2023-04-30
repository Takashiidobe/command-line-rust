use command_line_rust::echo::matches;

fn main() {
    let matches = matches();
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    if omit_newline {
        print!("{}", text.join(" "));
    } else {
        println!("{}", text.join(" "));
    }
}
