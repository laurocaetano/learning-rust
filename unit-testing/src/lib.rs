///
/// # Simple Struct for learning purposes
///
/// also using the docs notation for learning purposes.
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

///
/// # Simple function for testing the doc tests
///
/// ```
/// assert_eq!(unit_testing::plus_one(&4), 5);
/// ```
pub fn plus_one(number: &i32) -> i32 {
    number + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one_adds_one() {
        assert_eq!(plus_one(&3), 4);
    }

    #[test]
    fn larger_rectangle_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };

        let smaller = Rectangle {
            width: 8,
            height: 7,
        };

        assert!(larger.can_hold(&smaller));
    }
}
