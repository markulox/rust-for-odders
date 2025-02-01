// Implement sum_up_and_divide here


////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests{
    use crate::sum_up_and_divide;

    #[test]
    fn test_sumup() {
        assert_eq!(sum_up_and_divide(vec![1,2,3], 2), (6, true, 3.0));
        assert_eq!(sum_up_and_divide(vec![4,5,6,7], 10), (22, false, 2.2))
    }

    #[test]
    fn test_sumup2() {
        assert_eq!(sum_up_and_divide(vec![4], 1), (4, true, 4.0))
    }

    #[test]
    fn test_mode_3() {
        assert_eq!(sum_up_and_divide(vec![], 1), (0, true, 0.0))
    }
}

fn main(){}