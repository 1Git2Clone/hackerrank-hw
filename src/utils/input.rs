use std::{fmt::Debug, io::Write, str::FromStr};

use crate::utils::{gcd::Gcd, lcm::Lcm, Error};

pub fn get_numeric_input<T>() -> Result<T, Error>
where
    T: FromStr + Debug + Gcd + Lcm,
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
