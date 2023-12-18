use std::fs;
use std::fs::read_to_string;
use std::path::Path;

use crate::print_break_line;

pub fn explore() {
    print_break_line(32);
    read_line_by_line();

    print_break_line(32);
    ls_dir();

    print_break_line(32);
    path_finder();
}

fn read_line_by_line() {
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

    for line in read_to_string("Cargo.toml").unwrap().lines() {
        println!("{}", line.to_string());
    }
}

fn ls_dir() {
    let paths = fs::read_dir("./src").expect("dir not found");
    for path in paths {
        println!("{}", path.expect("path not found").path().display());
    }
}

fn path_finder() {
    // https://doc.rust-lang.org/stable/std/path/struct.Path.html
    // https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html
    // https://doc.rust-lang.org/std/path/struct.Components.html

    let path = Path::new("/tmp/foo/bar.txt");
    for component in path.components() {
        println!("{component:?}");
    }

    // https://stackoverflow.com/questions/37388107/how-to-convert-the-pathbuf-to-string
    let path = Path::new("/foo/bar");
    let parent = path.parent().unwrap();
    assert_eq!(parent, Path::new("/foo"));
    println!("{}", parent.display().to_string());

    // https://stackoverflow.com/questions/73845791/how-to-remove-path-and-get-the-filename-in-rust
    let path = Path::new("my_folder/file.txt");
    let filename = path.file_name().unwrap();
    println!("{}", filename.to_str().unwrap());
}
