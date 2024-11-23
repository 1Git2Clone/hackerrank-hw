use crate::utils::{
    gcd::Gcd,
    input::{get_numeric_input, invalid_input},
    Error,
};

const MIN: u32 = 20;
const MAX: u32 = 100;

/// # HOMEWORK 1 | TASK 2
///
/// ## Input
///
/// 1. Three numbers
///
/// ### Constaints
///
/// 1. Each number should be in the inclusive range of: `[20;100]`.
///
/// ### Output
///
/// The [GCD](<https://en.wikipedia.org/wiki/Greatest_common_divisor>) of the three numbers.
///
/// ### Sample
///
/// #### Input Format
///
/// ```txt
/// 24
/// 40
/// 64
/// ```
///
/// #### Output Format
///
/// ```txt
/// 8
/// ```
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
/// use hackerrank::hw1::task_2::Solution;
///
/// assert_eq!(
///     Solution::main([24, 40, 64]),
///     String::from("8")
/// );
/// assert_eq!(
///     Solution::main([23, 25, 26]),
///     String::from("1")
/// );
/// assert_eq!(
///     Solution::main([5, 25, 60]),
///     String::from("Invalid input data!")
/// );
/// assert_eq!(
///     Solution::main([20, 20, 301]),
///     String::from("Invalid input data!")
/// );
/// ```
pub struct Solution;

impl Solution {
    pub fn main(user_input: [u32; 3]) -> String {
        if user_input.iter().any(|x| *x < MIN || *x > MAX) {
            return invalid_input();
        }

        format!(
            "{}",
            user_input
                .iter()
                .skip(1)
                .fold(user_input[0], |acc, x| acc.gcd(*x))
        )
    }
}

pub fn main() -> Result<(), Error> {
    let user_input = [
        get_numeric_input::<u32>()?,
        get_numeric_input::<u32>()?,
        get_numeric_input::<u32>()?,
    ];

    println!("{}", Solution::main(user_input));

    Ok(())
}
