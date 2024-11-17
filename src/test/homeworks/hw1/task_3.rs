//! Homework 1 task runners

#[test]
#[cfg(any(feature = "hw1-test", feature = "hw1-test-3"))]
fn hw1_task_3() {
    use crate::hw1::task_3::Solution;

    assert_eq!(Solution::main([10, 20, 30]), String::from("60"));
    assert_eq!(
        Solution::main([9, 10, 20]),
        String::from("Invalid input data!")
    );
}
