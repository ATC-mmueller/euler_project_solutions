pub fn is_prime(number: u64) -> bool {
    let upper_limit = (number as f64).sqrt().floor() as u64;
  
    for factor in 2..upper_limit+1 {
      if factor == 2 || factor == 3 {
        if number % factor == 0 {
        return false;
        }
      }
  
      let remainder = factor % 6;
  
      if remainder == 1 || remainder == 5 {
        if number % factor == 0 {
          return false;
        }
      } 
    }
  
    return true;
}