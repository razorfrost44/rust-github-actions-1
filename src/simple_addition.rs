pub fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        // Arrange
        let a = 5;
        let b = 10;
        let expected_sum = 15;
        // Act
        let result = add_two(a, b);
        // Assert
        assert_eq!(
            result, expected_sum,
            "Expected {} + {} to equal {}",
            a, b, expected_sum
        );
    }
}
