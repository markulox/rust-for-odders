// Implement trait `GradeProcessor` here

// Implement struct `Student` here

// Implement struct `Classroom` here

////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_no_grades() {
        let s = Student{ grades: vec![] };
        assert_eq!(s.process_grades(), Err("No grades found".into()));
    }

    #[test]
    fn test_student_with_grades(){
        let s = Student{ grades: vec![0] };
        assert_eq!(s.process_grades(), Ok(Some(0.0)));
        let s = Student{ grades: vec![1,2,9] };
        assert_eq!(s.process_grades(), Ok(Some(4.0)));
        let s = Student{ grades: vec![12,10,11,9] };
        assert_eq!(s.process_grades(), Ok(Some(10.5)));
    }

    #[test]
    fn test_classroom_with_students() {
        let c = Classroom {
            students: vec![
                Student{ grades : vec![1,2,9]}, // 4
                Student{ grades : vec![12,11,10,9]}, // 10.5
                Student{ grades : vec![15,11,9]} // 11.66666666667
            ]
        };
        assert_eq!(c.process_grades(), Ok(Some(8.722222222222221)))
    }

    #[test]
    fn test_classroom_with_some_available_grades_student(){
        let c = Classroom {
            students: vec![
                Student{ grades : vec![1,2,9]}, // 4
                Student{ grades : vec![12,11,10,9]}, // 10.5
                Student{ grades : vec![]} // 11.66666666667
            ]
        };
        assert_eq!(c.process_grades(), Ok(Some(7.25)))
    }

    #[test]
    fn test_classroom_with_no_one_has_grade_and_no_student() {
        let c = Classroom {
            students: vec![
                Student{ grades: vec![] },
            ]
        };
        assert_eq!(c.process_grades(), Err("No valid grades found".into()));
        let c = Classroom {
            students: vec![]
        };
        assert_eq!(c.process_grades(), Err("No students in classroom".into()));
    }
}

fn main() {}
