
fn main() {

    let star = 3; //change as you like

    let mut  arrow = String::new();
    let mut index = 0;

    make_arrow2(star, &mut index, &mut arrow);
    print!("{}", arrow);
}

fn make_arrow2(star_num : i32, index : &mut i32, arrow : &mut String) {

    *index += 1;

   if *index < star_num*2 {

        let mut star_index = 0;
        let mut space_index = *index - 1;
        if *index <= star_num {

            generate_space( star_num , &mut space_index, arrow);
            generate_star(*index, &mut star_index, arrow);

        }
        else {

            let mut rev_index = star_num -1;
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
    arrow.push_str("*");
    
    if *index < star_num {
 
         generate_star(star_num, index, arrow);
    }
}

fn generate_space(star_num : i32, index : &mut i32, arrow : &mut String) {
    
    *index+= 1 ;
    if *index < star_num {
        arrow.push(' ');
         generate_space(star_num, index, arrow);
    }
}




