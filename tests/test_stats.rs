#[cfg(test)]
mod tests {
    use yijia_ids706_mini_proj7::stats::{mean, median};

    #[test]
    fn test_mean() {
        // Test with a normal list of numbers
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&numbers), Some(3.0));

        // Test with an empty list
        let numbers: Vec<f64> = vec![];
        assert_eq!(mean(&numbers), None);

        // Test with a list of one element
        let numbers = vec![10.0];
        assert_eq!(mean(&numbers), Some(10.0));

        // Test with negative numbers
        let numbers = vec![-1.0, -2.0, -3.0];
        assert_eq!(mean(&numbers), Some(-2.0));
    }

    #[test]
    fn test_median() {
        // Test with a list of odd length
        let mut numbers = vec![1.0, 3.0, 2.0];
        assert_eq!(median(&mut numbers), Some(2.0));

        // Test with a list of even length
        let mut numbers = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(median(&mut numbers), Some(2.5));

        // Test with an empty list
        let mut numbers: Vec<f64> = vec![];
        assert_eq!(median(&mut numbers), None);

        // Test with one element
        let mut numbers = vec![10.0];
        assert_eq!(median(&mut numbers), Some(10.0));
    }
}
