use std::time::Instant;

fn factorial(n: u128) -> u128 {
    (1..=n).product() 
}

fn main() {
    let n = 30;

    let start_time = Instant::now();
    let result = factorial(n);
    let duration = start_time.elapsed();

    println!("Factorial of {} is {}", n, result);
    println!("Execution time: {:?}", duration);
}
