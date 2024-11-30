use crate::utils::{
    gcd::ExtendedGcd,
    input::{get_numeric_input, invalid_input, no_solution},
    Error,
};

const MIN: i32 = 1;
const MAX: i32 = 100;

#[cfg_attr(doc, katexit::katexit)]
/// # HOMEWORK 3 | TASK 1
///
/// ## Input
///
/// 1. Three numbers (`a`, `b` and `c`).
///
/// ### Constaints
///
/// 2. The input should be in the inclusive range of: `[1;100]`.
///
/// ### Output
///
/// The values of `x` and `y` separated by new lines based on the Bezout's identity formula[1]:
///
/// $$
/// (a \times{x} + b \times{y}) \bmod{c} = 0
/// $$
///
///
/// [1]: https://en.wikipedia.org/wiki/B%C3%A9zout's_identity#Structure_of_solutions
///
/// ### Samples
///
/// #### Input Format
///
/// ```txt
/// 30
/// 18
/// 6
/// ```
///
/// #### Output Format
///
/// ```txt
/// -1
/// 2
/// ```
///
/// #### Input Format
///
/// ```txt
/// 30
/// 18
/// 4
/// ```
///
/// #### Output Format
///
/// ```txt
/// No solution!
/// ```
///
/// #### Explanation
///
/// (30, 18) = 6 => 6 % 4 != 0
///
/// #### Input Format
///
/// ```txt
/// 101
/// 25
/// 9
/// ```
///
/// #### Output Format
///
/// ```txt
/// Invalid input data!
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
/// If there's no solution, return:
///
/// ```rust
/// String::from("No solution!");
/// ```
///
/// ## Test cases
///
/// ```rust
/// use hackerrank::hw3::task_1::Solution;
///
/// assert_eq!(
///     Solution::main(30, 18, 6),
///     String::from("-1\n2")
/// );
/// assert_eq!(
///     Solution::main(30, 18, 4),
///     String::from("No solution!")
/// );
/// assert_eq!(
///     Solution::main(101, 25, 9),
///     String::from("Invalid input data!")
/// );
/// assert_eq!(
///     Solution::main(100, 50, 100),
///     String::from("0\n2")
/// );
/// assert_eq!(
///     Solution::main(99, 33, 66),
///     String::from("0\n2")
/// );
/// ```
pub struct Solution;

impl Solution {
    pub fn main(a: i32, b: i32, c: i32) -> String {
        if [a, b, c].iter().any(|item| !(MIN..=MAX).contains(item)) {
            return invalid_input();
        }
        let ((mut x, mut y), d) = a.extended_gcd(b);

        if c % d != 0 {
            return no_solution();
        }

        let scale_to_c = c / d;
        x *= scale_to_c;
        y *= scale_to_c;

        format!("{}\n{}", x, y)
    }
}

pub fn main() -> Result<(), Error> {
    let a = get_numeric_input::<i32>()?;
    let b = get_numeric_input::<i32>()?;
    let c = get_numeric_input::<i32>()?;

    println!("{}", Solution::main(a, b, c));

    Ok(())
}
