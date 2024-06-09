fn main() {
    // Creating a new Vector of type i32
    let _l: Vec<i32> = Vec::new();

    // Using the vec! macro to create and initialize a vector with values
    let _v = vec![12, 43, 54];

    // Creating a mutable vector so we can change it later
    let mut _v = Vec::new();

    // Adding elements to the vector using the push method
    _v.push(12);
    _v.push(32);
    _v.push(123);
    _v.push(43);

    // Accessing the third element of the vector directly
    // Note: This will panic if the element doesn't exist
    let third = &_v[2];
    println!("The third element in the _v vector is {third}");

    // Accessing the third element of the vector using the get method
    // This method returns an Option, which can be used to handle the case where the element doesn't exist
    let third: Option<&i32> = _v.get(2);

    // Using a match statement to handle the Option returned by the get method
    // If the element exists (Some), it's printed
    // If the element doesn't exist (None), a default message is printed
    match third {
        Some(third) => println!("The third element in the _v vector is {third}"),
        None => println!("There is no third element."),
    }

    // Initialize a vector with integers from 1 to 6
    let v = vec![1,2,3,4,5,6];

    // Iterate over each element in the vector and print it
    for i in v{
        println!("{i}")
    }

    // Initialize a mutable vector with the values 100, 32, and 57
    let mut v = vec![100, 32, 57];

    // Iterate over each element in the mutable vector
    // The &mut keyword allows us to modify the values in place
    for i in &mut v{
        // Add 50 to the current value of i
        *i+= 50;
        // Print the new value of i
        println!("{i}")
    }
}
