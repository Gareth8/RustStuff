struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
struct _Guess {
    _value: i32,
}
impl _Guess {
    pub fn _new(_value: i32) -> _Guess {
        if _value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {_value}."
            );
        } else if _value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {_value}."
            );
        }

        _Guess { _value }
    }
}
pub fn testing() {
    println!("1 + 2 = {}", adder(1,2));
    println!("2 + 2 = {}", add_two(2));

    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    if larger.can_hold(&smaller) {
        println!("It fits!!");
    }
    else if !larger.can_hold(&smaller) {
        println!("It doesn't fit!");
    }

}
fn adder(x: i32, y: i32) -> i32 {
    x + y
}
fn add_two(x: i32) -> i32 {
    x + 2
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn exploration() {
        let result = adder(1,2);
        assert_eq!(result, 3);
    }
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

        assert!(larger.can_hold(&smaller), "The larger rectangle cannot hold the smaller rectangle.");
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
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    #[test]
    #[should_panic = "less than or equal to 100"]
    fn greater_than_100() {
        _Guess::_new(200);
    }
}