// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.

mod result {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(5, result::add(2,3));
    }
}
