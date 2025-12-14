use std::iter::zip;
use std::ops::Index;

use itertools::Itertools;
use memoize::memoize;

use crate::parse_input;
use crate::solvers::Aoc;
use crate::file_utils;

use petgraph::graph::UnGraph;
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};
use petgraph::visit::NodeIndexable;
use petgraph::stable_graph::StableGraph;


use std::collections::{HashMap, HashSet};
use std::collections::BinaryHeap;

use std::sync::{Arc, Mutex};
use petgraph::algo::connected_components;

use petgraph::prelude::*;


use petgraph::visit::Bfs;




pub struct solution{
    part_1_solution:u128,
    part_2_solution:u128,
    capacity:usize
}
#[derive(Debug,Clone)]
enum day6op{
    ADD,
    MUL
}



#[derive(Hash,Eq,PartialEq,Clone,Debug)]
struct NodePoint{
    x:i64,
    y:i64,
    z:i64,
    cluster_number:Option<usize>,
    connected:bool
}
impl NodePoint{
    fn calc_dist(&self,other:&Self)->i64{
        let dist = ( (self.x-other.x).pow(2)+(self.y-other.y).pow(2)+(self.z-other.z).pow(2) );// as f64;
        //dist.sqrt()
        dist
    }
}


struct Edge {
    edge_index:usize,
    edge_weight:i64
}


impl TryFrom<&str> for NodePoint{
    type Error = &'static str;

    fn try_from(text: &str) -> Result<Self,Self::Error> {
        let v:Vec<i64> = text.split(",").filter_map(|x|{x.parse::<i64>().ok()}).collect();
        match v.len(){
            3=>Ok(NodePoint{
                x:v[0],
                y:v[1],
                z:v[2],
                cluster_number:None,
                connected:false
            }),
            _=>Err("Did not get 3 Ints")
        }
    }
}


impl solution{
    pub fn new()->Self{
        Self{part_1_solution:0,part_2_solution:0,capacity:0}
    }
    pub fn solve_part_1_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),5).expect("Unable to open file");
        self.capacity = 10;
        self.solve_part_1(&input)?;
        assert_eq!(self.part_1_solution,50);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),5).expect("Unable to open file");
        self.solve_part_2(&input)?;
        assert_eq!(self.part_2_solution,25272);
        Ok(())
    }
    pub fn check_part_2(&mut self){
        let input = self.get_input().unwrap();
        let _ = self.solve_part_2(&input);
        assert_eq!(self.part_2_solution,1026594680);
    }
    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();

        assert_eq!(self.part_1_solution,63920);
        assert_eq!(self.part_2_solution,1026594680);
    }

}





impl Aoc for solution{
    fn solve_day(&mut self) -> Result<(), std::io::Error>{
        self.capacity = 1000;
        let input = self.get_input()?;
        let _ = self.solve_part_1(&input);
        let _ = self.solve_part_2(&input);
        Ok(())
    }





    fn solve_part_1(&mut self,input:&str)->Result<(),std::io::Error>{
        println!("input is {:?}",input);

        let data = input.lines().filter_map(|line|{
            if let Some((a,b)) = line.split(",").collect_tuple(){
                Some((a.parse::<i64>().unwrap(),b.parse::<i64>().unwrap()))
            }
            else{
                None
            }
        }).collect_vec();

        println!("input is {:?}",data);

        let node_as =         (0..data.len());
        let node_bs =         (0..data.len());

        //let mut pairs= Vec::new();
        let nbyn:Vec<(usize,usize)> =node_as.map(|a| node_bs.clone().map(move |b: usize|{(a,b)})).flatten().collect();
        let nbyn = nbyn.iter();

        let all_areas = nbyn.filter_map(|(a,b)|{
            if *a!=*b{

                let area = calc_area(data[*a],data[*b]);
                //println!("Got area{area}. Between {:?},{:?}",data[*a],data[*b]);
                Some(area)
            }
            else{
                None
            }
        });
        let max_area = all_areas.max().unwrap();
        println!("max area is {:?}",max_area);


        self.part_1_solution = max_area as u128;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{

        //For part 2, I feel like the solution isnt that much harder. 
        //We simply create a filter to only get valid combinations, then check the same thing
        //In order to create that filter, it will be a bit challenging
        //One way to do so would be to check every square between each two points to see if they are all inside the green red area


        //For each point on the grid
        //compute what is the furthest up we can go
        //compute what is the furthest left we can go
        //Do this with dp / recursion with memonization

        //computing max left we can go for each position [row][col]
        //d[row][col] = 

        //start at top right corner [min row][max col]. Value will be 1
        //next compute [min row][max col-1] = [min row][max col]+1

        //now at every point, we know max area = min()

        //self.part_2_solution = solution as u128;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }



    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{9}
    fn get_year()->i32{2025}
}

fn calc_area(pt_a:(i64,i64),pt_b:(i64,i64))->usize{
    let width = (pt_a.0 - pt_b.0).abs()+1;
    let height = (pt_a.1 - pt_b.1).abs()+1;
    let area = width*height;
    //println!("area {}",area);
    area as usize
}