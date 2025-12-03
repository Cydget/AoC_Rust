use std::iter::zip;

use itertools::Itertools;

use crate::parse_input;
use crate::solvers::Aoc;
use crate::file_utils;

pub struct solution{
    part_1_solution:i64,
    part_2_solution:i64
}


impl solution{
    pub fn new()->Self{
        Self{part_1_solution:0,part_2_solution:0}
    }
    pub fn solve_part_1_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_1(&input)?;
        assert_eq!(self.part_1_solution,1227775554);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_2(&input)?;
        //assert_eq!(self.part_2_solution,6);

        //let input= format!("{input}\nR1000");
        //self.solve_part_2(&input)?;
        //assert_eq!(self.part_2_solution,16);

        Ok(())
    }

    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();
        assert_ne!(self.part_1_solution,1669563221);
        assert_ne!(self.part_1_solution,19562842152);

        assert_eq!(self.part_1_solution,995);
        assert_eq!(self.part_2_solution,5847);
    }

}

pub fn is_even(value:u128)->bool{
    if value&0b1==0{
        true
    }
    else{
        false
    }
}
pub fn is_double_num(input:u128)->bool{
    let string_num = format!("{input}");
    if input==2 || !is_even(string_num.len() as u128){
        return false
    }
    let mid_point = (string_num.len())/2;
    let (a,b )= string_num.split_at(mid_point);
    //println!("Num{input}.{a},{b}");
    if a==b{
        return true
    }

    false
}
pub fn is_double_num_old(input:u32)->bool{
    let number_digits = (1.0+input as f32).log10().ceil() as u32;
    //println!("Got{number_digits}");
    if input==2 || !is_even(number_digits as u128){
        return false;
    }

    //Number is only even
    let size_check = number_digits/2;
    let mut current_ending = input;
    let digits_array = (1..=number_digits).into_iter().map(|i|{
        let d: u32 = current_ending%(10);
        current_ending-=d;
        current_ending/=10;
        d
    }).rev();
    let digits_array:Vec<u32> = digits_array.collect_vec();
    let chunks: Vec<&[u32]> = digits_array.chunks_exact(size_check as usize).collect_vec();
    let first_chunk = chunks[0];
    let second_chunk = chunks[1];
    //println!("C1:{:?}",first_chunk);
    //println!("C2:{:?}",second_chunk);
    let z = zip(first_chunk.into_iter(),second_chunk.into_iter()).map(|x|{
        x.0==x.1
    }).all(|f|{f});
    
    if z{
        //println!("C1:{:?}",first_chunk);
        //println!("C2:{:?}",second_chunk);

        println!("Found pair{input}. Digits {number_digits},:{:?}",digits_array);
        true
    }
    else{
        false
    }
}

pub fn row_operation_part_1(input:&str)->Option<u128>{
    //This function should take a row. Convert it to a start and end
    if let Some((start,end)) = input.split_once("-"){
        let start_num = start.parse::<u128>().ok()?;
        let end_num = end.parse::<u128>().ok()?;
        let gap_size: u128 = end_num - start_num;
        let digits_count_max = (end_num as f32).log10().ceil() as i32;

        println!("Start: {start_num}. End: {end_num}. Gap Size:{gap_size}. Digit_count{digits_count_max}");
        let result:u128 = (start_num..=end_num).into_iter().filter_map(|test_number: u128|{
            let is_double = is_double_num(test_number ) ;
            match is_double{
                true=>Some(test_number),
                false=>None,
            }
        }).sum();
        //println!("Result is {}",result);
        return Some(result);
    }
    Some(0)
}

pub fn row_operation_part_2(input:&str){

}


impl Aoc for solution{
    fn solve_day(&mut self) -> Result<(), std::io::Error>{
        let input = self.get_input()?;
        let _ = self.solve_part_1(&input);
        let _ = self.solve_part_2(&input);
        Ok(())
    }





    fn solve_part_1(&mut self,input:&str)->Result<(),std::io::Error>{

        println!("Input is:\n{}",input);

        let mut counter: i32 = 50;

        let result: u128 = input.split(",")
                            .into_iter()
                            .filter(|f| f.len()>=1)
                            .filter_map(|row|{
                                row_operation_part_1(row)
                            })
                            .sum();

        println!("The solution for part 1 is:R {}",result);

        self.part_1_solution = result as i64;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{

        let result: i32 = input.split(",")
                            .into_iter()
                            .filter(|f| f.len()>=1)
                            .filter_map(|row|{
                                //println!("Rotate {rotates}, Was At:{last_counter}, Now at:{counter}. Add Amount {passes_0}");
                                Some(0)
                            })
                            .sum();


        self.part_2_solution = result as i64;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }


    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{2}
    fn get_year()->i32{2025}
}
