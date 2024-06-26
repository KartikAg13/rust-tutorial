pub fn add_two(a:i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a:i32, b:i32) -> i32 {
    a + b
}

fn prints_and_returns_10(a:i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub struct Guess {  
    value: i32
}

impl Guess {
    pub fn new(value:i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        }
        if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }
        Guess {
            value
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name:&str) -> String {
    format!("Hello {}!", name)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        assert_eq!(result, 4);  //asserts that the two values are equal

        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("two plus does not equal four"))
        }
    }

    #[test]
    #[ignore]   //this ignores the test
    fn failing_test() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Kartik");
        //if the given equation is false, then print the statement
        assert!(
            result.contains("Carol"),
            "Greetings did not contain name, value was {}", result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    //by default the test that fails only prints the output
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
