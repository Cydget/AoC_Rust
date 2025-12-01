
use crate::{file_utils, parse_input};

pub fn solve_day()->i64{
    let input = file_utils::read_input_file(2023, 1).expect("Should open file");
/*
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
*/

        //let input_vec = parse_input::parse_basic_string::<String>(&test_string,3);
        let input_line_count = parse_input::get_string_line_count(&input);

        let input_vec = parse_input::parse_basic_string::<String>(&input,input_line_count);
        let mut total_sum = 0;
        for a in input_vec{
            let r = parse_input::convert_text_string_to_list(&a);
            let mut row_sum=0;
            match r.len(){
                0=>{println!("We got 0 {:?} {:?}",r,a);},
                1=>{
                    //row_sum = r[0];
                    row_sum = r[0]*10+r[r.len()-1];
                    println!("We got 1 {:?} {:?}",r,a);
                },
                _=>{row_sum = r[0]*10+r[r.len()-1];}
            }
            total_sum+=row_sum;
            println!("{}, {:?},{} {}",a,r,row_sum,total_sum);

        }
        println!("2023 Day1. Final Result is:{}",total_sum);
        return total_sum;
        //let result = parse_input::parse_basic_string::<String>(&input, input_line_count);
        //println!("Got strings:{:?}",result)
    0
}