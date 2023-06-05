use command_line_rust::diff::{self, Config};
use similar::{ChangeTag, TextDiff};
use std::fs::read_to_string;

fn main() {
    let Config { left, right } = diff::get_args();
    let left_file_str = read_to_string(left).unwrap();
    let right_file_str = read_to_string(right).unwrap();

    let diff = TextDiff::from_lines(&left_file_str, &right_file_str);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{}{}", sign, change);
    }
}
