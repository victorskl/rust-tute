// https://doc.rust-lang.org/rust-by-example/custom_types.html
// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// https://doc.rust-lang.org/std/keyword.type.html

use crate::print_break_line;

pub fn explore() {
    print_break_line(32);
    let my_user = build_user("someone@example.com".to_string(), "victor".to_string());
    // println!("{:?}", my_user);
    println!("{}", my_user.username);
    println!("{}", my_user.email);
    println!("{}", my_user.active);
    println!("{}", my_user.sign_in_count);

    print_break_line(32);
    build_some_type();
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// https://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
#[allow(dead_code)]
#[derive(Debug)]
enum SomeType {
    Point { x: i64, y: i64 },
    Mount,
    UnMount,
    Move(String),
    Jump(char),
}

fn build_some_type() {
    // println!("{:?}", SomeType::Point { x: 0, y: 0 })
    let s_type = SomeType::Point { x: 10, y: 20 };
    println!("{:?}", s_type);
    println!("{:?}", SomeType::Mount);
}
