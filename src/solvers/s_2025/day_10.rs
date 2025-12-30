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


use std::collections::{HashMap, HashSet, hash_map, hash_set};
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
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.capacity = 10;
        self.solve_part_1(&input)?;
        assert_eq!(self.part_1_solution,491);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_2(&input)?;
        assert_eq!(self.part_2_solution,33);
        Ok(())
    }
    pub fn check_part_2(&mut self){
        let input = self.get_input().unwrap();
        let _ = self.solve_part_2(&input);
        assert_eq!(self.part_2_solution,20617);
    }
    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();

        assert_eq!(self.part_1_solution,63920);
        assert_eq!(self.part_2_solution,20617);
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
        //println!("The input is {input}");

        let parsed_input = input.lines().map(|line|{
            let mut line_data = line.split(" ").into_iter().peekable();
            let first_data = line_data.next();
            let second_data = line_data.peeking_take_while(|x|x.contains(")"));

            //println!("A:{:?}",first_data);

            let light_end_state = first_data.unwrap().chars().rev().filter_map(|c|
                match c{
                    '.'=>Some(0u16),
                    '#'=>Some(1u16),
                    _=>None,
                }
            ).reduce(|mut acc,bit|{
                acc= acc<<1;
                acc+bit
            })
            .unwrap();


            //let second_data =second_data.collect_vec()
            //println!("B:{:?}",second_data.collect_vec());
            let second_data = second_data.map(|a|{
                    a.chars().filter_map(|c|c.to_digit(10)).map(|val|{
                        1<<val
                    }).sum::<u16>()
                });
            let second_data = second_data.collect_vec();


            let last_data = line_data.next().unwrap();

            let last_data = last_data.chars().filter_map(|c|c.to_digit(10));
            let last_data = last_data.collect_vec();
            //println!("A:{:?}",light_end_state);
            //println!("B:{:?}",second_data);
            //println!("C:{:?}",last_data);
            (light_end_state,second_data,last_data)
        }).collect_vec();

        let min_numbers = parsed_input.iter().map(|(a,b,c)|{

            let ans_range = 1..=b.len();//Ignore case of all off. ( this will bite me later)
            let mut hash_map = HashMap::new();
            //println!("Looking for {a}");

            'search: for ans in ans_range{
                let buttons_pressed_combos = b.iter().combinations(ans);
                let value_of_buttons = buttons_pressed_combos.clone().map(|p|{
                    (p.clone().into_iter().copied().collect_vec(),p.into_iter().fold(0,|acc,x|(&acc)^x))
                });

                let found_key = value_of_buttons.take_while(|v|{
                    if !hash_map.contains_key(&(v.1)){
                        hash_map.insert(v.1, ans);
                    }
                    !hash_map.contains_key(&a)
                }).collect_vec();//I just need to eval this
                
                if hash_map.contains_key(&a){break 'search;}
            }


            //for x in hash_map.iter().sorted(){//.values()
            //    println!("{:?}",x);
            //}
            //println!("----------\n\n\n\n\n---");
            //At this point hash_map will have the solution.

            *(hash_map.get_key_value(&a).unwrap().1)
        });
        let min_numbers = min_numbers;
        //println!("The min numbers are {:?}",min_numbers);
        self.part_1_solution = min_numbers.sum::<usize>() as u128;

        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{
  
        let parsed_input = input.lines().map(|line|{
            let mut line_data = line.split(" ").into_iter().peekable();
            let first_data = line_data.next();
            let second_data = line_data.peeking_take_while(|x|x.contains(")"));

            //println!("A:{:?}",first_data);

            let light_end_state = first_data.unwrap().chars().rev().filter_map(|c|
                match c{
                    '.'=>Some(0u16),
                    '#'=>Some(1u16),
                    _=>None,
                }
            ).reduce(|mut acc,bit|{
                acc= acc<<1;
                acc+bit
            })
            .unwrap();


            //let second_data =second_data.collect_vec()
            //println!("B:{:?}",second_data.collect_vec());
            let second_data = second_data.map(|a|{
                    a.chars().filter_map(|c|c.to_digit(10)).map(|val|{
                        1<<val
                    }).sum::<u16>()
                });
            let second_data = second_data.collect_vec();


            let last_data = line_data.next().unwrap();


            
            let final_part = last_data.split(",");
            let ggg= final_part.map(|x|{
                let last_chunk_digits:String = x.chars().filter(|c|c.is_digit(10)).collect();
                let val = last_chunk_digits.parse::<u16>().unwrap();
                val
            }).collect_vec();
//            let last_data = .filter(|x|).filter_map(|c|c.to_digit(10));
  //          let last_data = last_data.collect_vec();
            //println!("A:{:?}",light_end_state);
            //println!("B:{:?}",second_data);
            //println!("C:{:?}",ggg);
            (light_end_state,second_data,ggg)
        }).collect_vec();

        let min_numbers = parsed_input.iter().map(|(a,b,c)|{

            let btns = b.iter().map(|f|{*f as u16}).map(|c|{
                let mut btn_vec = Vec::new();
                for b in 0..16{
                    let check_bit = 1<<b;
                    if check_bit == (check_bit&c){
                        btn_vec.push(b);
                    }
                }
                btn_vec
            }

            ).collect_vec();
            let joltage = c.iter().map(|f|{*f as u16}).collect_vec();
            //println!("Trying Joltage {:?}",joltage);
            //println!("Trying Btns {:?}",btns);
            let r = crate::solvers::s_2025::z3_solve::z3_day_10(&btns,&joltage,&joltage);
            r
        });
        let min_number :i64= min_numbers.sum();
        //println!("The min numbers are {:?}",min_numbers);
        self.part_2_solution = min_number as u128;



        //self.part_2_solution = solution as u128;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }













    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{10}
    fn get_year()->i32{2025}
}

