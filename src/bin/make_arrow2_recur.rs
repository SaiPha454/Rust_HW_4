
fn main() {

    let star = 4; //change as you like

    let mut  arrow = String::new();
    let mut index = 0;

    make_arrow2(star, &mut index, &mut arrow);
    print!("{}", arrow);
}

fn make_arrow2(star_num : i32, index : &mut i32, arrow : &mut String) {

    *index += 1;

   if *index < star_num*2 {

        let mut star_index = 0;
        let mut space_index = *index;
        if *index <= star_num {

            generate_space( star_num , &mut space_index, arrow);
            generate_star(*index, &mut star_index, arrow);

        }
        else {

            let mut rev_index = star_num;
            let mut star_rev_index = *index;

            generate_space( *index ,&mut rev_index , arrow);
            generate_star(star_num * 2, &mut star_rev_index, arrow);
        }

        arrow.push_str("\n");
        make_arrow2(star_num, index, arrow);
   }

}

fn generate_star(star_num : i32, index : &mut i32, arrow : &mut String) {

    *index += 1;
    
    if *index <= star_num {
        arrow.push_str("*");
         generate_star(star_num, index, arrow);
    }
}

fn generate_space(star_num : i32, index : &mut i32, arrow : &mut String) {
    
    *index+= 1 ;
    
    if *index <= star_num {
        arrow.push(' ');
         generate_space(star_num, index, arrow);
    }
}


#[cfg(test)]
mod tests {
    use crate::{make_arrow2, generate_star, generate_space};

    #[test]
    fn test_make_arrow2_recur_3() {
        
        let star_num = 3;
        let mut index = 0;
        let mut arrow = String::new();
        let expected = "  *\n **\n***\n **\n  *\n";
        make_arrow2(star_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow2_recur_1() {
        
        let star_num = 1;
        let mut index = 0;
        let mut arrow = String::new();
        let expected = "*\n";
        make_arrow2(star_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow2_generate_star_0() {
        
        let star_num = 0;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = "";

        generate_star(star_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow2_generate_star_2() {
        
        let star_num = 2;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = "**";

        generate_star(star_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }


    #[test]
    fn test_make_arrow2_generate_space_0() {
        
        let star_num = 0;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = "";

        generate_space(star_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow2_generate_space_2() {
        
        let space_num = 2;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = "  ";

        generate_space(space_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

    #[test]
    fn test_make_arrow2_generate_space_1() {
        
        let space_num = 1;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = " ";

        generate_space(space_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
    }

}




