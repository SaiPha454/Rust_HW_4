fn main() {

    let mut fah_vec = vec![10,100,200];
    let mut  fah_index: usize = 0;
    let mut cel_vec: Vec<f32> =  Vec::new();

    let result = fahr_to_cel_v_recursion(&mut fah_vec, &mut fah_index, &mut cel_vec);
    
    println!("Fahrenheit : {:?}", fah_vec);
    println!("Celcius : {:?}", result);

}

fn fahr_to_cel_v_recursion(fahs : &mut Vec<i32>, index : &mut usize , cels: &mut Vec<f32>) -> Vec<f32> {

    if fahs.len() == 0  {
        return cels.to_vec();
    }

    let fah = fahs[*index].to_string().parse::<f32>().unwrap();
    let mut cel = (5.0/ 9.0) * (fah - 32.0);
    
    cel = (cel * 100.0).round()/100.0 ; //two decimal precision
    cels.push(cel);

    *index += 1;

    if *index < fahs.len() {
        
        fahr_to_cel_v_recursion(fahs, index, cels);
    }

    cels.to_vec()
}


#[cfg(test)]
mod tests {
    use crate::fahr_to_cel_v_recursion;

    #[test]
    fn test_fahr_to_cel_v_recursion() {
        let mut input: Vec<i32> = vec![10, 100, 200];
        let mut cels: Vec<f32> = Vec::new();
        let expected: Vec<f32> = vec![-12.22,37.78 ,93.33];
        let mut index : usize = 0;

        let result = fahr_to_cel_v_recursion(&mut input, &mut index, &mut cels);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_fahr_to_cel_v_recursion_with_empty_vec() {
        let mut input: Vec<i32> = vec![];
        let mut cels: Vec<f32> = Vec::new();
        let expected: Vec<f32> = vec![];
        let mut index : usize = 0;

        let result = fahr_to_cel_v_recursion(&mut input, &mut index, &mut cels);
        assert_eq!(expected, result);
    }
}