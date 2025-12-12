use std::iter::zip;
use std::ops::Index;

use itertools::Itertools;
use petgraph::graph::UnGraph;

use crate::parse_input;
use crate::solvers::Aoc;
use crate::file_utils;

use ndarray::Array2;

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
        assert_eq!(self.part_1_solution,13);
        Ok(())
    }

    pub fn solve_part_2_demo(&mut self)->Result<(),std::io::Error>{
        let input = file_utils::read_code_block(solution::get_year(),solution::get_day(),0).expect("Unable to open file");
        self.solve_part_2(&input)?;
        assert_eq!(self.part_2_solution,43);

        //let input= format!("{input}\nR1000");
        //self.solve_part_2(&input)?;
        //assert_eq!(self.part_2_solution,16);

        Ok(())
    }

    pub fn check_solutions(&mut self){
        self.solve_day().unwrap();

        assert_eq!(self.part_1_solution,1460);
        assert_eq!(self.part_2_solution,9243);
    }

}

#[derive(Clone)]
struct nd_map_basic {
    data:ndarray::ArrayBase<ndarray::OwnedRepr<parse_input::grid_item_basic_char>, ndarray::Dim<[usize; 2]>, parse_input::grid_item_basic_char>,
}
impl nd_map_basic{


    pub fn inside_grid(&self,index:&(i64,i64))->bool{//row column
        let map_dim = self.data.dim();
        if index.0 < 0 ||
           index.1 < 0 ||
           index.0 as usize >= map_dim.0 ||
           index.1 as usize >= map_dim.1 {
            false
        }
        else{
            true
        }
    }

    pub fn get_directly_next_to_indices(&self,index:(usize,usize))->Vec<(usize, usize)>{
        let ret_vec = [
            (index.0 as i64+0,index.1 as i64+1),
            (index.0 as i64+0,index.1 as i64-1),
            (index.0 as i64+1,index.1 as i64+0),
            (index.0 as i64-0,index.1 as i64+0),
            ];
        let b = ret_vec.into_iter().filter(|f|{
            self.inside_grid(f)
        }).map(|f|{(f.0 as usize,f.1 as usize)});
        b.collect()
    }
    pub fn get_neighbor_indices(&self,index:(usize,usize))->Vec<(usize, usize)>{
        //let indexed_node = self.data.index(index);
        let map_shape = self.data.ndim();
        let dims = 0..map_shape;

        let dim_1 = -1..=1;
        let dim_2 = -1..=1;

        let a = dim_1.map(|x|{
            (x,dim_2.clone())
        });
        let maps = a.map(|(row,cols)|{
            cols.map(move |col|{
                (row,col)
            })
        }).flatten();

        let index_candidates = maps.map(|(row_offset,col_offset)|{
            (index.0 as i64+row_offset,index.1 as i64 +col_offset)
        });

        let good_indexes = index_candidates.filter(|f: &(i64, i64)|
            self.inside_grid(f)
        ).map(|f|{(f.0 as usize,f.1 as usize)})
        .filter(|f| {f.ne(&(index.0,index.1))});
        let return_vec: Vec<(usize, usize)> = good_indexes.collect_vec();
        return_vec
        //println!("Maps is {:?} ",good_indexes.collect_vec())
    }
}


fn print_map(rows:usize,cols:usize,points:Vec<(usize,usize)>){
    for r in 0..rows{
        for c in 0..cols{
            if points.contains(&(r,c)){
                print!("x");
            }
            else{
                print!("o");
            }
        }
        print!("\n");
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
        //input is a rectangular grid 

        //let indexed_data = parse_input::convert_text_grid_to_map(&input);
        //println!("The indexed data is{:?}",indexed_data);


        let arrayed_data = parse_input::convert_text_grid_to_ndarray(&input);
        //println!("The indexed data is{:?}",arrayed_data);
        let the_map = nd_map_basic{data:arrayed_data};

        let mut result = 0;
        let forklift_locations = the_map.data.iter()
        .filter(|f|{
            f.char_value.cmp(&'@').is_eq()
        })
        .filter(|this_square|{
            let neighbor_indexes = the_map.get_neighbor_indices((this_square.y as usize,this_square.x as usize));
            let paper_count:usize = neighbor_indexes.iter().filter(|nei|{
                let a = the_map.data.index(**nei).char_value.cmp(&'@');
                a.is_eq()
            }).count();
            paper_count<4
        });

        let forklift_indexes_iter = forklift_locations.clone().map(|x|{(x.y as usize,x.x as usize)});
        //println!("ForkLift Locations {:?}",forklift_indexes_iter.clone().collect_vec());
        //print_map(the_map.data.dim().0,the_map.data.dim().1,forklift_indexes_iter.clone().collect_vec());
        let result = forklift_indexes_iter.clone().unique().count();
        //println!("ForkLift Count {:?}",forklift_indexes_iter.clone().unique().count());

        /*
        let indexes_of_rolls_that_can_be_moves = forklift_locations.map(|this_square|{
            let neighbor_indexes: Vec<(usize, usize)> = the_map.get_directly_next_to_indices((this_square.y as usize,this_square.x as usize));
            let paper_locations_iter = neighbor_indexes.into_iter().filter(|nei|{
                let a = the_map.data.index(nei.clone()).char_value.cmp(&'@');
                a.is_eq()
            });
            let a = paper_locations_iter.collect_vec();
            a
        });//flatten().unique();
        println!("All Indexes of squares {:?}",indexes_of_rolls_that_can_be_moves.clone().collect_vec());
        let unique_iter = indexes_of_rolls_that_can_be_moves.flatten().unique();

        print_map(the_map.data.dim().0,the_map.data.dim().1,unique_iter.clone().collect_vec());

        println!("Unique {:?}",unique_iter.clone().collect_vec());
        println!("Unique Count {:?}",unique_iter.clone().count());
        */
        self.part_1_solution = result as u128;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }

    /*
    fn solve_part_1_old(&mut self,input:&str)->Result<(),std::io::Error>{

        println!("Input is:\n{}",input);
        //input is a rectangular grid 

        //let indexed_data = parse_input::convert_text_grid_to_map(&input);
        //println!("The indexed data is{:?}",indexed_data);


        let arrayed_data = parse_input::convert_text_grid_to_ndarray(&input);
        //println!("The indexed data is{:?}",arrayed_data);
        let the_map = nd_map_basic{data:arrayed_data};

        let mut result = 0;
        let forklift_locations = the_map.data.iter()
        //.filter(|f|{
        //    f.char_value.cmp(&'@').is_ne()
        //})
        .filter(|this_square|{
            let neighbor_indexes = the_map.get_neighbor_indices((this_square.y as usize,this_square.x as usize));
            let paper_count:usize = neighbor_indexes.iter().filter(|nei|{
                let a = the_map.data.index(**nei).char_value.cmp(&'@');
                a.is_eq()
            }).count();
            paper_count<4
        });

        let forklift_indexes_iter = forklift_locations.clone().map(|x|{(x.y as usize,x.x as usize)});
        println!("ForkLift Locations {:?}",forklift_indexes_iter.clone().collect_vec());
        print_map(the_map.data.dim().0,the_map.data.dim().1,forklift_indexes_iter.clone().collect_vec());
        println!("ForkLift Count {:?}",forklift_indexes_iter.clone().unique().count());


        let indexes_of_rolls_that_can_be_moves = forklift_locations.map(|this_square|{
            let neighbor_indexes: Vec<(usize, usize)> = the_map.get_directly_next_to_indices((this_square.y as usize,this_square.x as usize));
            let paper_locations_iter = neighbor_indexes.into_iter().filter(|nei|{
                let a = the_map.data.index(nei.clone()).char_value.cmp(&'@');
                a.is_eq()
            });
            let a = paper_locations_iter.collect_vec();
            a
        });//flatten().unique();
        println!("All Indexes of squares {:?}",indexes_of_rolls_that_can_be_moves.clone().collect_vec());
        let unique_iter = indexes_of_rolls_that_can_be_moves.flatten().unique();

        print_map(the_map.data.dim().0,the_map.data.dim().1,unique_iter.clone().collect_vec());

        println!("Unique {:?}",unique_iter.clone().collect_vec());
        println!("Unique Count {:?}",unique_iter.clone().count());

        self.part_1_solution = result as u128;
        println!("The solution for part 1 is: {}",self.part_1_solution);
        Ok(())
    }
*/





    fn solve_part_2(&mut self,input:&str)->Result<(),std::io::Error>{





        let arrayed_data = parse_input::convert_text_grid_to_ndarray(&input);
        //println!("The indexed data is{:?}",arrayed_data);
        let mut the_map = nd_map_basic{data:arrayed_data};
        let mut rolls_left = 0;
        let mut total_rolls = 0;
        
        let total_rolls_at_start = the_map.data.iter()
        .filter(|f|{
            f.char_value.cmp(&'@').is_eq()
        }).count();
        //println!("Start Roll Count {total_rolls_at_start}");

        loop {
            let imm_map = the_map.clone();
            let forklift_locations = imm_map.data.iter()
            .filter(|f|{
                f.char_value.cmp(&'@').is_eq()
            })
            .filter(|this_square|{
                let neighbor_indexes = imm_map.get_neighbor_indices((this_square.y as usize,this_square.x as usize));
                let paper_count:usize = neighbor_indexes.iter().filter(|nei|{
                    let a = imm_map.data.index(**nei).char_value.cmp(&'@');
                    a.is_eq()
                }).count();
                paper_count<4
            });
            //let mut the_map = the_map;
            
            
            
            let forklift_indexes_iter = forklift_locations.clone().map(|x|{(x.y as usize,x.x as usize)});

            //print_map(the_map.data.dim().0,the_map.data.dim().1,forklift_indexes_iter.clone().collect_vec());

            forklift_indexes_iter.clone().for_each(|inx|{
                the_map.data.get_mut(inx).unwrap().char_value='.';
            });
            let this_result = forklift_indexes_iter.clone().unique().count();
            //println!("Removed {this_result} rolls");
            total_rolls+=this_result;

            let rolls_left_at_start = imm_map.data.iter()
            .filter(|f|{
                f.char_value.cmp(&'@').is_eq()
            }).count();

            let rolls_left_at_end = the_map.data.iter()
            .filter(|f|{
                f.char_value.cmp(&'@').is_eq()
            }).count();

            
            let results_didnt_change= rolls_left_at_start==rolls_left_at_end;
            if results_didnt_change{
                break;
            }
        }
        //println!("total rolls {total_rolls}");
        //println!("ForkLift Locations {:?}",forklift_indexes_iter.clone().collect_vec());
        //print_map(the_map.data.dim().0,the_map.data.dim().1,forklift_indexes_iter.clone().collect_vec());
        let result = total_rolls;

        self.part_2_solution = result as u128;
        println!("The solution for part 2 is: {}",self.part_2_solution);
        Ok(())
    }


    fn get_input(&mut self)->Result<String,std::io::Error>{
        let result = file_utils::read_input_file(Self::get_year(), Self::get_day())?;
        Ok(result)
    }

    fn get_day()->i32{4}
    fn get_year()->i32{2025}
}
