use std::iter::zip;
use std::ops::Index;

use itertools::Itertools;
use memoize::memoize;

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
        assert_eq!(self.part_1_solution,21);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_2(&input)?;
        assert_eq!(self.part_2_solution,40);
        Ok(())
    }
    pub fn check_part_2(&mut self){
        let input = self.get_input().unwrap();
        let _ = self.solve_part_2(&input);
        assert_eq!(self.part_2_solution,231229866702355);
    }
    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();

        assert_eq!(self.part_1_solution,1672);
        assert_eq!(self.part_2_solution,231229866702355);
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

        let mut grid:Vec<Vec<char>> = input.lines().map(|line|{line.chars().collect()}).collect();
        //println!("Grid {:?}",grid);
        //println!("Grid {:?}",grid[1][2]);
        let col_count = grid[0].len();
        let row_count = grid.len();

        //Update next row
        (0..row_count-1).for_each(|row|    //dont create a new row
            (0..col_count).for_each(|col|
                //let center_point = (row,col);
                match (grid[row][col] , grid[row+1][col]){
                    ('|'|'S','.')=>{
                                    //println!("should update {row}+1, {col}");
                                    grid[row+1][col] = '|';
                                },
                    ('|','^')=>{
                                    if grid[row+1][col+1]=='.' {grid[row+1][col+1]='|';}
                                    if grid[row+1][col-1]=='.' {grid[row+1][col-1]='|';}
                                },
                    (_,_)=>{}
                }                

            )
        );
        //println!("Grid {:?}",grid);
        //(0..row_count).for_each(|row|println!("{:?}",grid[row].iter().collect::<String>()));
        /*
        (0..row_count).for_each(|row|println!("{:?}      |:{}  ^:{}",
                grid[row].iter().collect::<String>(),
                grid[row].iter().filter(|x|**x=='|').count() ,
                grid[row].iter().filter(|x|**x=='^').count()
        ));
        */
        let mut split_count = 0;
        (1..row_count).rev().for_each(|row|    //dont create a new row
            (0..col_count).for_each(|col|
                //let center_point = (row,col);
                match (grid[row][col] , grid[row-1][col]){
                    ('^','|')=>{
                        split_count +=1;
                    },
                    ('^','.')=>{
                        grid[row][col] = '.'//Dont need to do this, but it removed unneeded splitters
                    }
                    (_,_)=>{}
                }                

            )
        );


        self.part_1_solution = split_count as u128;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{




        let mut grid:Vec<Vec<char>> = input.lines().map(|line|{line.chars().collect()}).collect();
        //println!("Grid {:?}",grid);
        //println!("Grid {:?}",grid[1][2]);
        let col_count = grid[0].len();
        let row_count = grid.len();
        //println!("Row Count{row_count}, Col COunt{col_count}");
        let start_col = grid[0].iter().find_position(|c| **c=='S').expect("Invalid input").0;
        let part_2_solution = calc_ways_to_go(&grid,start_col,0);


        //For part two, we should think about this
        /*
            From part 1 we know that at maximum we can only split 1672 times. 
            From part 1, we also know the ending location of all tacyons as well as all possible paths.

            If we assume all possible solutions we have 2^1672 potential options
            That is ~2E503, so impossible to check them all to see how many are valid

            What I think is the most straight forward path is at every split, we mark the next row as potential splits.

            //So, because we can only go left or right, and only have one start point. This simplifies things significantly
            We only need to keep track of a single point each row

            I also think there is something to the fact that I'm not seeing any splitters side by side. This could eliminate the issue I was worrying about. In addition. They may only appear on even/odd spacing

                If we assume that start column # is even
                if split it will be an odd column number.




            I think recursion will be the best algorithm

            function_returns_ways_to_go_from_here ( this is easiest to implement )
                if split{
                    ways_to_go = function_returns_ways_to_go_from_here[left] + function_returns_ways_to_go_from_here[right]
                }
                else{
                    ways_to_go = 1*function_returns_ways_to_go_from_here[stay]
                }
                if last row{
                    ways_to_go 1
                }
                return ways_to_go
            This recursive function can be sped up as we might have already computed function_returns_ways_to_go_from_here[left]  or function_returns_ways_to_go_from_here[right]
            from a prior calc. memoize is free improvement 

            stat approach?
                Bottom row count of '|' is how many ways we can end up at the bottom
                For example
                there are 90 ways to end up on last row


            We could create a map using part 1 end state, but this doesnt seem helpful

            What if we walk'ed it backwards?
                for each start | point at the end, how many ways is there to get back to the S? 
        
        
        
         */
    
        self.part_2_solution = part_2_solution as u128;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }













    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{7}
    fn get_year()->i32{2025}
}


#[memoize(Ignore: grid)]
fn calc_ways_to_go(grid: &Vec<Vec<char>>,current_col:usize,current_row:usize)->u64{
    if grid.len() - current_row == 1 {
        1
    }
    else{
        match grid[current_row+1][current_col]{
            '.' =>{//_pair@
                            //println!("We can go Down {:?}",pair);
                            calc_ways_to_go(grid,current_col,current_row+1)
                        },
           '^'=>{// _pair@
                            //println!("We can go left or right! {:?}",pair);
                            let mut total = 0;
                            if  grid[current_row+1][current_col+1]=='.' {//(current_col < (col_count-1)) &&
                                total += calc_ways_to_go(grid,current_col+1,current_row+1);
                            }
                            if  grid[current_row+1][current_col-1]=='.' {//(current_col >(0+1) ) &&
                                total += calc_ways_to_go(grid,current_col-1,current_row+1);
                            }
                            total
                        },
            _=>{1}//println!("On{a},Next{b}");
        }           
    }
}