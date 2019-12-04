use crate::task::Task;

pub struct Task04A {}
pub struct Task04B {}

fn check_valid_number(num: i32) -> bool {
    false
}


impl Task for Task04A {
    fn run(&self) {
        let mut total_valid = 0;
        for num in 278384..824795 {
            let valid = check_valid_number(num);
            if valid {
                total_valid += 1;
            }
        }
    }
}

impl Task for Task04B {
    fn run(&self) {
        unimplemented!()
    }
}


#[test]
fn test_valid(){
    assert_eq!(check_valid_number(111111), true, "111111 should be valid");
    assert_eq!(check_valid_number(135679), true, "135679 should be valid");
    assert_eq!(check_valid_number(223450), false, "223450 should not be valid");
    assert_eq!(check_valid_number(123789), false, "123789 should not be valid");
}