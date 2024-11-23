use crate::utils::{
    input::{get_numeric_input, invalid_input},
    lcm::Lcm,
    Error,
};

const MIN: u32 = 10;
const MAX: u32 = 100;

/// # HOMEWORK 1 | TASK 3
///
/// ## Input
///
/// 1. Three numbers
///
/// ### Constaints
///
/// 1. Each number should be in the inclusive range of: `[10;100]`.
///
/// ### Output
///
/// The [LCM](<https://en.wikipedia.org/wiki/Least_common_multiple>) of the three numbers.
///
/// ### Sample
///
/// #### Input Format
///
/// ```txt
/// 10
/// 20
/// 30
/// ```
///
/// #### Output Format
///
/// ```txt
/// 60
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
/// use hackerrank::hw1::task_3::Solution;
///
/// assert_eq!(
///     Solution::main([10, 20, 30]),
///     String::from("60")
/// );
/// assert_eq!(
///     Solution::main([9, 10, 20]),
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
                .fold(user_input[0], |acc, x| acc.lcm(*x))
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
