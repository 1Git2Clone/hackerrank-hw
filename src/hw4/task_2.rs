use crate::utils::{
    input::{get_input, invalid_input},
    Error,
};

const JANUARY: u32 = 1;
const FEBRUARY: u32 = 2;
// const MARCH: u32 = 3;
const APRIL: u32 = 4;
// const MAY: u32 = 5;
const JUNE: u32 = 6;
// const JULY: u32 = 7;
// const AUGUST: u32 = 8;
const SEPTEMBER: u32 = 9;
// const OCTOBER: u32 = 10;
const NOVEMBER: u32 = 11;
const DECEMBER: u32 = 12;

const YEAR_BASELINE: u32 = 1900;
const BEFORE_1900_INC: u32 = 20;
const AFTER_1999_INC: u32 = 40;

/// # HOMEWORK 4 | TASK 2
///
/// ## Input
///
/// 1. The bulgarian identity number (`ID` from now on) as a string.
///
/// ### Constaints
///
/// 2. The ID should be exactly 10 characters long.
/// 3. The characters correspond to:
///     - 0 & 1 = Last 2 letters of the year
///     - 2 & 3 = Month
///         - The month is increased by 20 if it's before 1900
///         - The month should be between 1-12 without change
///         - The month is increased by 40 if it's after 1999
///     - 4 & 5 = The day which should be valid based on the month and whether it's a leap year or not
///     - 6 & 7 & 8 = These are dependant on the region and are only important for calculating 9
///     - 9 (control number) = Check if the control number is valid based on this formula:
///         - `[9] = (2*[0]+4*[1]+8*[2]+5*[3]+10*[4]+9*[5]+7*[6]+3*[7]+6*[8]) % 11`
///         - NOTE: If the control number is 10, then set it to 0
///
/// NOTE: All of this needs to be validated.
///
/// For more references:
///
/// - Bulgarian (more content - worth machine translating):
///     - https://bg.wikipedia.org/wiki/%D0%95%D0%B4%D0%B8%D0%BD%D0%B5%D0%BD_%D0%B3%D1%80%D0%B0%D0%B6%D0%B4%D0%B0%D0%BD%D1%81%D0%BA%D0%B8_%D0%BD%D0%BE%D0%BC%D0%B5%D1%80#%D0%9F%D1%80%D0%B8%D0%BC%D0%B5%D1%80%D0%B8
/// - English
///     - https://en.wikipedia.org/wiki/Unique_citizenship_number
///
/// ### Output
///
/// Output `String::from("1")` if the ID is valid or if it's invalid and it has the proper length,
/// return `String::from("0")` or else `String::from("Invalid input data!")`.
///
/// ### Error
///
/// If the input has a different amount of characters (than 10), return:
///
/// ```rust
/// String::from("Invalid input data!");
/// ```
///
/// If the input is invalid, return:
///
/// ```rust
/// String::from("0");
/// ```
///
/// ## Test cases
///
/// ```rust
/// use hackerrank::hw4::task_2::Solution;
///
/// assert_eq!(
///     Solution::main("9001011238"),
///     String::from("1")
/// );
/// assert_eq!(
///     Solution::main("0241011120"),
///     String::from("1")
/// );
/// assert_eq!(
///     Solution::main("9021011124"),
///     String::from("1")
/// );
/// assert_eq!(
///     Solution::main("9001011235"),
///     String::from("0")
/// );
/// assert_eq!(
///     Solution::main("0241011123"),
///     String::from("0")
/// );
/// assert_eq!(
///     Solution::main("9021011129"),
///     String::from("0")
/// );
/// assert_eq!(
///     Solution::main("9054011238"),
///     String::from("0")
/// );
/// assert_eq!(
///     Solution::main("9001321238"),
///     String::from("0")
/// );
/// assert_eq!(
///     Solution::main("9035011238"),
///     String::from("0")
/// );
/// assert_eq!(
///     Solution::main("12345678901"),
///     String::from("Invalid input data!")
/// );
/// ```
pub struct Solution;

impl Solution {
    pub fn valid_date(year: u32, month: u32, day: u32) -> bool {
        if !(JANUARY..=DECEMBER).contains(&month) || !(1..=31).contains(&day) {
            return false;
        }

        if month == FEBRUARY && (day > (if year % 4 == 0 { 29 } else { 28 })) {
            return false;
        }

        //               [      Months with 30 days.      ]
        if day == 31 && ([APRIL, JUNE, SEPTEMBER, NOVEMBER].contains(&month)) {
            return false;
        }

        true
    }
    pub fn main(id: &str) -> String {
        if id.len() != 10 {
            return invalid_input();
        }

        // Validate date.
        {
            let (Ok(mut year), Ok(mut month), Ok(day)) = (
                id[0..=1].parse::<u32>(),
                id[2..=3].parse::<u32>(),
                id[4..=5].parse::<u32>(),
            ) else {
                return "0".to_string();
            };

            year += YEAR_BASELINE;

            let before_1900 =
                ((BEFORE_1900_INC + JANUARY)..=(BEFORE_1900_INC + DECEMBER)).contains(&month);

            let after_1999 =
                ((AFTER_1999_INC + JANUARY)..=(AFTER_1999_INC + DECEMBER)).contains(&month);

            if before_1900 {
                month -= BEFORE_1900_INC;
                year -= 100;
            } else if after_1999 {
                month -= AFTER_1999_INC;
                year += 100;
            }

            if !(JANUARY..=DECEMBER).contains(&month) {
                return "0".to_string();
            }

            if !Self::valid_date(year, month, day) {
                return "0".to_string();
            }
        }

        let first_nine_digits = {
            let mut tmp = [0u8; 9];
            for (i, c) in id.chars().enumerate().take(9) {
                if c == '0' {
                    continue;
                }
                tmp[i] = c as u8 - b'0';
            }
            tmp
        };
        let control_digit = {
            let mut tmp = (2 * first_nine_digits[0]
                + 4 * first_nine_digits[1]
                + 8 * first_nine_digits[2]
                + 5 * first_nine_digits[3]
                + 10 * first_nine_digits[4]
                + 9 * first_nine_digits[5]
                + 7 * first_nine_digits[6]
                + 3 * first_nine_digits[7]
                + 6 * first_nine_digits[8])
                % 11;
            if tmp == 10 {
                tmp = 0
            }
            tmp
        };

        match id.chars().last() {
            Some(c) => match control_digit == (c as u8 - b'0') {
                true => "1".to_string(),
                false => "0".to_string(),
            },
            None => unreachable!(),
        }
    }
}

pub fn main() -> Result<(), Error> {
    let id = get_input()?;

    println!("{}", Solution::main(&id));

    Ok(())
}
