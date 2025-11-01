use colored::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle {
    width: u8,
    length: u8,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.length > other.length;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 8);
    }

    #[test]
    fn explore() {
        panic!("test panicked");
    }

    #[test]
    fn test_can_hold() {
        let r1 = Rectangle {
            width: 20,
            length: 30,
        };
        let r2 = Rectangle {
            width: 30,
            length: 10,
        };

        let msg: ColoredString = "this is custom error msg".red();

        assert!(r1.can_hold(&r2), "{}", msg);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        let a = 20;

        if a > 50 {
            panic!("panicked");
        }
    }
}
