// A struct in Rust is a composite data type, meaning it groups together related data into one logical unit.
// It's similar to a class in other languages, but without methods (though you can add methods with impl blocks).
// We use structs for several reasons:

// Organization: structs allow us to group related data together and name it, which makes our code more readable and organized.

// Type safety: Each field in a struct has a specific type, which the Rust compiler checks at compile time. This helps prevent errors.

// Encapsulation: structs can be used to encapsulate data and behavior together, especially when used with impl blocks.
#[allow(non_camel_case_types)]
struct User {
    active: bool,
    username: String,
    email: String,
    age: u8,
}

// Define a struct named Point that can hold three i32 values
// These values represent the x, y, and z coordinates of a point in 3D space
struct Point(i32, i32, i32);

fn main() {
    let user1: User = build_user(
        String::from("penchev.stefan@icloud.com"),
        String::from("StefanPenchev")
    );

    print_user_details(&user1);

    let user2: User = copy_user_with_different_email(
        user1,
        String::from("petya_pencheva@gmail.com")
    );
    print_user_details(&user2);

    let origin = Point(0, 0, 0);
    println!("The point is at coordinates ({}, {}, {})", origin.0, origin.1, origin.2);
}

// Function to build a new user
// Takes email and username as parameters and returns a User struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        age: 30,
    }
}

// Function to create a copy of a user with a different email
// Takes a User struct and a new email as parameters, and returns a new User struct
fn copy_user_with_different_email(copied_user: User, new_email: String) -> User {
    // Create a new User struct
    User {
        email: new_email, // Set the email to the new email
        ..copied_user // Use the rest of the fields from the copied user
    }
}

// Function to print the details of a user
// Takes a reference to a User struct as parameter
fn print_user_details(user: &User) {
    // Print the user details
    println!("User details:");
    println!("Email: {}", user.email);
    println!("Username: {}", user.username);
    println!("Active: {}", user.active);
    println!("Age: {}", user.age);
}
