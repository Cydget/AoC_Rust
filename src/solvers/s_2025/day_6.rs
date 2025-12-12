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
#[derive(Debug,Clone)]
enum day6op{
    ADD,
    MUL
}


impl solution{
    pub fn new()->Self{
        Self{part_1_solution:0,part_2_solution:0}
    }
    pub fn solve_part_1_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_1(&input)?;
        assert_eq!(self.part_1_solution,4277556);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_2(&input)?;
        assert_eq!(self.part_2_solution,3263827);
        Ok(())
    }

    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();

        assert_eq!(self.part_1_solution,4722948564882);
        assert_eq!(self.part_2_solution,9581313737063);
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



impl Aoc for solution{
    fn solve_day(&mut self) -> Result<(), std::io::Error>{
        let input = self.get_input()?;
        let _ = self.solve_part_1(&input);
        let _ = self.solve_part_2(&input);
        Ok(())
    }





    fn solve_part_1(&mut self,input:&str)->Result<(),std::io::Error>{

        //println!("Input is:\n{}",input);

        //Decided to take a less functional approach this day
        //2D indexing is kinda annoying with iterators.
        let lines_iter = input.lines();
        let last_line = lines_iter.clone().last();
        let line_count = lines_iter.clone().count();//maybe subtract one
    
        //println!("{}",line_count);

        let col_indexes = last_line.clone().unwrap().char_indices().tuple_windows().filter_map(|(cur,nxt)|{
            match nxt.1{
                '*' |'+' =>{Some(cur.0)},
                _=>{None}
            }
        });
    
        let column_count = col_indexes.clone().count() +1;
    
        //println!("Indexes {:?}",col_indexes.collect_vec());

        let row_numbers = lines_iter.clone().take(line_count-1).map(|line|{
            line.split(" ").filter_map(|txt|{
                txt.parse::<i64>().ok()
            }).collect_vec()
        });
    
        let input_data = row_numbers.collect_vec();
    
        //println!("Row numbers are  {:?}",input_data);

        let operators = last_line.unwrap().chars().filter_map(|c|{
            match c{
                '+' =>{Some(day6op::ADD)},
                '*' =>{Some(day6op::MUL)},
                _=>{None}
            }
        });
        //println!("The operators are  {:?}",operators.clone().collect_vec());

        let mut total = 0;
        operators.clone().enumerate().for_each(|(col_number,op)|{
            let mut col_total = input_data[0][col_number];
            (1..(line_count-1)).for_each(|row|{
                //println!("Row{row},col{col_number}");
                col_total = match op{
                    day6op::ADD=>  col_total + input_data[row][col_number],
                    day6op::MUL=>  col_total * input_data[row][col_number]
                }
            
            });
            total+=col_total;
        
        
        });

        //println!("The total is {}",total);
        //Part 1 Answer is 

        self.part_1_solution = total as u128;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{

        let lines_iter = input.lines();
        let last_line = lines_iter.clone().last();
        let line_count = lines_iter.clone().count();//maybe subtract one
    

        //println!("\nPart 2---");
        let input_chars_iter=input.chars();
        let text_width = last_line.unwrap().chars().count();
    
        let mut to_be_consumed = input_chars_iter.clone();
    
        let mut part_2_solution = 0;

        let mut current_acc = 0;
        let mut current_op = day6op::ADD;


        //Discount solution to rotating the data is to skip char count by line width. this essentailly lets us iterate columns.
        //There are more efficient ways to do so than cloning iters, but there are also worse ways. 

        //we have a state machine that clears the accumulaotr every time it finds a new operator, and updates the operator

        loop{
            let current_column = to_be_consumed.clone().step_by(text_width+1);
            let col_result_count = current_column.clone().count();
        
        
            if col_result_count < line_count {
                part_2_solution+=current_acc;//Add last accumulator
                break;
            }
        
            let current_column_vec = current_column.clone().collect_vec();
        
            match current_column_vec[current_column_vec.len()-1]{
                '+' =>{
                    //Tidy up last acc
                    part_2_solution+=current_acc;
                    current_acc=0;
                    current_op = day6op::ADD;
                },
                '*' =>{
                    part_2_solution+=current_acc;
                    current_acc=1;
                    current_op = day6op::MUL;
                },
                _=>{()}
            }
    
            let col_parse = current_column_vec.iter().filter(|c| c.is_digit(10)).collect::<String>().parse::<u64>();

            match (col_parse,current_op.clone()){
                    (Ok(val),day6op::ADD)=> {
                        //println!("Got Val:{val}. ADD");
                        current_acc = current_acc + val;
                    },
                    (Ok(val),day6op::MUL)=> {
                        current_acc= current_acc * val;
                    }
                    (_,_)=>{()}
            }
       
            //println!("First_col {:?}",current_column.collect_vec());        
            to_be_consumed.next();

        }
    
        self.part_2_solution = part_2_solution as u128;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }













    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{6}
    fn get_year()->i32{2025}
}
