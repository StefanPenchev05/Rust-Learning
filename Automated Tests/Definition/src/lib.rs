pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: String) -> String {
    format!("Hello, {name}")
}

pub struct Rectangle {
    width: i32,
    height: i32,
}

pub struct Guess {
    value: i32,
}

impl Rectangle {
    fn can_hold(&self, other_rec: &Rectangle) -> bool {
        self.width > other_rec.width && self.height > other_rec.height
    }
}

impl Guess {
    fn new(value: i32) -> Self {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let rec1 = Rectangle {
            width: 32,
            height: 54,
        };

        let rec2 = Rectangle {
            width: 12,
            height: 53,
        };

        assert!(rec1.can_hold(&rec2));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let rec1 = Rectangle {
            width: 32,
            height: 54,
        };

        let rec2 = Rectangle {
            width: 12,
            height: 53,
        };

        assert!(!rec2.can_hold(&rec1));
    }

    #[test]
    fn adding_two_to_var() {
        let a = 4;
        assert_eq!(add_two(a), 6);
    }

    #[test]
    fn greeting_conatins_name() {
        let result = greeting(String::from("Stefann"));
        assert!(result.contains("Stefan"), "Greetings did not contain name, values was {}", result);
    }

    #[test]
    #[should_panic]
    fn grater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "value must be greater than or equal to 1")]
    fn less_than_1() {
        Guess::new(-2);
    }

    #[test]
    fn two_plus_two() -> Result<(), String> {
        if 2 + 2 == 4 { Ok(()) } else { Err(String::from("two plus two does not make 5")) }
    }
}
