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
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.capacity = 10;
        self.solve_part_1(&input)?;
        assert_eq!(self.part_1_solution,40);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
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

        let capacity = self.capacity;//in demo this was 10


        let mut nodepoints:Vec<Arc<Mutex<NodePoint>>> = input.lines().filter_map(|f|TryInto::<NodePoint>::try_into(f).ok()).map(|f|Arc::new(Mutex::new(f))).collect();

        let mut node_indexes = HashMap::new();
        let mut graph = StableGraph::<Arc<Mutex<NodePoint>>, i64>::new();
        //let nodepoints = vec![Arc::new(NodePoint{x:0,y:0,z:0}),Arc::new(NodePoint{x:1,y:2,z:3}),Arc::new(NodePoint{x:5,y:5,z:5})];
        for np in nodepoints.clone(){
            let idx = graph.add_node(np.clone());
            node_indexes.insert(Arc::as_ptr(&np), idx);
        }
        //graph.add_edge(node_indexes[&nodepoints[0]],node_indexes[&nodepoints[1]], ());

        let mut shortest_edges = BinaryHeap::with_capacity(capacity);//.new();


        let z_1 =         (0..nodepoints.len());
        let z_2 =         (0..nodepoints.len());

        let nbyn:Vec<(usize,usize)> =z_1.map(|a| z_2.clone().map(move |b: usize|{(a,b)})).flatten().collect();
        let mut nbyn = nbyn.iter();
        while shortest_edges.len() <  shortest_edges.capacity(){//Fill out stack
            if let Some((node_a_idx,node_b_idx)) = nbyn.next(){
                if node_a_idx != node_b_idx{
                    //println!("Adding");


                    //let d = nodepoints[*node_a_idx].calc_dist(&nodepoints[*node_b_idx]);
                    let node_a_val = nodepoints[*node_a_idx].lock().unwrap();
                    let node_b_val = nodepoints[*node_b_idx].lock().unwrap();

                    let d = node_a_val.calc_dist(&node_b_val); 

                    let hash_a_ptr =Arc::as_ptr(&nodepoints[*node_a_idx]);
                    let hash_b_ptr =Arc::as_ptr(&nodepoints[*node_b_idx]);
                    
                    let node_a = node_indexes[&hash_a_ptr];
                    let node_b = node_indexes[&hash_b_ptr];



                    if !graph.contains_edge(node_a,node_b) && !graph.contains_edge(node_b,node_a){
                        let e1 = graph.add_edge(node_a,node_b, d);
                        let e2 = graph.add_edge(node_b,node_a, d);
                        shortest_edges.push((d,(e1,e2)));
                    }
                }
            }
            //println!("Len {}", shortest_edges.len());
        }
        //We have 1000 in there, but these 1000 are not the smallest 1000
        while let Some((node_a_idx,node_b_idx)) = nbyn.next(){
            if node_a_idx != node_b_idx{

                let node_a_val = nodepoints[*node_a_idx].lock().unwrap();
                let node_b_val = nodepoints[*node_b_idx].lock().unwrap();

                let d = node_a_val.calc_dist(&node_b_val); 




                if shortest_edges.peek().unwrap().0 > d{
                    //println!("Found Smaller");
                    //println!("The heap is {:?}",shortest_edges);

                    //We we have a new smallest item.

                    let hash_a_ptr =Arc::as_ptr(&nodepoints[*node_a_idx]);
                    let hash_b_ptr =Arc::as_ptr(&nodepoints[*node_b_idx]);
                    
                    let node_a = node_indexes[&hash_a_ptr];
                    let node_b = node_indexes[&hash_b_ptr];

                    if !graph.contains_edge(node_a,node_b) && !graph.contains_edge(node_b,node_a){
                        let edge_to_remove = shortest_edges.pop();
                        let e1 = graph.add_edge(node_a,node_b, d);
                        let e2 = graph.add_edge(node_b,node_a, d);
                        shortest_edges.push((d,(e1,e2)));
                        graph.remove_edge(edge_to_remove.unwrap().1.0);
                        graph.remove_edge(edge_to_remove.unwrap().1.1);
                    }
                }
                
                //let node_a = node_indexes[&nodepoints[*node_a_idx]];
                //let node_b = node_indexes[&nodepoints[*node_b_idx]];
                //graph.add_edge(node_a,node_b, d);
                //shortest_edges.push(d);
            }
        }
        //println!("The heap is {:?}",shortest_edges);




        
        //println!("Connected components {:?}",connected_components(&graph));
        //println!("{:?}", Dot::new(&graph));
        



        //Get spanning tree

        let graph_clone = graph.clone();
        let mut list_of_nodes = graph_clone.node_indices().into_iter();

        let mut cluster_count: HashMap<usize, i32> = HashMap::new();//This will be a (cluster#,count);


        let mut cluster_number = 0;
        cluster_count.insert(cluster_number, 0);
        while let Some(ep) =list_of_nodes.next() {

            let a = graph[ep].clone();
            
            let mut added_new_cluster=false;
            let mut bfs = Bfs::new(&graph, ep);
            'bfs :while let Some(nx) = bfs.next(&graph) {
                // we can access `graph` mutably here still
                let mut node_point_ref = graph[nx].lock().unwrap();
                if node_point_ref.cluster_number == None{
                    added_new_cluster = true;
                    node_point_ref.cluster_number = Some(cluster_number);
                    let val_in_hmap = cluster_count.get_mut(&cluster_number).unwrap();
                    *val_in_hmap+=1;


                }
                else{
                    break 'bfs;
                }
            }
            //println!("span {:?}",s);
            if added_new_cluster{
                cluster_number+=1;
                cluster_count.insert(cluster_number, 0);
            }
        }

        let cluster_count = cluster_count.iter().sorted_by(|f,g|{
            g.1.cmp(f.1)
        }).take(3).collect_vec();

        //println!("{:?}", Dot::new(&graph));
        
        println!("{:?}", cluster_count);
        let mut solution = cluster_count.iter().fold(1,|acc,e|{
            acc*e.1
        });//multiply all values together


        self.part_1_solution = solution as u128;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }








    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{
  


        //for this one it is actually less hash maps
        //Steps
        //1 ) create graph with no edges
        //2 ) find closest non connected edge
        //3 ) connect that edge. Mark both nodes a visited
        //4 ) loop over all nodes. If there is any unvisited go back to #2
        //4 ) if all are visited. use edge from #3/nodes from #2 to compute answer


        //another thought
        //what if the last connection is just the max distance apart? ( dont think this is correct)

        //1)
        let mut node_points:Vec<Arc<Mutex<NodePoint>>> = input.lines().filter_map(|f|TryInto::<NodePoint>::try_into(f).ok()).map(|f|Arc::new(Mutex::new(f))).collect();

        //let mut node_indexes = HashMap::new();
        let mut the_edge_set:HashSet<(usize, usize)> = HashSet::new();//this is a set of two indexes into node_points vector
        
        //This solution is dumb. I should have just computed all edge distances. Sorted that list of all edges, then kept connecting until they are all marked connected
        //This way I'm not searching each each time I make a connection
        //It is only n^2 to compute that list, and only need 1000x1000 memory to store it
        /*
        let mut solution =0;
        'addTillConnected :loop {
            let next_node = get_closest_two_nodes(&node_points,&the_edge_set);
            the_edge_set.insert(next_node);
            println!("Edge Set Len:{:?}",the_edge_set.len());
            node_points[next_node.0].lock().unwrap().connected=true;
            node_points[next_node.1].lock().unwrap().connected=true;
            if are_all_connected(&node_points){
                solution = node_points[next_node.0].lock().unwrap().x * node_points[next_node.1].lock().unwrap().x;               
                break 'addTillConnected;
            }
        }
        */

        //This should be a much faster design, but worse on memory. Can also remove the edge set if needed.
        let sorted_edges = get_sorted_edges_by_distance(&node_points);
        let mut sorted_edges_iter= sorted_edges.iter();
        let mut solution =0;
        while let Some(next_node) = sorted_edges_iter.next() {
            //the_edge_set.insert(next_node.1);
            //println!("Edge Set Len:{:?}",the_edge_set.len());
            node_points[next_node.1.0].lock().unwrap().connected=true;
            node_points[next_node.1.1].lock().unwrap().connected=true;
            if are_all_connected(&node_points){
                solution = node_points[next_node.1.0].lock().unwrap().x * node_points[next_node.1.1].lock().unwrap().x;               
                break;//'addTillConnected;
            }
        }
        






        self.part_2_solution = solution as u128;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }













    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{8}
    fn get_year()->i32{2025}
}


fn get_sorted_edges_by_distance(node_points:&Vec<Arc<Mutex<NodePoint>>>)->Vec<(i64, (usize,usize))>{
    //let node_as = node_points.clone().iter();
    //let node_bs = node_as.clone();

    let node_as =         (0..node_points.len());
    let node_bs =         (0..node_points.len());



    let mut pairs= Vec::new();
    let nbyn:Vec<(usize,usize)> =node_as.map(|a| node_bs.clone().map(move |b: usize|{(a,b)})).flatten().collect();
    let mut nbyn = nbyn.iter();

    while let Some((a,b))= nbyn.next(){
        if *a!=*b{
            let na = node_points[*a].clone();
            let nb = node_points[*b].clone();
            let d = na.lock().unwrap().calc_dist(&nb.lock().unwrap());
            pairs.push((d,(*a,*b)));
        }
    }
    pairs.sort();
    let pairs = pairs.iter().step_by(2).map(|x| *x).collect();
    pairs
}


fn are_all_connected(node_points:&Vec<Arc<Mutex<NodePoint>>>)->bool{
    let result = node_points.iter().all(|f|f.lock().unwrap().connected);
    result
}

fn get_closest_two_nodes(node_points:&Vec<Arc<Mutex<NodePoint>>>,set_of_edges:&HashSet<(usize, usize)>)->(usize,usize){
    let z_1 =         (0..node_points.len());
    let z_2 =         (0..node_points.len());

    let nbyn:Vec<(usize,usize)> =z_1.map(|a| z_2.clone().map(move |b: usize|{(a,b)})).flatten().collect();
    let mut nbyn = nbyn.iter();
    let mut min_d = i64::MAX;
    let mut shortest_pair = (0,0);
    while let Some((node_a_idx,node_b_idx)) = nbyn.next(){
        if node_a_idx != node_b_idx{
            let node_a_val = node_points[*node_a_idx].lock().unwrap();
            let node_b_val = node_points[*node_b_idx].lock().unwrap();
            let this_d = node_a_val.calc_dist(&node_b_val);
            if !set_of_edges.contains(&(*node_a_idx,*node_b_idx)){
                //This pair did not exist. It is candidate for shortest connection
                if min_d > this_d{
                    min_d = this_d;
                    shortest_pair = (*node_a_idx,*node_b_idx);
                }
            }
        }
    }
    shortest_pair
}