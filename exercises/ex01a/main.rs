fn adder(num1: i8, num2: i16) -> i16 {
    // Implement your logic here
    todo!()
    // End of logic implement section
}

fn adder2(another_number: f64) -> f64 {
    let adder_result = adder(3,6);
    // Implement your logic here
    todo!()
    // End of logic implement section
}


////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests{
    use crate::{adder, adder2};

    #[test]
    fn test_adder() {
        assert_eq!(adder(1, 2), 3);
        assert_eq!(adder(10, 12), 22);
    }

    #[test]
    fn test_adder2() {
        assert_eq!(adder2(3.5), 12.5);
        assert_eq!(adder2(6.43), 15.43);
    }
}

fn main(){}