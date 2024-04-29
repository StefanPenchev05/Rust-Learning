use std::io::stdin;

fn main() {
   // the int types - ix / ux (integer / unsigned)
   // x can be 8;16;32 that represents the bits
   let i8: u8 = 98;
   println!("{i8}");

   // floting points are the same - f32...
   let float: f32 = 23.9;
   println!("{float}");

   // tuple type
   let tup: (i32, char, String) = (7564, 'C', "hello".to_owned());

   //printing tuples
   println!("{}", tup.0);

   // The arrays are the same as other languages
   // But there are some diff things

   // By saying 3;5 the comoiler understands that the number 3 is going to be 5 times in the array
   let _arr = [3; 5];

   // Exercise
   search_by_index();
}

fn search_by_index(){
    let arr: [i32; 5] = [1,2,3,4,5];

    println!("Please enter the index");

    let mut index = String::new();

    loop {
        index.clear();

        stdin()
            .read_line(&mut index)
            .expect("Failed to read the input data");

        let result: Result<i8, _> = index.trim().parse();

        match result {
            Ok(parsed_index) => {
                if parsed_index > 0 && parsed_index < arr.len() as i8 {
                    println!("The result is {}", arr[parsed_index as usize]);
                    break;
                }else{
                    println!("Out of index, the max index is {}", arr.len() - 1)
                }
            },
            Err(_) => {
                println!("Please type a number!")
            }
        }
    }
}
