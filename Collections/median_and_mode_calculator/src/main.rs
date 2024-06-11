use std::collections::HashMap;

fn main() {
    let mut numbers: Vec<i32> = vec![1, 2, 2, 3, 4, 4, 4, 5];

    // Sort the vec
    numbers.sort();

    // finding the median in vec
    let median = if numbers.len() % 2 == 0 {
        (numbers[numbers.len() / 2 - 1] + numbers[numbers.len() / 2]) as f32 / 2.0
    } else {
        numbers[numbers.len() / 2] as f32
    };

    // Fidning the mode in vec
    let mut map = HashMap::new();
    for n in &numbers {
        map.entry(n)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mode = **map.iter().max_by_key(|&(_, count)| count).unwrap().0;

    println!("Median: {median}");
    println!("Mode: {mode}")
}
