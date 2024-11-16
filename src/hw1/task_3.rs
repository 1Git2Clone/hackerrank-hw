use crate::utils::{input::get_numeric_input, lcm::Lcm, Error};

const MIN: u8 = 10;
const MAX: u8 = 100;

pub fn main() -> Result<(), Error> {
    let user_input = [
        get_numeric_input::<u16>()?,
        get_numeric_input::<u16>()?,
        get_numeric_input::<u16>()?,
    ];

    if user_input
        .iter()
        .any(|x| *x < MIN.into() || *x > MAX.into())
    {
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
            .fold(user_input[0], |acc, x| Lcm::lcm(acc, *x))
    );

    Ok(())
}
