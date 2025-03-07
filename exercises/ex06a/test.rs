/// DO NOT EDIT THIS FILE! ///
#[cfg(test)]
mod tests{
    use crate::NumberCollection;

    #[test]
    fn test_all_ok() {
        let mut nc = NumberCollection {
            num_list: vec![
                "1.0".to_string(), 
                "2.0".to_string(), 
                "3.0".to_string(), 
                "4.0".to_string(), 
                "5.0".to_string(), 
                "6.0".to_string(), 
                "7.0".to_string(), 
                "8.0".to_string(), 
            ]
        };
        nc.add_num_string("9.0".to_string());
        nc.add_num_string("10.0".to_string());
        assert_eq!(nc.get_avg(), Ok(5.5));

        let nc = NumberCollection {
            num_list: vec![
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string()
            ]
        };
        assert_eq!(nc.get_avg(), Ok(0.0));
    }

    #[test]
    fn test_cannot_parse() {
        let nc = NumberCollection {
            num_list: vec![
                "1a".to_string(), 
                "2.0".to_string(), 
                "3.0".to_string(), 
                "4.0".to_string(), 
                "5.0".to_string(), 
                "6.0".to_string(), 
                "7.0".to_string(), 
                "8.0".to_string(), 
                "9.0".to_string(), 
                "10.0".to_string()
            ]
        };
        assert!(nc.get_avg().is_err());

        let nc = NumberCollection {
            num_list: vec![
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "0".to_string(), 
                "asld0".to_string()
            ]
        };
        assert!(nc.get_avg().is_err());
    }

}