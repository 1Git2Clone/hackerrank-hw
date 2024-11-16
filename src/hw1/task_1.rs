use crate::utils::{input::get_numeric_input, Error};

const MIN_NUM_SYSTEM: u8 = 2;
const MAX_NUM_SYSTEM: u8 = 16;
const MIN: u8 = 1;
const MAX: u8 = 100;

pub fn main() -> Result<(), Error> {
    let numeric_system = get_numeric_input::<u16>()?;
    let mut decimal_input = get_numeric_input::<u16>()?;

    if numeric_system < MIN_NUM_SYSTEM.into() || numeric_system > MAX_NUM_SYSTEM.into() {
        return Err(format!(
            "Numeric system ({}) is not in the range - [{}; {}]!",
            numeric_system, MIN_NUM_SYSTEM, MAX_NUM_SYSTEM
        )
        .into());
    } else if decimal_input < MIN.into() || decimal_input > MAX.into() {
        return Err(format!(
            "Decimal input ({}) is not in the range - [{}; {}]!",
            decimal_input, MIN, MAX
        )
        .into());
    }

    let result = {
        let mut temp = "".to_string();
        let encode_numeric_input = |x| {
            char::from_digit(x, numeric_system.into())
                .unwrap()
                .to_ascii_uppercase()
        };
        while decimal_input > 0 {
            temp.push(encode_numeric_input(
                (decimal_input % numeric_system) as u32,
            ));
            decimal_input /= numeric_system;
        }
        temp.chars().rev().collect::<String>()
    };

    println!("{}", result);

    Ok(())
}
