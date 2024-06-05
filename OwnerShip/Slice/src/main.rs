use std::io::stdin;

fn main() {
    let mut user_input: String = String::from("");

    stdin()
        .read_line(&mut user_input)
        .expect("Error while entering your input");

    
    let position: &str = first_word(&user_input);
    println!("{}", position);
}

fn first_word(string: &String) -> &str {
    let bytes_from_string = string.as_bytes();
    for (i, &item) in bytes_from_string.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }

    &string[..]
}
