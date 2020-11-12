#[cfg(test)]  // <-- https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#unit-tests
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // You can't use the #[should_panic] annotation on tests that use Result<T, E>.
    // Instead, you should return an Err value directly when the test should fail.
    #[test]
    fn it_works_2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Ignore Test
    // https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested
    // cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }
}

#[cfg(test)]
mod tests_rect {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // correct logic
        self.width > other.width && self.height > other.height

        // intentionally bug it
        // self.width < other.width && self.height > other.height
    }
}

// ---

#[cfg(test)]
mod tests_3 {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("Carol"));
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // Testing Private Functions
    // https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#testing-private-functions
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

pub fn add_two(a: i32) -> i32 {
    // correct logic
    // a + 2

    // intentionally bug it
    // a + 3

    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    // correct logic
    format!("Hello {}!", name)

    // intentionally bug it
    // String::from("Hello!")
}

// ---

#[cfg(test)]
mod tests_guess {
    use super::*;

/*
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
*/
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // correct logic
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        // intentionally bug it
        // if value < 1 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        // ---

        // correct logic
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        // intentionally bug it
/*
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }
*/
        Guess { value }
    }
}

// ---
// https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html#showing-function-output
// cargo test this_test_will_pass -- --show-output

#[cfg(test)]
mod tests_4 {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
