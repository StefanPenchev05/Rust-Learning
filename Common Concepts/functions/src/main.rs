fn main() {
    // Functions are the same as other languages
    // So i am going to write onlly the exceptions

    // cannot assign var like other languages for example x = y = 6;
    // but you can make a inner function that returns a value
    let x: i32 = {
        let mut y: i32 = 43;

        y += 100;
        y
    };

    println!("{}", x)
}
