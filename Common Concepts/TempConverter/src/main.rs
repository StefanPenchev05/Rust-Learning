use std::io::stdin;

#[allow(non_snake_case)]
fn main() {
    let option = [
        "Convert from Celsius to Fahrenheit",
        "Convert from Fahrenheit to Celsius",
    ];

    println!("1. {}\n2. {}", option[0], option[1]);

    let answer: i8 = get_integer("Please select the option", |parsed_string| {
        parsed_string > 0 && parsed_string <= option.len() as i32
    }) as i8;

    match answer {
        1 => {
            let degree: i32 = get_integer("Enter your celsius: ", |_| {
                return true;
            }) as i32;
            let result = cals_to_fahr(degree);
            println!("Celsius {degree} in fahrenheit is {result}")
        }
        2 => {
            let degree: i32 = get_integer("Enter your fahrenheit: ", |_| {
                return true;
            }) as i32;
            let result = fahr_to_cals(degree);
            println!("Fahrenheit {degree} in celsius is {result}")
        }
        _ => {}
    }
}

fn cals_to_fahr(degree: i32) -> f32 {
    let mut fahrenheit: f32 = degree as f32;
    fahrenheit = fahrenheit * 1.8 + 32.0;
    fahrenheit
}

fn fahr_to_cals(degree: i32) -> f32{
    let mut celsius:f32 = degree as f32;
    celsius = (celsius - 32.0) * 1.8;
    celsius
}

fn get_integer<F>(print_string: &str, validation_func: F) -> usize
where
    F: Fn(i32) -> bool,
{
    println!("{}", print_string.to_owned());

    loop {
        let mut string: String = String::new();

        stdin().read_line(&mut string).expect("Faild to read data");

        match string.trim().parse::<i32>() {
            Ok(parsed_string) => {
                if validation_func(parsed_string) {
                    return parsed_string as usize;
                } else {
                    println!("Invalid input, please try again.");
                }
            }
            Err(_) => {
                println!("Please enter a number!");
            }
        }
    }
}
