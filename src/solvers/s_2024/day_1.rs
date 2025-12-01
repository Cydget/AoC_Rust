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
    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_2(&input)?;
        Ok(())
    }

}
impl Aoc for solution{
    fn solve_day(&mut self) -> Result<(), std::io::Error>{
        let input = self.get_input()?;
        let _ = self.solve_part_1(&input);
        let _ = self.solve_part_2(&input);
        Ok(())
    }
    fn solve_part_1(&mut self,input:&str)->Result<(),std::io::Error>{
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }
    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{

        let input_line_count = parse_input::get_string_line_count(&input);    
        let input = parse_input::parse_basic_numbers::<i64>(&input, input_line_count);
        println!("The input is {:?}",input);
        let mut col_a: Vec<i64> = Vec::with_capacity(input.len());
        let mut col_b: Vec<i64> = Vec::with_capacity(input.len());
        input.into_iter().for_each(|f|{
            col_a.push(f[0]);
            col_b.push(f[1]);
        });
        col_a.sort();
        col_b.sort();
        let result:i64 = zip(col_a.iter(), col_b.iter()).into_iter().map(|f| (f.0-f.1).abs()).sum();
        println!("we got a result{}",result);
        self.part_2_solution = result;


        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }
    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{1}
    fn get_year()->i32{2024}
}
