// Implement your keep_pushing function here

// End of implementation zone

fn execute_me1() -> String {
    let mut my_button = "button".to_string();
    // Please fix the error in the code below
    keep_pushing(/*REPLACE ME*/);
    keep_pushing(/*REPLACE ME*/);
    keep_pushing(/*REPLACE ME*/);
    // End of fix zone
    my_button
}

////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests{
    use crate::execute_me1;

    #[test]
    fn test_execute_me1() {
        assert_eq!(execute_me1(), "button|Pushing|Pushing|Pushing".to_string());
    }

}

fn main(){}