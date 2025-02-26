use std::time::Instant;

fn main() {
    
    let now = Instant::now();

    // 
    for number in (100..997).rev() {
        let palindrome = 1000*number + rev_three_digit_number(number);

        for factor in (100..999).rev() {
            let second_factor = palindrome as f64 / factor as f64;

            if second_factor.fract () != 0.0 {
                continue;
            }

            if second_factor > 100.0 && second_factor < 1000.0 {
                println!("The largest palindrome that can be written as a product of two 3-digit numbers is {palindrome} = {factor} x {second_factor}");
                println!("Calculation time: {}ms", now.elapsed().as_millis());
                return;
            }
        }
    }
}

fn rev_three_digit_number (number: u32) -> u32 {
    if number > 999 || number < 100 {
        println!("The argument is not a 3-digit number.");
        return 0;
    } else {
        let single_digit = number % 10 as u32;
        let hundreds_digit = number / 100;

        return number - 99*hundreds_digit + 99*single_digit;
    }
}