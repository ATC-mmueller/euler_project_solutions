use std::time::Instant;

fn main() {
    let now = Instant::now ();

    // no need to store every value in a list, 
    // just keep the two last ones to be able to calculate the next number
    let mut last_value = 2;

    let mut second_to_last_value = 1;

    let mut next_value: u32 = 0;

    let mut sum: u32 = 2;

    while next_value < 4000000 {
        next_value = second_to_last_value + last_value;

        if next_value % 2 == 0 {
            sum += next_value;
        } 

        second_to_last_value = last_value;

        last_value = next_value;
    }

    println!("The sum of all even Fibonacci numbers not exceeding four million is {sum}");

    println!("Elapsed time: {}ns", now.elapsed().as_nanos());
}
