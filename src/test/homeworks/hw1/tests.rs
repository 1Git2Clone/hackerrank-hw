//! Homework 1 task runners

use crate::utils::Error;

#[test]
#[cfg(any(feature = "hw1-test", feature = "hw1-test-1"))]
fn hw1_task_1() -> Result<(), Error> {
    use crate::hw1::task_1::Solution;

    assert_eq!(Solution::main(2, 8), String::from("1000"));
    assert_eq!(Solution::main(2, 34), String::from("100010"));
    assert_eq!(Solution::main(2, 1), String::from("1"));
    assert_eq!(Solution::main(8, 22), String::from("26"));
    assert_eq!(Solution::main(8, 34), String::from("42"));
    assert_eq!(Solution::main(16, 34), String::from("22"));
    assert_eq!(Solution::main(16, 12), String::from("C"));
    assert_eq!(Solution::main(17, 5), String::from("Invalid input data!"));

    Ok(())
}

#[test]
#[cfg(any(feature = "hw1-test", feature = "hw1-test-2"))]
fn hw1_task_2() -> Result<(), Error> {
    use crate::hw1::task_2::Solution;

    assert_eq!(Solution::main([24, 40, 64]), String::from("8"));
    assert_eq!(Solution::main([23, 25, 26]), String::from("1"));
    assert_eq!(
        Solution::main([5, 25, 60]),
        String::from("Invalid input data!")
    );
    assert_eq!(
        Solution::main([20, 20, 301]),
        String::from("Invalid input data!")
    );

    Ok(())
}

#[test]
#[cfg(any(feature = "hw1-test", feature = "hw1-test-3"))]
fn hw1_task_3() -> Result<(), Error> {
    use crate::hw1::task_3::Solution;

    assert_eq!(Solution::main([10, 20, 30]), String::from("60"));
    assert_eq!(
        Solution::main([9, 10, 20]),
        String::from("Invalid input data!")
    );

    Ok(())
}
