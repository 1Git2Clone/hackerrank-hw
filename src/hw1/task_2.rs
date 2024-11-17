use crate::utils::{gcd::Gcd, input::get_numeric_input, Error};

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
/// ### Error
///
/// If the input is invalid:
///
/// ```rs
/// return String::from("Invalid input data!");
/// ```
pub struct Solution;

impl Solution {
    pub fn main(user_input: [u32; 3]) -> String {
        if user_input.iter().any(|x| *x < MIN || *x > MAX) {
            return String::from("Invalid input data!");
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
