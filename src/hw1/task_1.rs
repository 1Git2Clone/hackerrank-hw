use crate::utils::{
    input::{get_numeric_input, invalid_input},
    Error,
};

const MIN_NUM_SYSTEM: u32 = 2;
const MAX_NUM_SYSTEM: u32 = 16;
const MIN: u32 = 1;
const MAX: u32 = 100;

/// # HOMEWORK 1 | TASK 1
///
/// ## Input
///
/// 1. numeric system (radix)
///
/// 2. decimal input
///
/// ### Constaints
///
/// 1. The numeric system should be in the inclusive range of: `[2;16]`.
///
/// 2. The decimal input should be in the inclusive range of: `[1;100]`.
///
/// ### Output
///
/// A string representation of the decimal user input encoded in the numeric system radix (from
/// binary to hexadecimal).
///
/// ### Sample
///
/// #### Input Format
///
/// ```txt
/// 2
/// 34
/// ```
///
/// #### Output Format
///
/// ```txt
/// 10010
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
/// use hackerrank::hw1::task_1::Solution;
///
/// assert_eq!(
///     Solution::main(2, 8),
///     String::from("1000")
/// );
/// assert_eq!(
///     Solution::main(2, 34),
///     String::from("100010")
/// );
/// assert_eq!(
///     Solution::main(2, 1),
///     String::from("1")
/// );
/// assert_eq!(
///     Solution::main(8, 22),
///     String::from("26")
/// );
/// assert_eq!(
///     Solution::main(8, 34),
///     String::from("42")
/// );
/// assert_eq!(
///     Solution::main(16, 34),
///     String::from("22")
/// );
/// assert_eq!(
///     Solution::main(16, 12),
///     String::from("C")
/// );
/// assert_eq!(
///     Solution::main(17, 5),
///     String::from("Invalid input data!")
/// );
/// ```
pub struct Solution;

impl Solution {
    pub fn main(numeric_system: u32, mut decimal_input: u32) -> String {
        if (!(MIN_NUM_SYSTEM..=MAX_NUM_SYSTEM).contains(&numeric_system))
            || (!(MIN..=MAX).contains(&decimal_input))
        {
            return invalid_input();
        }

        let result = {
            let mut temp = "".to_string();
            let encode_numeric_input = |x| {
                char::from_digit(x, numeric_system)
                    .unwrap()
                    .to_ascii_uppercase()
            };
            while decimal_input > 0 {
                temp.push(encode_numeric_input(decimal_input % numeric_system));
                decimal_input /= numeric_system;
            }
            temp.chars().rev().collect::<String>()
        };
        result
    }
}

pub fn main() -> Result<(), Error> {
    let numeric_system = get_numeric_input::<u32>()?;
    let decimal_input = get_numeric_input::<u32>()?;

    println!("{}", Solution::main(numeric_system, decimal_input));

    Ok(())
}
