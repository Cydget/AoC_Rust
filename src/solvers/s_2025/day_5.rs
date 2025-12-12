use core::num;
use std::iter::zip;
use std::ops::Index;
use std::result;

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
        assert_eq!(self.part_1_solution,3);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_2(&input)?;
        assert_eq!(self.part_2_solution,14);

        //let input= format!("{input}\nR1000");
        //self.solve_part_2(&input)?;
        //assert_eq!(self.part_2_solution,16);

        Ok(())
    }

    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();

        assert_eq!(self.part_1_solution,761);
        assert_eq!(self.part_2_solution,345755049374932);
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



pub fn reduce_input_part_2(mut start_vec:Vec<(u128,u128)>)->Vec<(u128,u128)>{

    let mut dyn_ranges:Vec<(u128,u128)> = Vec::new();
    start_vec.sort();
    let mut ranges_og = start_vec.into_iter();
    dyn_ranges.push(        ranges_og.nth(0).unwrap()  );//push first element into range

    ranges_og.for_each(|(lower_unchecked,upper_unchecked)|{
        //println!("Adding range:{lower_unchecked}-{upper_unchecked}. To {:?}",dyn_ranges);
        let mut range_is_new = true;
        let a = dyn_ranges.iter().map(|(lower_checked,upper_checked)|{
            let lower_value_in_range = lower_unchecked >= *lower_checked && lower_unchecked<= *upper_checked;
            let upper_value_in_range = upper_unchecked <= *upper_checked && upper_unchecked>= *lower_checked;

            match (lower_value_in_range,upper_value_in_range){
                (false,false)=>{(*lower_checked,*upper_checked)},//Both fall outside range, keep this range unchanged
                (false,true)=>{range_is_new=false; (lower_unchecked,*upper_checked)},//There was some overlap. Modify Range
                (true,false)=>{range_is_new=false; (*lower_checked,upper_unchecked)},//There was some overlap. Modify Range
                (true,true)=>{range_is_new=false; (*lower_checked,*upper_checked)},//Both are already contained inside range. Do not add to range
            }
        });
        dyn_ranges = a.collect();
        if range_is_new{
            dyn_ranges.push((lower_unchecked,upper_unchecked));
        }
        //println!("Result Range is {:?}",dyn_ranges);

    });
    dyn_ranges
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

        let lines =input.lines();
        let ranges = lines.clone().take_while(|x|{
            x.contains("-")
        }).filter_map(|f|{
            //println!("On Strnig {}",f);
            let mut split = f.split("-");
            //println!("Split {:?}",split);

            let num_high= split.nth(0).unwrap().parse::<u128>().unwrap();
            let num_low= split.nth(0).unwrap().parse::<u128>().unwrap();
            Some((num_high,num_low))
        });

        let valid_numbers = lines.filter_map(|x|{
            x.parse::<u128>().ok()
        }).filter_map(|f|{
            let in_a_range = ranges.clone().any(|(lower,upper)|{
                f>=lower && f <= upper
            });
            match in_a_range{
                true=>Some(f),
                false=>None,
            }
        }).collect_vec();
        self.part_1_solution = valid_numbers.iter().count() as u128;
        //println!("Valid numbers:{:?}, Count:{}",valid_numbers,self.part_1_solution);
        //println!("All ranges {:?}",ranges);
/* 
        let result: u32 = input
                            .split("\n")
                            .into_iter()
                            .filter(|f| f.len()>=1)
                            .filter_map(|row|{
                                row_operation_part_1(row)
                            })
                            .sum();
*/
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }





    //fn reduce_range(range:Vec<(u128,u128)>,level:u32){
        //let mut dyn_ranges:Vec<(u128,u128)> = Vec::new();
        

    //}


    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{

        let lines =input.lines();
        let mut ranges_og = lines.clone().take_while(|x|{
            x.contains("-")
        }).filter_map(|f|{
            let mut split = f.split("-");
            let mut num_high= split.nth(0).unwrap().parse::<u128>().unwrap();
            let mut num_low= split.nth(0).unwrap().parse::<u128>().unwrap();
            Some((num_high,num_low))
        });
        let total_count = ranges_og.clone().count();
        let mut result_vec:Vec<(u128, u128)> = ranges_og.collect();
        for _ in 0..=(total_count){
            result_vec = reduce_input_part_2(result_vec.clone());//We could implement early exist 
            //println!("Range Complete");
        }
        result_vec.sort();
        //println!("All Ranges {:?}",result_vec);
        let result:u128 = result_vec.iter().map(|(lower,upper)|{
            upper - lower + 1
        }).sum();



        self.part_2_solution = result as u128;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }


    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{5}
    fn get_year()->i32{2025}
}
