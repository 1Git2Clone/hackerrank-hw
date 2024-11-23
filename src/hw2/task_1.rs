use crate::utils::{
    input::{get_numeric_input, invalid_input},
    Error,
};

const MIN: u32 = 2;
const MAX: u32 = 2000;

/// # HOMEWORK 2 | TASK 1
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
/// The count of all the positive numbers up to (and including) `n` which are divisible by `n`.
///
/// ### Sample
///
/// #### Input Format
///
/// ```txt
/// 200
/// ```
///
/// #### Output Format
///
/// ```txt
/// 12
/// ```
///
/// #### Explanation
///
/// The count of the numbers, divisble by n is 12 and they are:
///
/// 1  -   1
/// 2  -   2
/// 3  -   4
/// 4  -   5
/// 5  -   8
/// 6  -  10
/// 7  -  20
/// 8  -  25
/// 9  -  40
/// 10 -  50
/// 11 - 100
/// 12 - 200
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
/// use hackerrank::hw2::task_1::Solution;
///
/// assert_eq!(
///     Solution::main(200),
///     String::from("12")
/// );
/// assert_eq!(
///     Solution::main(40),
///     String::from("8")
/// );
/// assert_eq!(
///     Solution::main(2001),
///     String::from("Invalid input data!")
/// );
/// assert_eq!(
///     Solution::main(7),
///     String::from("2")
/// );
/// ```
pub struct Solution;

impl Solution {
    pub fn main(n: u32) -> String {
        if !(MIN..=MAX).contains(&n) {
            return invalid_input();
        }
        let mut result = 0;
        for i in 1..=n {
            if n % i == 0 {
                result += 1;
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
