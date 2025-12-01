
use std::io::Error;
pub mod s_2025;
pub mod s_2024;
pub mod s_2023;


pub trait Aoc{
    fn solve_day(   &mut self)->Result<(),Error>;
    fn solve_part_1(&mut self, input:&str)->Result<(),Error>;
    fn solve_part_2(&mut self, input:&str)->Result<(),Error>;
    fn get_input(&mut self)->Result<String,Error>;
    fn get_day()->i32;
    fn get_year()->i32;
}
