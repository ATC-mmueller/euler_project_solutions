use std::time::Instant;
use is_prime::is_prime;

fn main() {
    let now = Instant::now();

    let number: u64 = 600851475143;

    // Prime factors can not be grater that the square root of the number
    let upper_limit: u64 = (number as f64).sqrt() as u64;

    for factor in (1..upper_limit).rev() {
        if !is_prime(factor) {
            continue;
        }
            
        if number % factor == 0 {
            println!("The largest prime factor of {number} is {factor}.");
            println!("Calculation time: {}ms", now.elapsed().as_millis());
            return;
        }
    }
}
