fn main() {
    // Create a mutable string
    let mut name = String::from("Stefan Penchev");
    // Append another string to the existing string
    name.push_str(", hello my friend");

    // Create a variable
    let num = 3;

    // Shallow copying: copying the pointer, length, and capacity
    // The Rust compiler will destroy the original variable, so if another variable gets the value of name, it gets the address to the memory, length, and capacity
    let second_name = name;

    // Deep copying: copying the data and takes the same amount of memory as the original variable
    let second_num = num;

    // Print the variables
    println!("{}, {}", second_name, second_num);

    // Some types, like i32 and boolean, have the Copy trait and are directly deep copied
    let s = String::from("Lets try this ownership");
    // The function takes_ownership takes the ownership of s and s cannot be used anymore in this scope
    takes_ownership(s);

    let n: i32 = 4324;
    // The function makes_copy makes a copy of n
    makes_copy(n);

    // Print the variable n
    println!("{}", n);

    // Create a new string
    let s2 = gives_ownership();
    // Print the string
    println!("{}", s2);

    // Call a function that takes a string and returns it
    let s3 = takes_and_gives_ownership(String::from("Takes and Gives back ownership"));
    // Print the returned string
    println!("{}", s3);
}

// Function that takes ownership of a string
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// Function that makes a copy of an i32
fn makes_copy(n: i32) {
    println!("{}", n)
}

// Function that creates a string and gives ownership to the caller
fn gives_ownership() -> String {
    // Create a new string
    let s = String::from("Giving ownership to var");

    // Return the string, giving ownership to the caller
    s
}

// Function that takes a string and gives ownership back to the caller
fn takes_and_gives_ownership(s: String) -> String {
    // Return the string, giving ownership back to the caller
    s
}
