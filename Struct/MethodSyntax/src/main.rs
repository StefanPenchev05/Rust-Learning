#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug)]

// Define a struct named Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement a method for the Rectangle struct
// In Rust, methods are defined within an 'impl' block.
impl Rectangle {
    // Define a method named 'area' that takes a reference to self
    // 'self' refers to the instance of the struct the method is being called on.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Function to check if the width of the rectangle is greater than 0
    fn width(&self) -> bool {
        self.width > 0
    }

    // Function to check if one rectangle can hold another
    // It does this by comparing the width and height of the two rectangles
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        dbg!(
            ("Comparing widths", self.width, ">", other_rect.width),
            ("Comparing heights", self.height, ">", other_rect.height)
        );
        self.width > other_rect.width && self.height > other_rect.height
    }

    // Function to create a new Rectangle instance where width and height are equal, forming a square
    fn square(size: u32) -> Self {
        Self{
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 32,
        height: 48,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        &rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(32);
    println!(
        "The area of the square is {} square pixels.",
        &square1.area()
    );

}
