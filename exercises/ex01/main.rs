// Implement calc_and_display here

////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests{
    use crate::calc_and_display;

    #[test]
    fn test_mode_1() {
        assert_eq!(calc_and_display("Jane", "Doe", 27.4, 1), "Your name is Jane Doe".to_string());
        assert_eq!(calc_and_display("Turbo", "Granny", 405.95, 1), "Your name is Turbo Granny".to_string());
    }

    #[test]
    fn test_mode_2() {
        assert_eq!(calc_and_display("Jane", "Doe", 27.4, 2), "Your age is 27".to_string());
        assert_eq!(calc_and_display("Turbo", "Granny", 405.95, 2), "Your age is 405".to_string());
    }

    #[test]
    fn test_mode_3() {
        assert_eq!(calc_and_display("Jane", "Doe", 27.4, 3), "The sum of length and age is 34".to_string());
        assert_eq!(calc_and_display("Turbo", "Granny", 405.95, 3), "The sum of length and age is 416".to_string());
    }
}

fn main(){}