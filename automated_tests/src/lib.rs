/*    
#[cfg(test)]
mod tests{
    #[test]
    fn exploration(){
        assert_eq!(2 + 2, 4)
    }
    #[test]
    fn another(){
        panic!("Make this test fail")
    }
}
*/

#[derive(Debug)]
struct Rectangle{
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.heigth > other.heigth
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{
            width: 8,
            heigth: 7,
        };
        let smaller = Rectangle{
            width: 5,
            heigth: 1,
        };
        assert!(larger.can_hold(&smaller))
    }
}


//testing equality
pub fn add_two(a: i32) -> i32{
    a + 2
}

#[cfg(test)]
mod tests2{
    use super::*;

    #[test]
    fn it_adds_two(){
        assert_eq!(4, add_two(2))
    }
}


//check panics with should_panic
pub struct Guess{
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess {value}
    }
}

#[cfg(test)]
mod tests3{
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(101);
    }
}