use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nRectangle:\n\twidth: {}\n\theight: {}\n", self.width, self.height)
    }
}

impl Rectangle {

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle::rectangle(30, 50);
    let rect2 = Rectangle::rectangle(10, 40);
    let rect3 = Rectangle::rectangle(60, 45);

    println!("{}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
