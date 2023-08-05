fn main() {
    let given_score_list = vec![-10,10,0,100,50, 102];
    let mut grade_list: Vec<&str> = Vec::new();
    let mut index: usize = 0;

    make_grades_recursion(& given_score_list, &mut index, &mut grade_list);

    println!("{:?}", grade_list);
}

fn make_grades_recursion(score_list : &Vec<i32>, index: &mut usize,grade_list: &mut Vec<&str>) {

    if score_list.len() <= 0 {
        return;
    }

    let s = score_list[*index];

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

    *index += 1;

    if *index < score_list.len() {
        make_grades_recursion(score_list, index, grade_list);
    }
}



#[cfg(test)]
mod tests {
    use crate::make_grades_recursion;

    #[test]
    fn test_make_grades_recursion() {
        let test_list = vec![-10,10,50,65,75,92,100,110];
        let mut index : usize = 0;
        let mut grade_list: Vec<&str> = vec![];

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

        make_grades_recursion(&test_list,&mut index, &mut grade_list );

        assert_eq!(expected_list, grade_list);
    }

    #[test]
    fn test_make_grades_recursion_with_empty_score() {
        let test_list = vec![];
        let mut index: usize = 0;
        let mut grade_list = vec![];
        let expected_list: Vec<String> = vec![];

        make_grades_recursion(&test_list, &mut index, &mut grade_list);

        assert_eq!(expected_list, grade_list);
    }
}

