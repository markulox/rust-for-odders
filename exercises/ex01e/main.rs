fn taker(sentence: String) {
    println!("{}", sentence)
}

fn take_and_give(sentence: String) -> String {
    println!("{}", sentence);
    sentence
}

fn give() -> String {
    String::from("New string is coming up!")
}

fn execute_me1() -> String {
    let s1 = String::from("Ai Yaahh!!");
    // Please fix the error in the code below
    taker(s1);
    let s2 = s1.clone();
    // End of fix zone
    s2
}

fn execute_me2() -> String {
    // Please fix the error in the code below
    let s1 = String::from("Can I take a break?");
    /* REPLACE ME */ = take_and_give(s1);
    // End of fix zone
    s2
}

fn execute_me3() -> String {
    // Please fix the error in the code below
    /* REPLACE ME */ = give();
    // End of fix zone
    s1 = take_and_give(s1);
    s1
}


////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests{
    use crate::{execute_me1, execute_me2, execute_me3};

    #[test]
    fn test_execute_me1() {
        assert_eq!(execute_me1(), "Ai Yaahh!!".to_string());
    }

    #[test]
    fn test_execute_me2() {
        assert_eq!(execute_me2(), "Can I take a break?".to_string());
    }

    #[test]
    fn test_execute_me3() {
        assert_eq!(execute_me3(), "New string is coming up!".to_string());
    }
}

fn main(){}