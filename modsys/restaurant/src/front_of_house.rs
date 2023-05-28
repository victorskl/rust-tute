/*
pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
}
*/
// and, we can further split it by creating `front_of_house` folder and corresponding file `hosting.rs`
pub mod hosting;

mod serving {
    fn take_order() {
        println!("take_order called");
    }

    fn serve_order() {
        println!("serve_order called");
    }

    fn take_payment() {
        println!("take_payment called");
    }
}
