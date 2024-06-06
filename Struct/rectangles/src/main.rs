// The #[derive(Debug)] annotation automatically adds the Debug trait to the Rectangle struct.
// The Debug trait enables formatting of the struct for output, typically used for debugging purposes.
// With this trait, instances of the struct can be printed using the {:?} or {:#?} format specifiers.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Define an enum that can be either a Rectangle or two u32 values
enum RectangleDefinition {
    Dimensions(u32, u32),
    Rectangle(Rectangle),
}

fn main() {
    // This is the most simpole way of doing this
    let width = 32;
    let height = 14;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_of_rectangle(RectangleDefinition::Dimensions(width, height))
    );

    // Implement finding the are of rectangle with struct
    let rec1 = build_rectangle_struct(42, 18);
    println!(
        "Strcut Rectangle with values width: {} and height: {} is\n {:#?}",
        rec1.width, rec1.height, rec1
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area_of_rectangle(RectangleDefinition::Rectangle(rec1))
    );

    // Using the dbg! macro, which is error message - stderr
    let rec2 = build_rectangle_struct(100, 100);
    dbg!(&rec2);
    
}

// This function calculates the area of a rectangle.
// It accepts a RectangleDefinition enum which can be either a Rectangle struct or two u32 values (width and height).
fn area_of_rectangle(def: RectangleDefinition) -> u32 {
    // Use pattern matching to determine the type of RectangleDefinition
    match def {
        // If it's Dimensions, multiply the width and height to get the area
        RectangleDefinition::Dimensions(width, height) => width * height,
        // If it's a Rectangle, multiply the width and height fields of the Rectangle to get the area
        RectangleDefinition::Rectangle(rect) => rect.width * rect.height,
    }
}

fn build_rectangle_struct(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}
