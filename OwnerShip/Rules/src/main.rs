fn main() {
    let mut name = String::from("Stefan Penchev");
    name.push_str(", hello my frined"); // append other string to already made up string

    let num = 3;
    
    // The rust compiler will destroy the name before goes to heap
    // so is some veriable get the value of name, it gets the address to the memroy and len and capacity

    // shallow copy - copying the pointer, length and capacity
    // deep copy- copying the data and takes memory as the first var

    let second_name = name; // shallow copying
    let second_num = num.clone(); // deep copying
    println!("{second_name} {num} {second_num}");
}
