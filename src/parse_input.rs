use std::{fs::{self, File, read}, io::{self, Error, Read, Write}};

use std::str::FromStr;
use std::fmt::Debug;

use crate::file_utils;


pub fn get_char_inst_count(input:&str,search:char)->usize{
    input.chars().filter(|c| *c == search).count()
}
pub fn get_string_line_count(input:&str)->usize{
    get_char_inst_count(input,'\n')
}

pub fn parse_basic_numbers<T:FromStr>(input:&str,line_count:usize)->Vec<Vec<T>>
where <T as FromStr>::Err: Debug
{
    //We assume each line is is own row
    let delimiter="   ";
    let mut rows: Vec<Vec<T>> = Vec::with_capacity(line_count-1);

    let input_split = input.split("\n");    
    input_split.into_iter().for_each(|line|{
        let inner_split = line.split(delimiter);
        let row_data:Vec<T> = inner_split.into_iter().filter(|x| x.len()>=1 ).map(|x| x.parse::<T>().unwrap() ).collect();
        if row_data.len()!=0{
            rows.push(row_data);
        }
    });
    return rows
}

pub fn parse_basic_string<T:FromStr>(input:&str,line_count:usize)->Vec<&str>
where <T as FromStr>::Err: Debug
{
    let mut rows: Vec<&str> = Vec::with_capacity(line_count-1);
    let input_split = input.split("\n");    
    input_split.into_iter().for_each(|line|{
        if line.len()>=1{
            rows.push(line);
        }
    });
    return rows
}


pub fn get_next_int(input:&str)->(Option<i64>,usize){
    let the_map= [
        //("0",0),
        ("1",1),
        ("2",2),
        ("3",3),
        ("4",4),
        ("5",5),
        ("6",6),
        ("7",7),
        ("8",8),
        ("9",9),
        ("one",1),
        ("two",2),
        ("three",3),
        ("four",4),
        ("five",5),
        ("six",6),
        ("seven",7),
        ("eight",8),
        ("nine",9),
        ];
    for (s,val) in the_map.iter(){
        if input.starts_with(s){
            //return (Some(*val),s.len())
            return (Some(*val),1)//Alllow nestest numbers twone
        }
    }
    (None,1)
}

pub fn convert_text_string_to_list(input:&str)->Vec<i64>{
    //This function takes in a input string
    //It returns a list of numbers from the text it was given
    //Valid words are one two three...
    //valid digits are 0-9
    let mut ret_list:Vec<i64> = Vec::new();
    let mut current_pos = 0;
    while current_pos<input.len(){
        let result = get_next_int(&input[current_pos..]);
        //println!("On:{:?},got{:?}",&input[current_pos..],result);
        match result{
            (Some(x),step_size)=>{
                ret_list.push(x);
                current_pos+=step_size;
            },
            (None,step_size)=>{
                current_pos+=step_size;
            }
        }
    }
    ret_list
}