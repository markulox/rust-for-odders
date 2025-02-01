// Implement enums here

// Implement counting function here

////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests {
    use crate::{put_in_cart, show_in_cart};

    #[test]
    fn test_print_apple(){
        let mut in_cart = Vec::new();
        put_in_cart(&mut in_cart, crate::Brand::Apple(crate::Apple::iPhone(12)));
        put_in_cart(&mut in_cart, crate::Brand::Apple(crate::Apple::iPhonePro(14)));
        put_in_cart(&mut in_cart, crate::Brand::Apple(crate::Apple::iPhoneProMax(16)));
        assert_eq!(show_in_cart(&in_cart), vec![
            "1. Apple iPhone 12".to_string(),
            "2. Apple iPhone 14 Pro".to_string(),
            "3. Apple iPhone 16 Pro Max".to_string()
        ])
    }

    #[test]
    fn test_print_samsung(){
        let mut in_cart = Vec::new();
        put_in_cart(&mut in_cart, crate::Brand::Samsung(crate::Samsung::Galaxy('S', 12)));
        put_in_cart(&mut in_cart, crate::Brand::Samsung(crate::Samsung::Galaxy('A', 35)));
        put_in_cart(&mut in_cart, crate::Brand::Samsung(crate::Samsung::Earbuds));
        assert_eq!(show_in_cart(&in_cart), vec![
            "1. Samsung Galaxy S 12".to_string(),
            "2. Samsung Galaxy A 35".to_string(),
            "3. Samsung Earbuds".to_string()
        ])
    }

    #[test]
    fn test_both() {
        let mut in_cart = Vec::new();
        put_in_cart(&mut in_cart, crate::Brand::Apple(crate::Apple::iPhone(12)));
        put_in_cart(&mut in_cart, crate::Brand::Samsung(crate::Samsung::Galaxy('S', 12)));
        put_in_cart(&mut in_cart, crate::Brand::Samsung(crate::Samsung::Galaxy('A', 35)));
        put_in_cart(&mut in_cart, crate::Brand::Apple(crate::Apple::iPhonePro(14)));
        put_in_cart(&mut in_cart, crate::Brand::Apple(crate::Apple::iPhoneProMax(16)));
        put_in_cart(&mut in_cart, crate::Brand::Samsung(crate::Samsung::Earbuds));
        assert_eq!(show_in_cart(&in_cart), vec![
            "1. Apple iPhone 12".to_string(),
            "2. Samsung Galaxy S 12".to_string(),
            "3. Samsung Galaxy A 35".to_string(),
            "4. Apple iPhone 14 Pro".to_string(),
            "5. Apple iPhone 16 Pro Max".to_string(),
            "6. Samsung Earbuds".to_string()
        ])
    }
}

fn main(){}