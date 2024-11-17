#[test]
#[cfg(any(feature = "hw1-test", feature = "hw1-test-1"))]
fn hw1_task_1() {
    use crate::hw1::task_1::Solution;

    assert_eq!(Solution::main(2, 8), String::from("1000"));
    assert_eq!(Solution::main(2, 34), String::from("100010"));
    assert_eq!(Solution::main(2, 1), String::from("1"));
    assert_eq!(Solution::main(8, 22), String::from("26"));
    assert_eq!(Solution::main(8, 34), String::from("42"));
    assert_eq!(Solution::main(16, 34), String::from("22"));
    assert_eq!(Solution::main(16, 12), String::from("C"));
    assert_eq!(Solution::main(17, 5), String::from("Invalid input data!"));
}
