use std::io;
use std::time::Instant;

fn main() {
  println!("Choose a number:");

  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line.");

  let number: u32 = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("No number has been chosen.");
      return;
    }
  };

  // For measuring calculation time
  let now = Instant::now();

  let mut sum: u32 = 0;

  for num in 1..number {
    if num % 3 == 0 {
      sum += num;
    } else if num % 5 == 0 {
      sum += num;
    }
  }

  println!("The sum of every number lower than {number} that is divisible by 3 or 5 is: {sum}");

  println!("The calculation took {}ms.", now.elapsed().as_millis());
}
