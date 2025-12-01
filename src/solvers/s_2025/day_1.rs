use std::iter::zip;

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
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_2(&input)?;
        Ok(())
    }

}


pub fn row_operation_part_1(input:&str){

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

        println!("Input is:{}",input);

        let result: i32 = input.split("\n")
                            .into_iter()
                            .filter(|f| f.len()>=1)
                            .filter_map(|row|{
                                let a = row_operation_part_1(&row);
                                Some(0)
                            })
                            .sum();


        self.part_1_solution = result as i64;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{


        let result: i32 = input.split("\n")
                        .into_iter()
                        .filter(|f| f.len()>=1)
                        .filter_map(|row|{
                            let a = row_operation_part_2(&row);
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

    fn get_day()->i32{1}
    fn get_year()->i32{2025}
}
