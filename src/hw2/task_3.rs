use crate::utils::{
    input::{get_numeric_input, invalid_input},
    Error,
};

const MIN: u32 = 2;
const MAX: u32 = 2000;

/// # HOMEWORK 2 | TASK 3
///
/// ## Input
///
/// 1. One number
///
/// ### Constaints
///
/// 2. The input should be in the inclusive range of: `[2;2000]`.
///
/// ### Output
///
/// The sum of all the positive numbers up to (but **excluding**) `n` which are divisible by `n`
/// and are NOT prime numbers.
///
/// ### Sample
///
/// #### Input Format
///
/// ```txt
/// 12
/// ```
///
/// #### Output Format
///
/// ```txt
/// 10
/// ```
///
/// #### Explanation
///
/// The divisors of 12 are 2, 3, 4, and 6
///
/// The sum of the divisors that are not prime is: 4 + 6 = 10.
///
/// ### Error
///
/// If the input is invalid, return:
///
/// ```rust
/// String::from("Invalid input data!");
/// ```
///
/// ## Test cases
///
/// ```rust
/// use hackerrank::hw2::task_3::Solution;
///
/// assert_eq!(
///     Solution::main(12),
///     String::from("10")
/// );
/// assert_eq!(
///     Solution::main(20),
///     String::from("14")
/// );
/// assert_eq!(
///     Solution::main(2003),
///     String::from("Invalid input data!")
/// );
/// ```
pub struct Solution;

impl Solution {
    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }

        for i in 2..=((n as f32).sqrt() as u32) {
            if n % i == 0 {
                return false;
            }
        }

        true
    }
    pub fn main(n: u32) -> String {
        if !(MIN..=MAX).contains(&n) {
            return invalid_input();
        }

        let mut result = 0;
        for i in 2..n {
            if n % i == 0 && !Self::is_prime(i) {
                result += i;
            }
        }

        format!("{}", result)
    }
}

pub fn main() -> Result<(), Error> {
    let n = get_numeric_input::<u32>()?;

    println!("{}", Solution::main(n));

    Ok(())
}

