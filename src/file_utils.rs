
use std::io::{self,Read,Error};
use std::fs::{File, read_to_string};

pub fn read_code_block(year:i32,day:i32,block_number:i32)->Result<String,io::Error>{
    let file_full_path = format!("./inputs/{year}/{day}/codeblock_{block_number}");
    let mut ret_string=String::new();
    let mut file = File::open(file_full_path)?;
    let _ = file.read_to_string(&mut ret_string)?;
    Ok(ret_string)
}

pub fn read_input_file(year:i32,day:i32)->Result<String,Error>{

    let file_path = format!("./inputs/{year}/{day}");
    let file_full_path = format!("{file_path}/input.txt");

    let mut f = File::open(file_full_path)?;
    let mut ret_string = String::new();
    f.read_to_string(&mut ret_string)?;

    Ok(ret_string)

}