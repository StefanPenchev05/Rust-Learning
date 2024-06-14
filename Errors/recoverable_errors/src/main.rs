use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

// Open File
fn open_file(filename: &str) -> Result<File, std::io::Error> {
    File::open(filename)
}

// Create file
fn create_file(filename: &str) -> Result<File, std::io::Error> {
    File::create(filename)
}

fn login(username: &str, password: &str) -> Result<bool, std::io::Error>{
    let users_file_name = "users.txt";
    let mut content = String::new();
    let _users_file = open_file(&users_file_name)?.read_to_string(&mut content);

    let user_credentials = format!("{}, {}", username, password);
    Ok(content.lines().any(|line| line == user_credentials))
}

fn main() {
    let filename_for_match = "hello.txt";
    let greating_file_result = open_file(&filename_for_match);

    // using match
    let _greating_file = match greating_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match create_file(&filename_for_match) {
                Ok(fs) => fs,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // using if statment
    let filename_for_if = "hello_second.txt";
    let _greating_file_result = open_file(&filename_for_if).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            create_file(&filename_for_if).unwrap_or_else(|error: Error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    if login("username", "passsword").unwrap(){
        println!("Logged In Successfully!");
    } else {
        println!("Wrong Credentials!");
    }
}
