fn main() {
    let star = 3;

    let arrow = make_arrow2(&star);

    print!("{}", arrow);

}

fn make_arrow2(star : &i32) -> String {

    let mut arrows = String::new();
    for i in 1..(star*2) {
        if i <= *star {
            for _j in 0..(star - i) {
                arrows.push_str(" ");
            }
            for _k in 0..i {
                arrows.push_str("*");
            }
        }else {
            for _j in 0..(i-star) {
                arrows.push_str(" ");
            }
            for _k in i..(star*2) {
                arrows.push_str("*");
            }
        }
        arrows.push_str("\n");
    }
    return arrows;
}



#[cfg(test)]
mod tests {
    use crate::make_arrow2;

    #[test]
    fn test_make_arrow2_3() {
        
        let star = 3;
        let expected = "  *\n **\n***\n **\n  *\n";
        let arrow = make_arrow2(&star);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow2_1() {
        
        let star = 1;
        let expected = "*\n";
        let arrow = make_arrow2(&star);

        assert_eq!(expected, arrow);
    }
}