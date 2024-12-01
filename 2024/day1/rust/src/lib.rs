pub mod errors;
pub mod parse;

pub fn find_distance(num1: i64, num2: i64) -> i64 {
    if num1 > num2 {
        return num1 - num2;
    }

    num2 - num1
}

pub fn find_simularity_score(list1: &Vec<i64>, list2: &Vec<i64>) -> i64 {
    let mut amount_same: i64;
    let mut score = 0;
    for num1 in list1 {
        amount_same = 0;
        for num2 in list2 {
            if num2 == num1 {
                amount_same += 1;
            }
        }
        score += amount_same * num1;
    }

    score
}
