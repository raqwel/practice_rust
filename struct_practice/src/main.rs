#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle{ width : size, height : size }
    }
}

fn main() {
    let rectangle1 = Rectangle { width: 30, height: 50 };
    let rectangle2 = Rectangle { width: 10, height: 40 };
    let rectangle3 = Rectangle { width: 60, height: 45 };

    println!("rectangle is {:#?}", rectangle1);

    println!(
        // 四角形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        rectangle1.area()
    );
    println!("Can rect1 hold rect2? {}", rectangle1.can_hold(&rectangle2));
    println!("Can rect1 hold rect3? {}", rectangle1.can_hold(&rectangle3));
}

//fn area(rect: &Rectangle) -> u32 {
//    rect.width * rect.height
//}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_hold_over_width_only() {
        let rectangle1 = Rectangle { width: 30, height: 50 };
        let rectangle2 = Rectangle { width: 40, height: 40 };
        debug_assert_eq!(rectangle1.can_hold(&rectangle2), false)
    }

    #[test]
    fn can_hold_over_height_only() {
        let rectangle1 = Rectangle { width: 30, height: 50 };
        let rectangle2 = Rectangle { width: 20, height: 60 };
        debug_assert_eq!(rectangle1.can_hold(&rectangle2), false)
    }

    #[test]
    fn can_hold() {
        let rectangle1 = Rectangle { width: 30, height: 50 };
        let rectangle2 = Rectangle { width: 20, height: 40 };
        debug_assert_eq!(rectangle1.can_hold(&rectangle2), true)
    }
}