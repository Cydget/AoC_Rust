use std::iter::zip;
use std::ops::Index;

use itertools::{Itertools, iproduct};
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

fn calc_dist(v1:&Vec<i64>,v2:&Vec<i64>)->Vec<i64>{
    v2.iter().zip(v1.iter()).map(|(a,b)| a-b).collect()
}
fn abs_vec(v1:&Vec<i64>)->Vec<i64>{
    v1.iter().map(|a| a.abs()).collect()
}
fn sgn_vec(v1:&Vec<i64>)->Vec<i64>{
    v1.iter().map(|a| a.signum()).collect()
}


fn calc_dist_tup(v1:&(i64,i64),v2:&(i64,i64))->(i64,i64){
    (v2.0-v1.0,v2.1-v1.1)
}
fn abs_tup(v1:&(i64,i64))->(i64,i64){
    (v1.0.abs(),v1.1.abs())
}
fn sgn_tup(v1:&(i64,i64))->(i64,i64){
    (v1.0.signum(),v1.1.signum())
}



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
        //assert_eq!(self.part_2_solution,25272);
        Ok(())
    }
    pub fn check_part_2(&mut self){
        let input = self.get_input().unwrap();
        let _ = self.solve_part_2(&input);
        assert_eq!(self.part_2_solution,1560299548);
    }
    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();

        assert_eq!(self.part_1_solution,4776487744);
        assert_eq!(self.part_2_solution,1560299548);
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
        //println!("input is {:?}",input);

        let data = input.lines().filter_map(|line|{
            if let Some((a,b)) = line.split(",").collect_tuple(){
                Some((a.parse::<i64>().unwrap(),b.parse::<i64>().unwrap()))
            }
            else{
                None
            }
        }).collect_vec();

        //println!("input is {:?}",data);
        println!("part 1 point count{}",data.len());
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


        let mut red_points:Vec<(i64,i64)> = input.lines().filter_map(|line|{
            if let Some((a,b)) = line.split(",").collect_tuple(){
                let v= (a.parse::<i64>().unwrap(),b.parse::<i64>().unwrap());
                Some(v)
            }
            else{
                None
            }
        }).collect_vec();

        red_points.push(red_points[0].clone()); //connect back to start

        let winding = calc_winding_direction(&red_points);
        println!("Overall winding is {:?}",winding);
        match winding{
            WindingDirection::CW=>{},
            WindingDirection::CCW=>{
                red_points = red_points.iter().rev().map(|f|f.clone()).collect_vec();
            },
        }
        let winding = calc_winding_direction(&red_points);
        println!("Overall winding is now {:?}",winding);

        let max_height = ( red_points.clone().iter().map(|x|x.1).max().unwrap() +2);
        let max_width =  (red_points.clone().iter().map(|x|x.0).max().unwrap() +2);


        //panic!("Early exit");


        let all_start_and_end_pos = (0..red_points.len()).combinations(2);//Much smaller list
        //println!("{:?}",all_start_and_end_pos.collect_vec());
        let mut max_area = 0;
        let mut valid_paths = Vec::new();
        for v in all_start_and_end_pos{
            let p1 = red_points[v[0]];
            let p2 = red_points[v[1]];

            //let winding_num = (p2.0-p1.0)*(p2.1+p1.1);


            //if (p1 == (9,5) && p2 == (2,3)) || p1 == (2,3) && p2 == (9,5) {
            //    println!("\n\n\nLargest Rect\n\n\n\n");
            //}

            let can_move = can_move_clockwise_to_point(&red_points,v[0],v[1]);

            if can_move.0{
                let a =                 calc_area(p1,p2);
                //println!("Valid Rect. Calc Area{:?} to {:?}. A={}",p1,p2,a);
                let solution_points = vec![can_move.1,can_move.2,can_move.3,can_move.4,can_move.1];
                if a > max_area{
                    let solution_path = crate::solvers::s_2025::day_9_vis::get_square_path(&solution_points);
                    valid_paths.push(solution_path);
                }
                max_area = max_area.max(a);
            }
        }

        crate::solvers::s_2025::day_9_vis::create_svg(&red_points, max_width, max_height, "output_path.svg",&valid_paths).expect("img error");


        println!("Red {}",red_points.len());
        self.part_2_solution = max_area as u128;
        //println!("RedGreenCount {}",red_green_points.len());
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




#[derive(PartialEq,Debug)]
enum TravelDirection{
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    NONE,
    ERR,
}
impl From<(i64,i64)> for TravelDirection{
    fn from((a,b): (i64, i64)) -> Self { 
        match (a,b){
            (0,0)=>Self::NONE, //no movement
            (1,0)=>Self::RIGHT,//Increase x
            (-1,0)=>Self::LEFT,//decrease x
            (0,1)=>Self::UP,   //increase y
            (0,-1)=>Self::DOWN,//decrease y
            ( 1, 1)=>Self::UpRight,
            (-1, 1)=>Self::UpLeft,
            ( 1,-1)=>Self::DownRight,
            (-1,-1)=>Self::DownLeft,
            (_,_)=>Self::ERR,  //diagnal or something
        } 
    }
}

fn calc_if_can_move_to_point(red_points:&Vec<(i64,i64)>,start_index:usize,end_position:(i64,i64))->(bool,usize){
    //Red points is a connected boundry. start index should be a point before our current position. We walk along the boundry. If boundry cuts off our desired point. we can not move to the  desired postion directly.
    //This assumes the red points are winding clockwise. 
    let index_range = start_index..red_points.len();

    let moving_direction = TravelDirection::RIGHT;
    let mut max_travel_distance_in_direction = 0;
    match moving_direction{
            TravelDirection::RIGHT=>{
                for red_index in index_range{
                    let mut current_position = red_points[red_index];
                    println!("Moving past point{:?}",current_position);
                    if current_position.0 >= end_position.0{//We moved left far enough to include our current point.
                        max_travel_distance_in_direction=max_travel_distance_in_direction.max(current_position.0);//This is if we remove the early return
                        return (true,red_index);
                        break;//break for red_index loop                        
                    }
                    else if current_position.1 < end_position.1 && red_index != start_index{
                        //return false;//Along the way, we ended up moving down before passing our current point
                        break;
                    }
                }
            },
            TravelDirection::DOWN=>{
                for red_index in index_range{
                    let mut current_position = red_points[red_index];
                    println!("Moving past point{:?}",current_position);

                    if current_position.1 <= end_position.1{//We moved down far enough to include our current point.
                        max_travel_distance_in_direction=max_travel_distance_in_direction.max(current_position.1);//This is if we remove the early return
                        return (true,red_index);
                        break;//break for red_index loop                        

                    }
                    if current_position.0 < end_position.0 && red_index != start_index{
                        //return true;//Along the way, we ended up moving left before passing our current point
                        break;
                    }
                }
            },
            TravelDirection::UP=>{
                for red_index in index_range{
                    let mut current_position = red_points[red_index];
                    println!("Moving past point{:?}",current_position);

                    if current_position.1 >= end_position.1{//We moved down far enough to include our current point.
                        max_travel_distance_in_direction=max_travel_distance_in_direction.max(current_position.1);//This is if we remove the early return
                        return (true,red_index);
                        break;//break for red_index loop                        

                    }
                    if current_position.0 > end_position.0 && red_index != start_index{
                        //return true;//Along the way, we ended up moving left before passing our current point
                        break;
                    }
                }
            },
            TravelDirection::LEFT=>{
                for red_index in index_range{
                    let mut current_position = red_points[red_index];
                    println!("Moving past point{:?}",current_position);

                    if current_position.0 <= end_position.0{//We moved left far enough to include our current point.
                        max_travel_distance_in_direction=max_travel_distance_in_direction.max(current_position.0);//This is if we remove the early return
                        return (true,red_index);
                        break;//break for red_index loop                        
                    }
                    else if current_position.1 > end_position.1 && red_index != start_index{
                        //return false;//Along the way, we ended up moving up before passing our current point
                        break;
                    }
                }
            },
            TravelDirection::NONE=>{
                return (true,start_index);
            },
            TravelDirection::ERR=>{
                return (false,start_index);
            },
            _=>{}

    }

    return (false,start_index);
}

#[derive(Debug,Clone)]
enum WindingDirection{
    CW,
    CCW
}

fn calc_winding_direction(red_points:&Vec<(i64,i64)>)->WindingDirection{
    let winding_number = calc_winding_number(red_points);
    println!("Winding number:{winding_number}");
    if winding_number > 0 {
        WindingDirection::CW
    }
    else{
        WindingDirection::CCW
    }
}

fn calc_winding_number(red_points:&Vec<(i64,i64)>)->i64{
    assert_eq!(red_points[0],red_points[red_points.len()-1],"This function needs loop to close itself");
    let val_iter= red_points.windows(2).filter_map(|pair|{
        if pair.len()==2{
            let v = (pair[1].0-pair[0].0)*(pair[1].1+pair[0].1);
            Some(v)
        }
        else{
            None
        }
    });
    val_iter.sum()
}






fn can_move_right_cw(red_points:&Vec<(i64,i64)>,start_index:usize,end_position:(i64,i64))->bool{

    let offsets = 0..red_points.len();

    for offset in offsets{
        let current_index = start_index+offset;
        let current_index = current_index % red_points.len();

        let current_position = red_points[current_index];
        //println!("Searching Right CW. Current_point_on_edge:{:?}",current_position);
        if current_position.0 >= end_position.0{//We moved right far enough to include our current point.
            return true;
        }
        else if current_position.1 < end_position.1 {
            return false;//Along the way, we ended up moving down before passing our current point
        }
    }

    false
}

fn can_move_right_ccw(red_points:&Vec<(i64,i64)>,start_index:usize,end_position:(i64,i64))->bool{

    let offsets = (0..red_points.len());//.rev();

    for offset in offsets{
        //let current_index = start_index+offset;
        //let current_index = current_index % red_points.len();
        let current_index =   (start_index + red_points.len() - offset) % red_points.len();
        let current_position = red_points[current_index];
        //println!("Searching Right CCW. Current_point_on_edge:{:?}",current_position);
        if current_position.0 >= end_position.0{//We moved right far enough to include our current point.
            return true;
        }
        else if current_position.1 > end_position.1 {
            return false;//Along the way, we ended up moving down before passing our current point
        }
    }

    false
}

fn can_move_left_cw(red_points:&Vec<(i64,i64)>,start_index:usize,end_position:(i64,i64))->bool{
    
    let offsets = (0..red_points.len());

    for offset in offsets{
        let current_index = start_index+offset;
        let current_index = current_index % red_points.len();

        let current_position = red_points[current_index];
        //println!("Searching left CW. Current_point_on_edge:{:?}",current_position);
        if current_position.0 <= end_position.0{//We moved left far enough to include our current point.
            return true;
        }
        else if current_position.1 > end_position.1{
            return false;//Along the way, we ended up moving down before passing our current point
        }
    }
    false
}

fn can_move_left_ccw(red_points:&Vec<(i64,i64)>,start_index:usize,end_position:(i64,i64))->bool{
    
    let offsets = (0..red_points.len());//.rev();

    for offset in offsets{
        //let current_index = start_index+offset;
        //let current_index = current_index % red_points.len();
        let current_index =   (start_index + red_points.len() - offset) % red_points.len();

        let current_position = red_points[current_index];
        //println!("Searching left CCW. Current_point_on_edge:{:?}",current_position);
        if current_position.0 <= end_position.0{//We moved left far enough to include our current point.
            return true;
        }
        else if current_position.1 < end_position.1{
            return false;//Along the way, we ended up moving down before passing our current point
        }
    }
    false
}


fn can_move_up_cw(red_points:&Vec<(i64,i64)>,start_index:usize,end_position:(i64,i64))->bool{

    let offsets = 0..red_points.len();

    for offset in offsets{
        let current_index = start_index+offset;
        let current_index = current_index % red_points.len();

        let current_position = red_points[current_index];
        //println!("Searching Up CW. Current_point_on_edge:{:?}",current_position);
        if current_position.1 >= end_position.1{//We moved right far enough to include our current point.
            return true;
        }
        else if current_position.0 > end_position.0 {
            return false;//Along the way, we ended up moving down before passing our current point
        }
    }

    false
}

fn can_move_up_ccw(red_points:&Vec<(i64,i64)>,start_index:usize,end_position:(i64,i64))->bool{

    let offsets = (0..red_points.len());//.rev();

    for offset in offsets{
        //let current_index = start_index+offset;
        //let current_index = current_index % red_points.len();
        let current_index =   (start_index + red_points.len() - offset) % red_points.len();

        let current_position = red_points[current_index];
        //println!("Searching UP CCW. Current_point_on_edge:{:?}",current_position);
        if current_position.1 >= end_position.1{//We moved right far enough to include our current point.
            return true;
        }
        else if current_position.0 < end_position.0 {
            return false;//Along the way, we ended up moving down before passing our current point
        }
    }

    false
}

fn can_move_down_cw(red_points:&Vec<(i64,i64)>,start_index:usize,end_position:(i64,i64))->bool{
    
    let offsets = (0..red_points.len());

    for offset in offsets{
        let current_index = start_index+offset;
        let current_index = current_index % red_points.len();

        let current_position = red_points[current_index];
        //println!("Searching DOWN CW. Current_point_on_edge:{:?}",current_position);
        if current_position.1 <= end_position.1{//We moved left far enough to include our current point.
            return true;
        }
        else if current_position.0 < end_position.0{
            return false;//Along the way, we ended up moving down before passing our current point
        }
    }
    false
}

fn can_move_down_ccw(red_points:&Vec<(i64,i64)>,start_index:usize,end_position:(i64,i64))->bool{
    
    let offsets = (0..red_points.len());//.rev();

    for offset in offsets{
        //let current_index = start_index+offset;
        //let current_index = current_index % red_points.len();
        let current_index =   (start_index + red_points.len() - offset) % red_points.len();

        let current_position = red_points[current_index];
        //println!("Searching DOWN CCW. Current_point_on_edge:{:?}",current_position);
        if current_position.1 <= end_position.1{//We moved left far enough to include our current point.
            return true;
        }
        else if current_position.0 > end_position.0{
            return false;//Along the way, we ended up moving down before passing our current point
        }
    }
    false
}






fn can_move_clockwise_to_point(red_points:&Vec<(i64,i64)>,start_index:usize,end_index:usize)->(bool,(i64,i64),(i64,i64),(i64,i64),(i64,i64)){


    //There are 4 possiblities.
    //Each C stands for the remaining corner. There is always 2 remaining corners because each rect has 4 corners.
    //The first C will be C_1. It is for the corner on the top row
    //the Second C will be C_2. It is the corner on the bottome row
    //DOWN_RIGHT
    //SC     Search Right(CW) and Down(CCW) From Start
    //CE     Search  LEFT(CW) and UP  (CCW) From End

    //UP_LEFT
    //EC     Search Right(CW) and Down(CCW) From End
    //CS     Search  LEFT(CW) and UP  (CCW) From Start

    //DOWN_LEFT
    //CS     Search  Left(CCW)  and Down(CW) from Start
    //EC     Search RIGHT(CCW)  and Up  (CW) from End

    //UP_RIGHT
    //CE     Search  Left(CCW)  and Down(CW) from End
    //SC     Search RIGHT(CCW)  and Up  (CW) from Start

    //Others possiblities, but I dont care for now about one wide
    //SE RIGHT

    //ES LEFT

    //S  DOWN
    //E

    //E  UP
    //S

    //(SE) NONE


    let c_s = red_points[start_index].clone();
    let c_e =   red_points[end_index].clone();
    //let right_top_corner = (start_corner.0,   end_corner.1);
    //let left_bot_corner =  (  end_corner.0, start_corner.1);


    //Step 1. Get direction from Start To END
    let need_to_travel = calc_dist_tup(&c_s,&c_e);
    let dist_sign = sgn_tup(&need_to_travel);
    let start_to_end_direction:TravelDirection= dist_sign.clone().into();
    //println!("We need to travel {:?}. To Go From {:?} to {:?}. {:?}",start_to_end_direction,c_s,c_e,need_to_travel);

    match start_to_end_direction{
        TravelDirection::UpRight=>{
            //UP_RIGHT
            //CE     Search  Left(CCW)  and Down(CW) from End
            //SC     Search RIGHT(CCW)  and Up  (CW) from Start
            let c_1 = (c_s.0,c_e.1);
            let c_2 = (c_e.0,c_s.1);
            let a = can_move_left_ccw(red_points, end_index, c_1);
            let b = can_move_down_cw(red_points, end_index, c_2);
            let c = can_move_up_cw(red_points, start_index, c_1);
            let d = can_move_right_ccw(red_points, start_index, c_2);
            let e = a&&b&&c&&d;
            //if e{
            //    println!("Four Conditions are {a}, {b}, {c}, {d}. Can we move there {e}");
            //}
            (e,c_1,c_e,c_2,c_s)
        },
        TravelDirection::UpLeft=>{
            //UP_LEFT
            //EC     Search Right(CW) and Down(CCW) From End
            //CS     Search  LEFT(CW) and UP  (CCW) From Start
            let c_1 = (c_s.0,c_e.1);
            let c_2 = (c_e.0,c_s.1);
            let a = can_move_right_cw(red_points, end_index, c_1);
            let b = can_move_down_ccw(red_points, end_index, c_2);
            let c = can_move_left_cw(red_points, start_index, c_1);
            let d = can_move_up_ccw(red_points, start_index, c_2);
            let e = a&&b&&c&&d;
            //if e{
            //    println!("Four Conditions are {a}, {b}, {c}, {d}. Can we move there {e}");
            //}
            (e,c_1,c_e,c_2,c_s)
        },
        TravelDirection::DownRight=>{
            //DOWN_RIGHT
            //SC     Search Right(CW) and Down(CCW) From Start
            //CE     Search  LEFT(CW) and UP  (CCW) From End
            let c_1 = (c_e.0,c_s.1);
            let c_2 = (c_s.0,c_e.1);


            let a = can_move_right_cw(red_points, start_index, c_1);
            let b = can_move_down_ccw(red_points, start_index, c_2);
            let d = can_move_left_cw(red_points, end_index, c_2);
            let c = can_move_up_ccw(red_points, end_index, c_1);
            let e = a&&b&&c&&d;
            //if e{
            //    println!("Four Conditions are {a}, {b}, {c}, {d}. Can we move there {e}");
            //}
            (e,c_1,c_e,c_2,c_s)
        },
        TravelDirection::DownLeft=>{
            //DOWN_LEFT
            //CS     Search  Left(CCW)  and Down(CW) from Start
            //EC     Search RIGHT(CCW)  and Up  (CW) from End
            let c_1 = (c_e.0,c_s.1);
            let c_2 = (c_s.0,c_e.1);

            let a = can_move_left_ccw(red_points, start_index, c_1);
            let b = can_move_down_cw(red_points, start_index, c_2);
            let c = can_move_up_cw(red_points, end_index, c_1);
            let d = can_move_right_ccw(red_points, end_index, c_2);
            let e = a&&b&&c&&d;
            //if e{
            //    println!("Four Conditions are {a}, {b}, {c}, {d}. Can we move there {e}");
            //}
            (e,c_1,c_e,c_2,c_s)
        },
        _=>{            (false,(0,0),(0,0),(0,0),(0,0))},
        /* 
        TravelDirection::UP=>{},
        TravelDirection::DOWN=>{},
        TravelDirection::LEFT=>{},
        TravelDirection::RIGHT=>{},
        TravelDirection::NONE=>{},
        TravelDirection::ERR=>{},
        */
    }

}




fn get_boarder_points(red_points:&Vec<Vec<i64>>)->Vec<Vec<i64>>{
    //let green_points:Vec<Vec<i64>> = Vec::new();
    let mut red_green_points:Vec<Vec<i64>> = Vec::new();
    let mut window_iter = red_points.windows(2);
    while let Some([a,b]) = window_iter.next(){
        red_green_points.push(a.clone());
        //println!("{:?} {:?}",a, b);
        let diff = calc_dist(a,b);
        let diff_mag = abs_vec(&diff.clone());
        let diff_sgn = sgn_vec(&diff.clone());
        //println!("{:?}",diff);
        //println!("{:?}",diff_mag);
        //println!("{:?}",diff_sgn);
        for (idx,sgn) in diff_sgn.iter().enumerate(){
            //println!("{idx},{:?}",sgn);
            match sgn{
                sgn @ -1 |sgn@ 1=>{
                    if diff_mag[idx]>1{
                        let mut start_pos = a.clone();
                        start_pos[idx]+=sgn;
                        for _ in 0..(diff_mag[idx]-1){
                            red_green_points.push(
                                start_pos.clone()
                            );
                            start_pos[idx]+=sgn;
                        }
                    }
                },
                0=>{},

                _=>unreachable!()
            }
        } 

    }
    red_green_points
}






fn calc_area(pt_a:(i64,i64),pt_b:(i64,i64))->usize{
    let width = (pt_a.0 - pt_b.0).abs()+1;
    let height = (pt_a.1 - pt_b.1).abs()+1;
    let area = width*height;
    //println!("area {}",area);
    area as usize
}