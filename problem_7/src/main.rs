use std::time::Instant;
use is_prime::is_prime;

fn main() {

    let now = Instant::now();

    let mut counter = 2;

    let mut current_number = 3;

    while counter < 10001 {
        current_number += 2;

        if is_prime (current_number) {
            counter += 1;
        }
    }

    println!("The 10001st prime number is: {current_number}");
    println!("Calculation time: {}ms", now.elapsed().as_millis());
}
