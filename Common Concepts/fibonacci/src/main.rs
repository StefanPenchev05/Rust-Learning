fn fibonacci(n: u32) -> u128 {
    let mut fib = vec![0u128, 1u128];
    for i in 2..=n as usize {
        let next_fib = fib[i - 1] + fib[i - 2];
        fib.push(next_fib);
    }
    fib[n as usize]
}

fn main() {
    let n = 4;
    println!("Fibonacci of {} is {:?}", n, fibonacci(n));
}