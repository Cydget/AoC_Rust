pub mod download_day;
pub mod parse_input;
pub mod solvers;

use tokio;
use std::io;
use crate::solvers::Aoc;
pub mod file_utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        println!("Enter Session Token");
        dotenvy::dotenv()?;
        let mut user_input_text = String::new();
        //let user_input_res = std::io::stdin().read_line(&mut user_input_text);
        let session_string =             &std::env::var("session").unwrap();

        let cli = download_day::build_cli_with_sess(&session_string).await?;
        //Lets try downloading a years of input
        for y in 2016..=2017{
        for i in 1..=24{
            let res = download_day::download_day(&cli, y, i);
            res.await?;
        }}

        Ok(())
    }


    #[tokio::test]
    async fn download_single_day() -> Result<(), Box<dyn std::error::Error>> {
        let year = 2025;
        let day = 9;
        dotenvy::dotenv()?;
        let session_string =             &std::env::var("session").unwrap();
        let cli = download_day::build_cli_with_sess(&session_string).await?;
        download_day::download_day(&cli, year, day).await?;

        Ok(())
    }



    #[test]
    fn test_parse(){
        //let input = parse_input::read_input_file(2022, 1).expect("Should open file");
        //let input_line_count = parse_input::get_string_line_count(&input);
        //parse_input::parse_basic_numbers::<i64>(&input, input_line_count);
    }

    #[test]
    fn solve_day()->Result<(), Box<dyn std::error::Error>>{
        dotenvy::dotenv()?;

        let mut day_solver = solvers::s_2025::day_9::solution::new();
        //day_solver.solve_part_1_demo()?;
        //day_solver.solve_part_2_demo()?;
        //day_solver.check_part_2();
        //day_solver.solve_part_1("....\n@@@@\n@@@.\n....")?;
        //day_solver.solve_part_2("4,4\n5,4\n5,3\n3,3\n3,9\n8,9\n8,3\n7,3\n7,8\n4,8")?;
        //day_solver.solve_day()?;
        //370443228 is too low for part 2 day 9
        //1560299548
        day_solver.check_solutions();
        Ok(())
    }

}
