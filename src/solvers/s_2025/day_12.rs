use std::fmt::Debug;
use std::io::Write;
use std::iter::zip;
use std::ops::Index;
use std::{char, option};

use itertools::Itertools;
use petgraph::Direction::Outgoing;
use petgraph::prelude::GraphMap;
use petgraph::Directed;
use crate::parse_input;
use crate::solvers::Aoc;
use crate::file_utils;
use memoize::memoize;

use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::{self, dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};
use petgraph::visit::NodeIndexable;
use petgraph::stable_graph::StableGraph;
use std::collections::hash_map::RandomState;
use std::collections::hash_map;
use petgraph::visit::Bfs;
use petgraph::visit::Dfs;

use std::collections::{HashMap, HashSet, hash_set};
use std::collections::BinaryHeap;

use std::sync::{Arc, Mutex};
use petgraph::algo::connected_components;

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
        //assert_eq!(self.part_1_solution,5);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),1).expect("Unable to open file");
        self.solve_part_2(&input)?;
        //assert_eq!(self.part_2_solution,2);
        Ok(())
    }

    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();

        assert_eq!(self.part_1_solution,472);
        assert_eq!(self.part_2_solution,526811953334940);
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

//#[derive(Hash,Eq,Clone,Debug)]
#[derive(Hash,Clone, Copy, Eq, Ord, PartialOrd,Debug)]
struct Node{
    name: [u8;3]
    //children:Vec<String>
    //cluster_number:Option<usize>,
    //connected:bool
}
impl Node{

}
impl PartialEq for Node{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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


        let mut current_index = 0;
        let mut shape_stack = Vec::new();
        let mut shapes = Vec::new();
        let mut all_test_cases = Vec::new();

        for line in input.lines(){
            if line.contains("x"){
                let mut splits = line.split("x");
                let width = splits.nth(0).expect("bad input file").parse::<u16>().expect("bad input");
                let mut splits = splits.nth(0).unwrap().split(":");
                let height = splits.nth(0).expect("bad input file").parse::<u16>().expect("bad input");
                let splits = splits.nth(0).unwrap().split(" ");
                let counts = splits.filter(|a|a.len()>0).filter_map(|a|{a.parse::<u16>().ok()}).collect_vec();
                println!("Got width{}. Height{}. Counts{:?}",width,height,counts);
                let mut new_hash_map: HashMap<usize, u16> = HashMap::new();
                counts.iter().enumerate().for_each(|(a,b)|{
                    new_hash_map.insert(a, *b);
                });
                let parsed_test_case = test_case{present_count:new_hash_map,width,height,area:(width as u32)*(height as u32)};
                all_test_cases.push(parsed_test_case);
            }
            else if line.contains(":"){
                let index = line.split(":").nth(0).unwrap().chars().filter_map(|c| c.to_digit(10)).nth(0).expect("Bad input file");
                current_index = index;
                println!("Current index is{current_index}");
            }
            else if line.contains(".") || line.contains("#"){
                let row = line.chars().map(|c|
                    c=='#'
                ).collect_vec();
                let new_row = [row[0],row[1],row[2]];
                shape_stack.push(new_row);
            }
            else if shape_stack.len()>=3{
                //add shape to shapes
                let shape = [shape_stack.pop().unwrap(),shape_stack.pop().unwrap(),shape_stack.pop().unwrap()];
                let a = shape.iter().flatten().filter(|c|**c).count();
                let p = present{data:shape,area:a as u8};
                shapes.push(p);
            }
        }

        
        let achievable = all_test_cases.iter().filter(|t| check_test_case_basic_area(&shapes,&t));        
        println!("PresentShapes{:?}",shapes);
        println!("We are only able to solve at max:{:?}",achievable.count());



        //self.part_1_solution = all_paths_len as u128;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{
        
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }













    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{12}
    fn get_year()->i32{2025}
}

#[derive(Debug)]
struct present{
    data:[[bool;3];3],
    area:u8
}


#[derive(Debug)]
struct test_case{
    present_count:HashMap<usize, u16>,
    width:u16,
    height:u16,
    area:u32,
}

fn check_test_case_basic_area(present_shapes:&Vec<present>,test:&test_case)->bool{
    let mut area_total = 0;
    for key in test.present_count.keys(){
        area_total+= (present_shapes[*key].area as u32)*(test.present_count[key] as u32);
    } 
    //println!("{area_total}-{}",test.area);
    if test.area>area_total{
        println!("only have {} free spaces",test.area-area_total);
    }
    test.area>area_total
}
