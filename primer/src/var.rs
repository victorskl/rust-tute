pub fn explorer() {

    let x = 5;
    let y = 10;
    // curly bracket in double quote is a positional placeholder
    println!("x = {} and y = {}", x, y); // println! is a macro that prints a string to the screen

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
