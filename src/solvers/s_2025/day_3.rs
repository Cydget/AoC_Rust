use std::iter::zip;
use std::ops::Index;

use itertools::Itertools;

use crate::parse_input;
use crate::solvers::Aoc;
use crate::file_utils;

pub struct solution{
    part_1_solution:u128,
    part_2_solution:u128
}


impl solution{
    pub fn new()->Self{
        Self{part_1_solution:0,part_2_solution:0}
    }
    pub fn solve_part_1_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_1(&input)?;
        assert_eq!(self.part_1_solution,357);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_2(&input)?;
        assert_eq!(self.part_2_solution,3121910778619);

        //let input= format!("{input}\nR1000");
        //self.solve_part_2(&input)?;
        //assert_eq!(self.part_2_solution,16);

        Ok(())
    }

    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();

        assert_eq!(self.part_1_solution,17155);
        assert_eq!(self.part_2_solution,169685670469164);
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



pub fn row_operation_part_1(input:&str)->Option<u32>{
    //This function should take a row
    //It returns the u32 value made from the max two digits
    //It should return 
    
    //First digit can not be the last digit
    let first_digit = input.chars().take(input.len()-1).sorted().rev().nth(0).unwrap();
    let first_digit_index = input.find(first_digit).expect("We know value exists");
    let second_digit = input.chars().skip(first_digit_index+1).sorted().rev().nth(0).unwrap();


    //println!("String  Got input {input}, First {first_digit}. Next {second_digit}");
    let value=format!("{first_digit}{second_digit}").parse::<u32>().ok();
    value
}

pub fn row_operation_part_1_fails(input:&str)->Option<u32>{
    //This function should take a row
    //It returns the u32 value made from the max two digits
    //It should return 
    let digits = input.chars().sorted().rev().take(2).map(|f|{
        let index = input.find(f).expect("We know value exists");
        index
    }).sorted()
    .map(|index| {
        let digit = input.chars().nth(index).expect("We know this index exists");
        digit
    }).collect::<String>();

    println!("String  Got input {input}, val {digits}");
    let value=digits.parse::<u32>().ok();
    value
}


pub fn row_operation_part_2(input:&str)->Option<u128>{
    //This function should take a row
    //It returns the u32 value made from by enableing 12 digits
    
    let mut mut_string = String::from("");

    let mut start_position = 0;//This is saying the digit start at index 0 
    let mut end_position = input.len() - 11;//This is saying the digit start at index 0 
    let input_vec = input.chars().filter_map(|x|{
        x.to_digit(10)
    }).collect_vec();

    'have_12:loop{
        if mut_string.len()>=12{ break 'have_12;}
        let valid_digit_positions = &input_vec[start_position..end_position];
        let (d_index,digit) = valid_digit_positions.iter().enumerate().sorted_by(|x,y|{
            Ord::cmp(y.1,x.1)
            //Consider case of to digits that are equal. We must return the first index not the second
            //because we want the highest value, we can simply reverse order. We can also use nth_back and keep order normal
        }).nth(0).expect("List should always be long enough");
        //println!("Searching{:?}, Max val {} at {}",valid_digit_positions,digit,d_index);

        mut_string.push_str(&digit.to_string());
        //d_index is given from enumerate which starts at 0 not at last start pos. Because of that we must add
        start_position += d_index+1;
        end_position+=1;

    }
    //mut_string.push(second_digit);

    //println!("String  Got input {input},String '{mut_string}'");
    let value=mut_string.parse::<u128>().ok();
    //println!("Final Value{:?}",value);
    value

}


impl Aoc for solution{
    fn solve_day(&mut self) -> Result<(), std::io::Error>{
        let input = self.get_input()?;
        let _ = self.solve_part_1(&input);
        let _ = self.solve_part_2(&input);
        Ok(())
    }





    fn solve_part_1(&mut self,input:&str)->Result<(),std::io::Error>{

        //println!("Input is:\n{}",input);

        let result: u32 = input
                            .split("\n")
                            .into_iter()
                            .filter(|f| f.len()>=1)
                            .filter_map(|row|{
                                row_operation_part_1(row)
                            })
                            .sum();

        self.part_1_solution = result as u128;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{

        let result: u128 = input
                            .split("\n")
                            .into_iter()
                            .filter(|f| f.len()>=1)
                            .filter_map(|row|{
                                row_operation_part_2(row)
                            })
                            .sum();
        self.part_2_solution = result as u128;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }


    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{3}
    fn get_year()->i32{2025}
}
