#[test]
#[cfg(any(feature = "hw1-test", feature = "hw1-test-2"))]
fn hw1_task_2() {
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
}
