use std::fs::read_to_string;

use crate::print_break_line;

pub fn explore() {
    print_break_line(32);
    read_line_by_line();
}

fn read_line_by_line() {
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

    for line in read_to_string("Cargo.toml").unwrap().lines() {
        println!("{}", line.to_string());
    }
}
