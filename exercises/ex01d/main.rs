// Implement filter_out here

////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests{
    use crate::filter_out;

    #[test]
    fn test_sumup() {
        assert_eq!(filter_out(vec![1,2,3], 2), vec![1,3]);
    }

    #[test]
    fn test_sumup2() {
        assert_eq!(filter_out(vec![10,20,30,44,50,60,70,80,99,10], 0), vec![44,99]);
    }

    #[test]
    fn test_mode_3() {
        assert_eq!(filter_out(vec![], 3), vec![]);
    }
}

fn main(){}