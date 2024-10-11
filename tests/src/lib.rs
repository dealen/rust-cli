#[cfg(test)]
mod tests {

    use cli_calc::calc::Calc;

    #[test]
    fn can_calculate_simple_addition() {
        // arrange
        let input = "2+2";

        // act
        let result = Calc::calculate(input);

        // assert        
        assert_eq!(result, 4.0);
    }
    
    #[test]
    fn can_calculate_simple_substraction() {
        // arrange
        let input = "4-2";

        // act
        let result = Calc::calculate(input);

        // assert        
        assert_eq!(result, 2.0);
    }
    
    #[test]
    fn can_calculate_simple_multiplication() {
        // arrange
        let input = "3*3";

        // act
        let result = Calc::calculate(input);

        // assert        
        assert_eq!(result, 9.0);
    }
    
    #[test]
    fn can_calculate_simple_division() {
        // arrange
        let input = "12/3";

        // act
        let result = Calc::calculate(input);

        // assert        
        assert_eq!(result, 4.0);
    }
    
    #[test]
    fn can_calculate_more_complex_problem() {
        // arrange
        let input = "12/3-2*2+4";

        // act
        let result = Calc::calculate(input);

        // assert        
        assert_eq!(result, 4.0);
    }
    
    #[test]
    fn can_calculate_more_complex_problem_with_brakcets() {
        // arrange
        let input = "(9*7-4*3*4)/2";

        // act
        let result = Calc::calculate(input);

        // assert        
        assert_eq!(result, 7.5);
    }
    
    #[test]
    fn can_calculate_more_complex_problem_with_brakcets_and_power() {
        // arrange
        let input = "(6-2)*2^2";

        // act
        let result = Calc::calculate(input);

        // assert        
        assert_eq!(result, 16.0);
    }
}
