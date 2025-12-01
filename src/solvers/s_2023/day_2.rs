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
#[derive(Debug)]
struct dice_set{
    red:i32,
    green:i32,
    blue:i32,
}

fn parse_set_string_to_set(set:&str)->dice_set{
    let mut current_set = dice_set{red:0,green:0,blue:0};
    //println!("Parsing Set:{}",set);
    set.split(",").into_iter().for_each(|item|{
        if let Some((num,color)) = item.split_at(1).1.split_once(" "){
            //println!("The Num is{num},color is:{color}");
            match color{
                "blue"=>{current_set.blue+=     num.parse::<i32>().unwrap();},
                "green"=>{current_set.green+=   num.parse::<i32>().unwrap();},
                "red"=>{current_set.red+=       num.parse::<i32>().unwrap();},
                _=>{println!("Error!!!");}
            }
        }
    });
    //println!("Result:{:?}",current_set);

    current_set
}

fn get_max_dice_set(the_set:&dice_set,the_max_set:&dice_set)->dice_set{
    dice_set { red: the_set.red.max(the_max_set.red),
              green: the_set.green.max(the_max_set.green),
              blue: the_set.blue.max(the_max_set.blue)
    }
}
fn is_valid_dice_set(the_set:&dice_set)->bool{
    if the_set.red > 12 {return false;}
    if the_set.green > 13 {return false;}
    if the_set.blue > 14 {return false;}
    return true
}


fn get_max_dice_power(row:&str)->Option<i32>{
    //return game id and if we are valid
    if let Some((game_id,rest)) = row.split_at(5).1.split_once(":"){
        //println!("Game ID is:{game_id},Game String is {rest}");
        let sets = rest.split(";");
        let mut max_dice_set = dice_set{red:0,green:0,blue:0};
        let row_dice_power = sets.into_iter().for_each(|game_set|{
            let the_parsed_set = parse_set_string_to_set(game_set);
            //println!("Parsed Set {:?}",the_parsed_set);
            max_dice_set = get_max_dice_set(&the_parsed_set,&max_dice_set);
        });
        println!("The max dice set is {:?}",max_dice_set);
        return Some(max_dice_set.red*max_dice_set.blue*max_dice_set.green);
    }
    None
}

fn is_game_row_valid(row:&str)->Option<i32>{
    //return game id and if we are valid
    if let Some((game_id,rest)) = row.split_at(5).1.split_once(":"){
        //println!("Game ID is:{game_id},Game String is {rest}");
        let sets = rest.split(";");
        let game_is_valid = sets.into_iter().map(|game_set|{
            let the_parsed_set = parse_set_string_to_set(game_set);
            //println!("Parsed Set {:?}",the_parsed_set);
            the_parsed_set
        })
        .all(|f|{
            is_valid_dice_set(&f)
        });
        //println!("This game is valid {game_is_valid}");
        if game_is_valid{
            return Some(game_id.parse::<i32>().unwrap());
        }
    }
    None
}


impl Aoc for solution{
    fn solve_day(&mut self) -> Result<(), std::io::Error>{
        let input = self.get_input()?;
        let _ = self.solve_part_1(&input);
        let _ = self.solve_part_2(&input);
        Ok(())
    }
    fn solve_part_1(&mut self,input:&str)->Result<(),std::io::Error>{
        //println!("Input is:{}",input);
        let result: i32 = input.split("\n").into_iter().filter(|f| f.len()>=1).filter_map(|row|{
            let a = is_game_row_valid(&row);
            a
        }).sum();
        self.part_1_solution = result as i64;
        

        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }
    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{
        let result: i32 = input.split("\n").into_iter().filter(|f| f.len()>=1).filter_map(|row|{
            let a = get_max_dice_power(&row);
            a
        }).sum();
        self.part_2_solution = result as i64;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }
    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{2}
    fn get_year()->i32{2023}
}
