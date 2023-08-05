
fn main() {

    let star = 5;
    let arrow = make_arrow1(&star);

    print!("{}", arrow);
}

fn make_arrow1(star : &i32)-> String {
    let mut arrows = String::new();

    for i in 1..(star*2) {
        if i <=  *star {
            for _j in 0..i {
                arrows.push_str("*");
            }
        }else {
            for _j in 0..(star*2 -i) {
                arrows.push_str("*");
            }
        }
        arrows.push_str("\n");
    }
    return arrows;
}


#[cfg(test)]
mod tests {
    use crate::make_arrow1;

    #[test]
    fn test_make_arrow1_3() {
        
        let star = 3;
        let expected = "*\n**\n***\n**\n*\n";
        let arrow = make_arrow1(&star);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow1_1() {
        
        let star = 1;
        let expected = "*\n";
        let arrow = make_arrow1(&star);

        assert_eq!(expected, arrow);
    }
}