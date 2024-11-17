use crate::utils::{input::get_numeric_input, Error};

const MIN_NUM_SYSTEM: u8 = 2;
const MAX_NUM_SYSTEM: u8 = 16;
const MIN: u8 = 1;
const MAX: u8 = 100;

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
/// ### Error
///
/// If the input is invalid:
///
/// ```rs
/// return String::from("Invalid input data!");
/// ```
pub struct Solution;

impl Solution {
    pub fn main(numeric_system: u32, mut decimal_input: u32) -> String {
        if (numeric_system < MIN_NUM_SYSTEM.into() || numeric_system > MAX_NUM_SYSTEM.into())
            || (decimal_input < MIN.into() || decimal_input > MAX.into())
        {
            return String::from("Invalid input data!");
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
