use crate::utils::{
    input::{get_input, get_numeric_input, invalid_input},
    Error,
};

const MIN_CIPHER: u32 = 0;
const MAX_CIPHER: u32 = 25;

/// # HOMEWORK 4 | TASK 1
///
/// ## Input
///
/// 1. One str (the text to cipher) and 1 number (the cipher amount).
///
/// ### Constaints
///
/// 2. The cipher amount should be in the inclusive range of: `[0;25]`.
///
/// ### Output
///
/// The ciphered version of the the input text by the cipher amount.
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
/// use hackerrank::hw4::task_1::Solution;
///
/// assert_eq!(
///                                                      // lol
///     Solution::main("Declaring and Invoking Functions in C++!", 3),
///     String::from("Ghfodulqj dqg Lqyrnlqj Ixqfwlrqv lq F++!")
/// );
/// assert_eq!(
///     Solution::main("Discrete Mathematics and Programming", 3),
///     String::from("Glvfuhwh Pdwkhpdwlfv dqg Surjudpplqj")
/// );
/// assert_eq!(
///     Solution::main("Discrete Mathematics and Programming", 5),
///     String::from("Inxhwjyj Rfymjrfynhx fsi Uwtlwfrrnsl")
/// );
/// assert_eq!(
///     Solution::main("Reading and Printing 2D Arrays", 0),
///     String::from("Reading and Printing 2D Arrays")
/// );
/// assert_eq!(
///     Solution::main("Reading and Printing 2D Arrays", 26),
///     String::from("Invalid input data!")
/// );
/// assert_eq!(
///     Solution::main("Generate code!", 22),
///     String::from("Cajanwpa ykza!")
/// );
/// assert_eq!(
///     Solution::main("Exchanging Knowledge", 10),
///     String::from("Ohmrkxqsxq Uxygvonqo")
/// );
/// ```
pub struct Solution;

impl Solution {
    pub fn main(str: &str, cipher: u32) -> String {
        if cipher == 0 {
            return str.to_string();
        }
        if !(MIN_CIPHER..=MAX_CIPHER).contains(&cipher) {
            return invalid_input();
        }

        str.chars()
            .map(|c| match c {
                'A'..='Z' => ((((c as u8) - b'A' + cipher as u8) % 26) + b'A') as char,
                'a'..='z' => ((((c as u8) - b'a' + cipher as u8) % 26) + b'a') as char,
                _ => c,
            })
            .collect()
    }
}

pub fn main() -> Result<(), Error> {
    let str = get_input()?;
    let cipher = get_numeric_input::<u32>()?;

    println!("{}", Solution::main(&str, cipher));

    Ok(())
}
