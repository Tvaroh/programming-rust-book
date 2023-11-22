use std::env::args;
use std::process::exit;
use std::str::FromStr;

use crate::gcd::gcd;

pub fn run() {
  let numbers: Vec<u64> =
    args()
      .skip(1)
      .map(|arg| u64::from_str(&arg).expect("Error parsing argument"))
      .collect();

  if numbers.len() == 0 {
    eprintln!("Usage: {} NUMBER", args().next().unwrap());
    exit(1);
  }

  let gcd = numbers.iter().skip(1).fold(numbers[0], |acc, n| { gcd(acc, *n) });

  println!("The GCD of {:?} is {}", numbers, gcd);
}
