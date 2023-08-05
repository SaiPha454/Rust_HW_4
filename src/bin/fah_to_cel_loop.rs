use std::error::Error;

fn main() {

    let fahs = vec![10,100,120];

    let result = fahr_to_cel_v(&fahs).unwrap();

    println!("Fahrenheit : {:?}", fahs);
    println!("Celcius : {:?}", result);

}


fn fahr_to_cel_v(fah_vec : &Vec<i32>) -> Result<Vec<f32>, Box<dyn Error>> {

    let mut cel_vec: Vec<f32> = Vec::new();

    for fah in fah_vec {
        let fah = fah.to_string().parse::<f32>().unwrap();
        let cel = (5.0/ 9.0) * (fah - 32.0);
        let cel = (cel*100.0).round()/ 100.0;

        cel_vec.push(cel);
    }

    Ok(cel_vec)

}



#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_fn_fahr_to_cel_v_loop() {
        let input_vec = vec![10, 50, 100];
        let expected_vec = vec![-12.22, 10.0, 37.78];

        let result = fahr_to_cel_v(&input_vec).unwrap();

        assert_eq!(expected_vec, result);
    }


    #[test]
    fn test_fn_fahr_to_cel_v_loop_with_empty_vector() {
        let input_vec: Vec<i32> = vec![];
        let expected_vec: Vec<f32> = vec![];

        let result = fahr_to_cel_v(&input_vec).unwrap();

        assert_eq!(expected_vec, result);
    }
    

}