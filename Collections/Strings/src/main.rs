#[allow(dead_code)]
fn main() {
    // In Rust, the term "string" usually refers to the string slice 'str' in its borrowed form '&str'.
    // String slices are references to some UTF-8 encoded string data stored elsewhere. For instance, string literals are stored in the program’s binary and are therefore string slices.
    //
    // The 'String' type, provided by Rust’s standard library, is a growable, mutable, owned, UTF-8 encoded string type.
    // When Rustaceans refer to “strings” in Rust, they might be referring to either the 'String' or the string slice '&str' types.
    // Both 'String' and string slices are used heavily in Rust’s standard library, and both 'String' and string slices are UTF-8 encoded.

    // Initialize an empty string using the `String::new()` method
    let mut _s = String::new();

    // Define a string slice
    let data = "initial contents";

    // Convert the string slice to a `String` using the `to_string()` method
    let _s = data.to_string();

    // The `to_string()` method can also be used directly on a string literal
    let _s = "initial contents".to_string();

    // The `String::from()` method can be used to create a `String` from a string literal
    let _s = String::from("Initial contents");

    // Initialize a mutable string `s` with the value "foo"
    let mut s = String::from("foo");
    // Append the string "bar" to `s` using the `push_str` method
    // The `push_str` method takes a string slice because it doesn't need to take ownership
    s.push_str("bar");

    // Initialize another mutable string `s1` with the value "foo"
    let mut s1 = String::from("foo");
    // Define a string slice `s2` with the value "bar"
    let s2 = "bar";
    // Append the value of `s2` to `s1` using the `push_str` method
    // Again, `push_str` does not take ownership of `s2`, so it's still valid and can be used afterwards
    s1.push_str(s2);
    // Print the value of `s2`
    println!("s2 is {}", s2);


    // Define a string with the value "Hola"
    let hello = String::from("Hola");
    // The length of `hello` is 4, as each character in "Hola" takes 1 byte when encoded in UTF-8

    // Define a string with the value "Здравствуйте"
    let hello = String::from("Здравствуйте");
    // The length of `hello` is 24, as each Unicode scalar value in "Здравствуйте" takes 2 bytes of storage when encoded in UTF-8

    // Define a string with the value "नमस्ते"
    let namaste = "नमस्ते";
    // The string "नमस्ते" is stored as a vector of u8 values
    // When viewed as Unicode scalar values, the string "नमस्ते" is represented as ['न', 'म', 'स', '्', 'त', 'े']
    // When viewed as grapheme clusters, the string "नमस्ते" is represented as ["न", "म", "स्", "ते"]

    // Define a string with the value "Здравствуйте"
    let hello = "Здравствуйте";
    // Create a string slice `s` that contains the first 4 bytes of the string
    let s = &hello[0..4];
    // The value of `s` is "Зд", as each character in "Зд" takes 2 bytes of storage when encoded in UTF-8

    for c in "Здр".chars(){
        println!("{c}");
    }

    for b in "Здр".bytes(){
        println!("{b}");
    }
}
