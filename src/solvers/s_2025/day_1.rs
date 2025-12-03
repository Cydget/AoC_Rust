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
        assert_eq!(self.part_2_solution,6);

        let input= format!("{input}\nR1000");
        self.solve_part_2(&input)?;
        assert_eq!(self.part_2_solution,16);

        Ok(())
    }

    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();
        assert_eq!(self.part_1_solution,995);
        assert_eq!(self.part_2_solution,5847);
    }

}


pub fn row_operation_part_1(input:&str)->i32{
    if input.starts_with("L"){
        let val = input.split_at(1).1.parse::<i32>().unwrap();
        let val = -val;
        val
    }
    else{
        let val = input.split_at(1).1.parse::<i32>().unwrap();
        val
    }
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

        //println!("Input is:\n{}",input);

        let mut counter: i32 = 50;

        let result: i32 = input.split("\n")
                            .into_iter()
                            .filter(|f| f.len()>=1)
                            .filter_map(|row|{
                                let a = row_operation_part_1(&row);
                                counter+=100+a;
                                counter= counter%100;
                                //println!("Row was {} got val {} dial at{}",&row,a,counter);

                                if counter == 0 { 
                                    Some(1)
                                }else{None}
                            })
                            .sum();


        self.part_1_solution = result as i64;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{

        let mut counter: i32 = 50;
        let mut last_direction_was_right:bool=false;
        let result: i32 = input.split("\n")
                            .into_iter()
                            .filter(|f| f.len()>=1)
                            .filter_map(|row|{
                                let mut rotates = row_operation_part_1(&row);
                                let mut passes_0=0;
                                let last_counter = counter;
                                if rotates>0{
                                    //We are rotating right
                                    let full_rotations = rotates/100;
                                    let rest_rotation = rotates - 100*full_rotations;
                                    passes_0+=full_rotations;
                                    counter+=rest_rotation;
                                    if counter==0{
                                        passes_0+=1;
                                    }
                                    else if counter>=100{
                                        counter-=100;
                                        if !(last_counter==0){
                                            passes_0+=1;
                                        }
                                    }
                                    last_direction_was_right=true;
                                }
                                else if rotates<0{
                                    let full_rotations = rotates/100;
                                    passes_0+= full_rotations.abs();
                                    let rest_rotation = rotates - 100*full_rotations;
                                    counter+=rest_rotation;
                                    if counter==0{
                                        passes_0+=1;
                                    }
                                    else if counter<0{
                                        counter+=100;
                                        //println!("The last counter is{last_counter},{}",!(last_counter==0));
                                        if !(last_counter==0){
                                            passes_0+=1;
                                        }
                                    }
                                    last_direction_was_right=false;
                                }
                                //println!("Rotate {rotates}, Was At:{last_counter}, Now at:{counter}. Add Amount {passes_0}");
                                Some(passes_0)
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
