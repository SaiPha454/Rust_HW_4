
fn main() {

    let star = 3; //change as you like

    let mut  arrow = String::new();
    let mut index = 0;

    make_arrow1(star, &mut index, &mut arrow);
    print!("{}", arrow);
}

fn make_arrow1(star_num : i32, index : &mut i32, arrow : &mut String) {

    *index += 1;

   if *index < star_num*2 {

        let mut star_index = 0;

        if *index <= star_num {

            generate_star(*index, &mut star_index, arrow);
        }else {
            let rev_index = (star_num*2) - *index;
            generate_star(rev_index, &mut star_index, arrow);
        }

        arrow.push_str("\n");

        make_arrow1(star_num, index, arrow);
   }

}
fn generate_star(star_num : i32, index : &mut i32, arrow : &mut String) {
    
    *index += 1;
    
    if *index <= star_num {
        arrow.push_str("*");
         generate_star(star_num, index, arrow);
    }
}


#[cfg(test)]
mod tests {
    use crate::{make_arrow1, generate_star};

    #[test]
    fn test_make_arrow1_recur_3() {
        
        let star = 3;
        let mut  arrow = String::new();
        let mut index = 0;

        let expected = "*\n**\n***\n**\n*\n";
        make_arrow1(star, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow1_recur_1() {
        
        let star = 1;
        let mut  arrow = String::new();
        let mut index = 0;

        let expected = "*\n";
        make_arrow1(star, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow1_generate_star_0() {
        
        let star_num = 0;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = "";

        generate_star(star_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow1_generate_star_2() {
        
        let star_num = 2;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = "**";

        generate_star(star_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

}