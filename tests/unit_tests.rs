// The code to be tested
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Unit test
#[cfg(test)]
mod tests {
    // Import the add function from the parent module
    use super::add;

    #[test]
    fn test_add_positive_numbers() {
        // Arrange: Set up the input values
        let a = 1;
        let b = 2;

        // Act: Call the function under test
        let result = add(a, b);

        // Assert: Check the result
        assert_eq!(result, 3);
    }

    #[test]
    fn test_add_negative_numbers() {
        // Arrange: Set up the input values
        let a = -5;
        let b = -3;

        // Act: Call the function under test
        let result = add(a, b);

        // Assert: Check the result
        assert_eq!(result, -8);
    }

    // Add more test cases as needed
}
