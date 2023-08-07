
fn main() {
    let given_score_list = vec![10,0,100,50,-110,-5];

    let grades = make_grades(&given_score_list);

    println!("Score List : {:?}", given_score_list);
    println!("Grade List : {:?}", grades);

}

fn make_grades(score_list : &Vec<i32>)-> Vec<&str> {
    let mut grade_list: Vec<&str> = Vec::new();

    for s in score_list {
        let s = s.to_string().parse::<i32>().unwrap();
        if s >=0 && s <=100 {
            if s >=0 && s <=49 {
                grade_list.push("F");
            }else if s >=50 && s <=60 {
                grade_list.push("D");
            }else if s >=61 && s <= 70 {
                grade_list.push("C");
            }else if s >= 71 && s <= 80 {
                grade_list.push("B");
            }else if s >=81 && s <= 94 {
                grade_list.push("A");
            }else {
                grade_list.push("A+");
            }
        }else {
            grade_list.push("Invalid score");
        }
        
    }
    grade_list
}


#[cfg(test)]
mod tests {
    use crate::make_grades;

    #[test]
    fn test_make_grades_loop() {
        let test_list = vec![-10,10,50,65,75,92,100,110];
        let expected_list = vec![
            "Invalid score",
            "F",
            "D",
            "C",
            "B",
            "A",
            "A+",
            "Invalid score"
        ];

        let result = make_grades(&test_list);

        assert_eq!(expected_list, result);
    }

    #[test]
    fn test_make_grades_loop_with_empty_score() {
        let test_list = vec![];
        let expected_list: Vec<String> = vec![];

        let result = make_grades(&test_list);

        assert_eq!(expected_list, result);
    }
}