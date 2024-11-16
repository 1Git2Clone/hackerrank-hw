use crate::utils::{gcd::Gcd, input::get_numeric_input, Error};

const MIN: u8 = 20;
const MAX: u8 = 100;

pub fn main() -> Result<(), Error> {
    let user_input = [
        get_numeric_input::<u8>()?,
        get_numeric_input::<u8>()?,
        get_numeric_input::<u8>()?,
    ];

    if user_input.iter().any(|x| *x < MIN || *x > MAX) {
        return Err(format!(
            "One or more of the user input variables ({:?}) are not in the range - [{}; {}]!",
            user_input, MIN, MAX
        )
        .into());
    }

    println!(
        "{:?}",
        user_input
            .iter()
            .skip(1)
            .fold(user_input[0], |acc, x| acc.gcd(*x))
    );

    Ok(())
}
