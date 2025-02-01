fn count_vowel(sentence: &str) -> i32 {
    let mut vowel_count = 0;
    for each_char in sentence.chars(){
        // Implement your logic here
        todo!()
        // End of your implementation zone
    }
    vowel_count
}

// Implement your is_prime function here


////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests{
    use crate::{count_vowel, is_prime};

    #[test]
    fn test_is_vowel() {
        assert_eq!(count_vowel("Hello World"), 3);
        assert_eq!(count_vowel("NTFS"), 0);
    }

    #[test]
    fn test_is_vowel2() {
        assert_eq!(count_vowel(""), 0);
        assert_eq!(count_vowel("EEIIOOUU"), 8);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
    }

    #[test]
    fn test_is_prime2() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
    }
}

fn main(){}