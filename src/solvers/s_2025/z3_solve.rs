

use itertools::Itertools;
use z3::{self,ast};

use crate::solvers::s_2023::day_2::solution;




fn  calc_light_state(btns:&Vec<Vec<u16>>,press_count:&Vec<i64>,result_size:usize){
    let mut result = Vec::new();
    for _ in 0..result_size{
        result.push(0);
    }
    let mut result_joltage = Vec::new();
    for _ in 0..result_size{
        result_joltage.push(0);
    }

    //println!("filled result:{:?}",result);
    for (idx,count) in press_count.iter().enumerate(){
        for press in 0..*count{
            let lights = btns[idx].clone();
            //println!("Pressing BTN {:?}",lights);
            for light in lights{
                result[light as usize]+=1;
                result_joltage[light as usize]^=1;
            }
            //println!("Current Lights:{:?}",result_joltage);

        }
    }

    println!("before mod:{:?}",result);
    let result = result.iter().map(|f|f%2).collect_vec();
    println!("After mod:{:?}",result);
}

pub fn z3_day_10(buttons:&Vec<Vec<u16>>,joltage_rec:&Vec<u16>,light_state:&Vec<u16>)->i64{

    /*
    let mut buttons:Vec<Vec<u16>> = Vec::new();
    buttons.push( vec![3]) ;  //0
    buttons.push( vec![1,3]) ;//1
    buttons.push( vec![2]) ;  //2
    buttons.push( vec![2,3]) ;//3
    buttons.push( vec![0,2]) ;//4
    buttons.push( vec![0,1]) ;//5
    */



    //let joltage_rec:Vec<u16>= vec![3,5,4,7];
    //let light_state:Vec<u16>= vec![0,1,1,0];
    //let solution = vec![1,3,0,3,1,2];
    //calc_light_state(&buttons,&solution,light_state.len());
    //return;
    //let light_state:Vec<u16>= vec![0,1,1,0];

    let mut ast_solution_btn_count:Vec<ast::Int> = Vec::new(); 
    for i in 0..buttons.len(){
        ast_solution_btn_count.push(ast::Int::new_const(format!("BTN_{i}_count")));
    }


    //let solver = z3::Solver::new();
    let solver = z3::Optimize::new();
    let total_buttons_pressed_ast: z3::ast::Int = ast_solution_btn_count.iter().fold(z3::ast::Int::from_u64(0),|acc: ast::Int,e: &ast::Int|acc+e);

    let min_button_presses = joltage_rec.iter().max().unwrap();
    
    //We know we need at least a minimum number of presses to get to the solution
    solver.assert(&total_buttons_pressed_ast.gt(*min_button_presses-1));    
    //We know that we cant have negative button presses
    for btn_ast in ast_solution_btn_count.iter(){
        solver.assert(&btn_ast.gt(-1));//Only Positive answers
    }

    //solver.assert(&total_buttons_pressed_ast.lt(11));//Force it for now
    solver.minimize(&total_buttons_pressed_ast);//Force it for now


    
    //This creates the equations to check joltage_requirements
    for jolt_index in 0..joltage_rec.len(){
        let mut running_sum = ast::Int::from_u64(0);
        for button_index in 0..buttons.len(){
            if buttons[button_index].contains(&(jolt_index as u16)){
                //println!("Button {} controls slot {}",button_index,jolt_index);
                running_sum+= (&ast_solution_btn_count[button_index]);
            }
        }
        solver.assert(&running_sum.eq(joltage_rec[jolt_index]));
        //println!("{} The running sum is: {:?}. Adds to {}",jolt_index,running_sum,joltage_rec[jolt_index]);
    }
    
    /*
    //Now we must add in light state requirements
    for light_index in 0..light_state.len(){
        let mut running_sum = ast::Int::from_u64(0);
        for button_index in 0..buttons.len(){
            if buttons[button_index].contains(&(light_index as u16)){
                //println!("Button {} controls slot {}",button_index,jolt_index);
                running_sum+=((&ast_solution_btn_count[button_index])%2);
            }
        }
        //We are almost there. 
        //0+
        running_sum%=2;
        solver.assert(running_sum.eq(light_state[light_index]));
        println!("{} The Mod sum is: {:?}. Adds to {}",light_index,running_sum,light_state[light_index]);
    }
    */

    /*
    for solution in solver
        .solutions(ast_solution_btn_count.clone(), true)
        //.take_while(|x|)
        {
        // we use take to ensure that this loop terminates in case there are very many solutions
        //.filter(|x|Some(x)){
        // extract concrete values for each modeled Int Ast
        let solution: Vec<i64> = solution.iter().map(ast::Int::as_i64).filter_map(|x|{x}).collect();
        println!("{:?}",solution);
        calc_light_state(&buttons,&solution,light_state.len());

    }
    */
    let p = solver.check(&[]);
    //println!("{:?}",p);
    if let z3::SatResult::Sat = p{
        let model = solver.get_model().unwrap();

        //let model = model.eval(ast, true);
        let end_result = ast_solution_btn_count.clone().iter().filter_map(|f| {model.eval(f,true)}).filter_map(|x| x.as_i64()).collect_vec();
        let total_count = end_result.clone().iter().fold(0, |acc,e|acc+e);
        //println!("Our result is {:?}. Total_count {}",end_result, total_count);
        return total_count;
        //calc_light_state(&buttons,&end_result,light_state.len());

    }

    0
}




#[test]
fn z3_test_main(){
    let alice = ast::Int::fresh_const("alice");
    let bob = ast::Int::fresh_const("bob");
    let charlie = ast::Int::fresh_const("charlie");
    // instantiate a Solver
    let solver = z3::Solver::new();
    solver.assert((&alice + &bob + &charlie).eq(30));
    solver.assert(alice.gt(0));
    solver.assert(bob.gt(0));
    solver.assert(charlie.gt(0));
    solver.assert(alice.ge(5));
    solver.assert((&bob % 2).eq(0));
    solver.assert(charlie.ge(&bob));
    solver.assert(alice.eq(&charlie * 3));


    for solution in solver
        .solutions([alice, bob, charlie], false)
        // we use take to ensure that this loop terminates in case there are very many solutions
        .take(100) {
        // extract concrete values for each modeled Int Ast
        let solution: Vec<u64> = solution.iter().map(ast::Int::as_u64).map(Option::unwrap).collect();
        let alice = solution[0];
        let bob = solution[1];
        let charlie = solution[2];
        println!("alice: {alice}, bob: {bob}, charlie: {charlie}");
        // check that the concrete values match the constraints we gave the solver
        assert_eq!(alice + bob + charlie, 30);
        assert!(alice >= 5);
        assert_eq!(bob % 2, 0);
        assert!(charlie >= bob);
        assert_eq!(alice, charlie * 3);
    }
    /* 
 for solution in solver
        .solutions(ast_solution_btn_count.clone(), true)
        //.take_while(|x|)
        {
        // we use take to ensure that this loop terminates in case there are very many solutions
        //.filter(|x|Some(x)){
        // extract concrete values for each modeled Int Ast
        let solution: Vec<i64> = solution.iter().map(ast::Int::as_i64).filter_map(|x|{x}).collect();
        println!("{:?}",solution);
        calc_light_state(&buttons,&solution,light_state.len());

    }*/
}
