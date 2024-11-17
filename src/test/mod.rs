//! A runner for all homework tests.

#[cfg(test)]
#[cfg(any(
    feature = "all-tests",
    feature = "hw1-test",
    feature = "hw1-test-1",
    feature = "hw1-test-2",
    feature = "hw1-test-3"
))]
pub mod homeworks;
