use std::{fmt::Debug, io::Write, str::FromStr};

use crate::utils::Error;

pub fn get_numeric_input<T>() -> Result<T, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut tmp = String::new();
    while tmp.trim().parse::<T>().is_err() {
        tmp.clear();
        if let Err(err) = std::io::stdin().read_line(&mut tmp) {
            return Err(err.into());
        }
        std::io::stdout().flush()?;
    }

    Ok(tmp.trim().parse::<T>().unwrap())
}

pub fn get_input() -> Result<String, Error> {
    let mut tmp = String::new();
    if let Err(err) = std::io::stdin().read_line(&mut tmp) {
        return Err(err.into());
    }
    std::io::stdout().flush()?;

    Ok(tmp.trim().into())
}

#[must_use = "This function returns the 'Invalid input data!' value used for asserting tests."]
pub fn invalid_input() -> String {
    String::from("Invalid input data!")
}

#[must_use = "This function returns the 'No solution!' value used for asserting tests."]
pub fn no_solution() -> String {
    String::from("No solution!")
}
