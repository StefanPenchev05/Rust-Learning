use std::collections::HashMap;

fn main() {
    // Create a mutable HashMap
    let mut h = HashMap::new();

    // Insert key-value pairs into the HashMap
    h.insert("Stefan Penchev".to_string(), 1);
    h.insert("Venera Mineva".to_string(), 2);

    // Print the HashMap
    println!("{:?}", h);

    // Create a String from a string literal
    let person_name = String::from("Venera Mineva");

    // Get the value associated with the key from the HashMap, copy it, and if it doesn't exist, return 0
    let index_of_person = h.get(&person_name).copied().unwrap_or(0);

    // Print the HashMap, the key, and the value associated with the key
    println!(
        "From the hash map {:?} the index of person {}, is {:?}",
        h, person_name, index_of_person
    );

    // Iterate over the HashMap and print each key-value pair
    for (key, value) in &h {
        println!("{key} : {value}")
    }

    let mut scores = HashMap::new();

    // Insert a key-value pair into the HashMap
    scores.insert(String::from("Blue"), 10);

    // If the key doesn't exist in the HashMap, insert the key-value pair, otherwise do nothing
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // Print the HashMap
    println!("{:?}", scores);

    let text = "Hello, my name is Stefan, and I think that my name is so cool";

    // Create a new mutable HashMap called repeated_words
    let mut repeated_words = HashMap::new();

    // Iterate over each word in the text
    for word in text.split_whitespace() {
        // If the word exists in the HashMap, increment its counter, otherwise insert the word with a counter of 1
        repeated_words
            .entry(word)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    // Print the HashMap
    // This will print each word in the text and its frequency
    println!("{:?}", repeated_words);
}
