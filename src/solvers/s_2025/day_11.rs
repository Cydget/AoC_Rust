use std::fmt::Debug;
use std::io::Write;
use std::iter::zip;
use std::ops::Index;
use std::option;

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

use petgraph::visit::Bfs;
use petgraph::visit::Dfs;

use std::collections::{HashMap, HashSet};
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
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),2).expect("Unable to open file");
        self.solve_part_1(&input)?;
        assert_eq!(self.part_1_solution,5);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),1).expect("Unable to open file");
        self.solve_part_2(&input)?;
        assert_eq!(self.part_2_solution,2);
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

        //println!("Input is:\n{}",input);
        //let mut nodepoints:Vec<Arc<Mutex<NodePoint>>> = input.lines().filter_map(|f|TryInto::<NodePoint>::try_into(f).ok()).map(|f|Arc::new(Mutex::new(f))).collect();
        //let mut node_indexes = HashMap::new();
        //let mut graph = StableGraph::<Arc<Mutex<Node>>, i64>::new();
        let mut graph = GraphMap::<Node, i64,petgraph::Directed>::new();


        input.lines().for_each(|line|{
            let mut split_c = line.split(":");
            let start = split_c.nth(0).unwrap();
            let rest = split_c.nth(0).unwrap();
            let children = rest.split(" ").into_iter().filter_map(|s|{
                if s.len()>2{
                    Some(s)
                }
                else{
                    None
                }
            }).collect_vec();
            //println!("Head is:{}",start);
            //println!("Rest is:{:?}",children);
            //children:children.iter().map(|f|f.to_string()).collect_vec()
            let new_node = Node{name:start.as_bytes().try_into().unwrap()};
            let parent_node = graph.add_node(new_node);
            for child in children{
                let child_node = graph.add_node(Node{name:child.as_bytes().try_into().unwrap()});            
                graph.add_edge(parent_node, child_node, 1);
            }
        });

        let you_node = graph.add_node(Node{name:"you".as_bytes().try_into().unwrap()});            
        let out_node = graph.add_node(Node{name:"out".as_bytes().try_into().unwrap()});            
        let mut path_count = 0;
        
        let all_paths = algo::all_simple_paths::<Vec<_>, _, RandomState>(&graph, you_node, out_node, 0, None);
        let all_paths_len = all_paths.collect_vec().len();
        /*let mut bfs = Dfs::new(&graph,you_node);
        
        while let Some(nx)= bfs.next(&graph){
            println!("Node is {:?}",nx);
            if nx == out_node{
                path_count+=1;
            }
        }
        */

        //println!("The Graph is {:?}",graph);
        //println!("{:?}", Dot::new(&graph));
        println!("The path count is: {}",all_paths_len);
        self.part_1_solution = all_paths_len as u128;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{
        //println!("Input:{}",input);
        let mut graph = GraphMap::<Node, u32,petgraph::Directed>::new();


        input.lines().for_each(|line|{
            let mut split_c = line.split(":");
            let start = split_c.nth(0).unwrap();
            let rest = split_c.nth(0).unwrap();
            let children = rest.split(" ").into_iter().filter_map(|s|{
                if s.len()>2{
                    Some(s)
                }
                else{
                    None
                }
            }).collect_vec();
            //println!("Head is:{}",start);
            //println!("Rest is:{:?}",children);
            //children:children.iter().map(|f|f.to_string()).collect_vec()
            let new_node = Node{name:start.as_bytes().try_into().unwrap()};
            let parent_node = graph.add_node(new_node);
            for child in children{
                let child_node = graph.add_node(Node{name:child.as_bytes().try_into().unwrap()});            
                graph.add_edge(parent_node, child_node, 1);
            }
        });
        //println!("{:?}", Dot::new(&graph));
        let is_cyclic = algo::is_cyclic_directed(&graph);
        println!("The graph is cyclic {}",is_cyclic);


        //let mut dot_file = std::fs::File::create("out.dot").expect("Making basic file");
        //let dot_text = format!("{:?}",Dot::new(&graph));
        //dot_file.write(dot_text.as_bytes()).expect("write");
        
        //let you_node = graph.add_node(Node{name:"you".as_bytes().try_into().unwrap()});       
        let out_node = graph.add_node(Node{name:"out".as_bytes().try_into().unwrap()});
        let svr_node = graph.add_node(Node{name:"svr".as_bytes().try_into().unwrap()});
        let fft_node = graph.add_node(Node{name:"fft".as_bytes().try_into().unwrap()});            
        let dac_node = graph.add_node(Node{name:"dac".as_bytes().try_into().unwrap()});            
        
        //This is a directed graph. We can only go in one order, because if we dont we would end up having cyclic loops.
        
        let count_fft_dac = get_count_of_ways(&graph,fft_node,dac_node);

        if count_fft_dac!=0{
            let count_svr_fft = get_count_of_ways(&graph,svr_node,fft_node);
            let count_dac_out = get_count_of_ways(&graph,dac_node,out_node);
            println!("Fast Count is {}x{}x{}",count_svr_fft,count_fft_dac,count_dac_out);
            self.part_2_solution+=count_svr_fft*count_fft_dac*count_dac_out
        }

        let count_dac_fft = get_count_of_ways(&graph,dac_node,fft_node);

        if count_dac_fft!=0{
            let count_svr_dac = get_count_of_ways(&graph,svr_node,dac_node);
            let count_fft_out = get_count_of_ways(&graph,fft_node,out_node);
            println!("Fast Count is {}x{}x{}",count_svr_dac,count_dac_fft,count_fft_out);
            self.part_2_solution+=count_svr_dac*count_dac_fft*count_fft_out;

        }
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }













    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{11}
    fn get_year()->i32{2025}
}




#[memoize(Ignore: graph)]
fn get_count_of_ways<'a>(graph:&GraphMap<Node, u32, Directed>,start:Node,end:Node)->u128{

    if start == end{
        return 1;
    }
    let candidates = graph.neighbors_directed(start, Outgoing);
    let in_path_nodes = candidates.filter(|c|algo::has_path_connecting(&graph, *c, end,None));

    let mut current_sum = 0;
    for next_child in in_path_nodes{
        current_sum += get_count_of_ways(graph, next_child, end.clone());
    }
    current_sum
} 