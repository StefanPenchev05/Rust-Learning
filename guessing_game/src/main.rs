// we use "use" keyword to import dependecies like io 
// the std is standard lib, for now we will ude std::io and esspecially stdin
use std::{cmp::Ordering, io::stdin};

// Using the dependecy for random value
use rand::Rng;

fn main() {
    println!("Hello to my guessing game!");
    println!("Please enter your number: ");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    // let - is creating new variable, but in rust the variables are imutable by default
    // so to make them mutable we need to use the keyword mut
    // The String::new() create new isntace of String, and ny using the mut we make the number variable
    // to recive data from type string and it is mutable
    let mut number: String = String::new();

    // using stdin for getting the input data from user
    // Read Line is getting the data and puts it in the variabel, and we need to pass the memory address
    // of the variable, by saying &mut we are giving the mutable memory adress of the var
    // Except is for handling the errors
    stdin()
        .read_line(&mut number)
        .expect("Faild to read user input");

    let guess: u32 = number.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your number is small"),
        Ordering::Greater => println!("Your number is a bigger"),
        Ordering::Equal => println!("You win!!!")
    }

    println!("Your guessed number is {number}");
}
