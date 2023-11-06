mod var;
mod custom_type;
mod file_io;

fn print_break_line(count: usize) {
    // https://stackoverflow.com/questions/35280798/printing-a-character-a-variable-number-of-times-with-println
    // println!("{:♥<1$}", "", count);
    println!("{:-<1$}", "", count);
}

fn main() {
    println!("Hello from Primer!");
    print_break_line(32);

    var::explorer();
    custom_type::explore();
    file_io::explore();
}
