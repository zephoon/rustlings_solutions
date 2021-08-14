// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

// I AM NOT DONE
fn my_function(v: i32) -> i32 {
    return v;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(my_function(3), 3);
    }
}
