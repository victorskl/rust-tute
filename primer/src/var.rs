use std::path::Path;
use crate::print_break_line;

pub fn explorer() {
    print_formatting();
    print_break_line(32);

    var_referencing();
    print_break_line(32);

    string_manipulation();
    print_break_line(32);

    vector_explorer();
    print_break_line(32);

    path_finder();
}

fn print_formatting() {
    let x = 5;
    let y = 10;
    // curly bracket in double quote is a positional placeholder
    println!("x = {} and y = {}", x, y); // println! is a macro that prints a string to the screen

    // https://doc.rust-lang.org/std/fmt/
    // https://www.google.com/search?q=rust+string+format+variable
    let my_fmt = format!("\
Some multiple lines formatting \
with var replacement like x:{x} \
and y:{y}.\nAnd a new line here.");

    println!("{}", my_fmt);
}

fn var_referencing() {
    // Rust variables are immutable by default
    let immutable_foo = 32; // immutable foo
    println!("immutable_foo = {}", immutable_foo);
    // immutable_foo = 10; // rustc --explain E0384

    // Use `mut` before the variable to make it mutable
    let mut mutable_foo = 3; // mutable foo
    println!("mutable_foo = {}", mutable_foo);
    mutable_foo = 10;
    println!("mutable_foo = {}", mutable_foo);

    // The `&foo` references (addresses) are immutable by default, too.
    // Hence, similarly, `&mut foo` for mutable reference.

    // Associated function means static method i.e. in a form of `String::new();`
}

fn string_manipulation() {
    let my_str_lit = "literal &str";
    println!("{}", my_str_lit);

    let my_str = String::from("a b c");
    println!("{}", my_str);

    // https://stackoverflow.com/questions/27043268/convert-a-string-to-int
    let str_to_int: i8 = "42".parse().unwrap();
    println!("{}", str_to_int);

    // https://stackoverflow.com/questions/34606043/how-do-i-replace-specific-characters-idiomatically-in-rust
    let result = str::replace("Hello World!", "!", "?");
    println!("{}", result);

    // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
    let parts = "some string 123 content".split(" ");
    let collection: Vec<&str> = parts.collect();
    dbg!(collection);
}

fn vector_explorer() {
    // https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/vectors.html
    // https://doc.rust-lang.org/book/ch08-01-vectors.html
    // https://doc.rust-lang.org/std/vec/struct.Vec.html

    let vector_elements: Vec<f64> = vec![1.0, 2.3, 4.56];
    println!("{}", vector_elements[1]); // 2.3

    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    let vec2 = Vec::from([1, 2, 3, 4]);
    assert_eq!(vec1, vec2);
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
