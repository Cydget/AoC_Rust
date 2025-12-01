
use reqwest::{self, Client, Url, cookie::{ Jar}};
use std::{fs::{self, File, read}, io::{self, Read, Write}};

use reqwest_cookie_store;
use cookie;

extern crate soup;
use soup::prelude::*;
use itertools::Itertools;


pub async  fn build_cli_with_sess(sess:&str)->Result<Client,io::Error>{

    let cookie = format!("session={sess}; Domain=.adventofcode");
    let url = "https://adventofcode.com".parse::<Url>().unwrap();
    let mut cookie_store = reqwest_cookie_store::CookieStore::new();
    let raw_cookie = cookie::Cookie::new("session", sess);
    cookie_store.insert_raw(&raw_cookie, &url).expect("How is cookie broken??");

    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);
    {
        let store = cookie_store.lock().unwrap();
        for c in store.iter_any() {
            println!("{:?}", c);
        }
    }


    static APP_USER_AGENT: &str = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
    );
    
    println!("{:?}",cookie);
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        //.cookie_store(true)
        .cookie_provider(cookie_store)
        .build().map_err(|e: reqwest::Error|  io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
    

    Ok(client)

}



pub async fn download_input_file(rq_client:&reqwest::Client,year:i32,day:i32)->Result<(),io::Error>{
    
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let body = rq_client.get(url).send()
        .await.map_err(|e: reqwest::Error|  io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .text()
        .await.map_err(|e: reqwest::Error|  io::Error::new(io::ErrorKind::Other, e.to_string()))?;


    let file_path = format!("./inputs/{year}/{day}");
    fs::create_dir_all(&file_path)?;
    let file_full_path = format!("{file_path}/input.txt");
    let mut file = File::create(file_full_path)?;
    file.write_all(body.as_bytes())?;
    println!("We downloaded the day's input file");
    Ok(())

}




pub fn save_to_file(text:&str,file_name:&str,file_path:&str)->Result<(),io::Error>{
    //println!("Saving to file");
    fs::create_dir_all(&file_path)?;
    let file_full_path = format!("{file_path}/{file_name}");
    let mut file = File::create(file_full_path)?;
    //println!("writing file");
    file.write_all(text.as_bytes())?;
    Ok(())
}




pub async fn download_code_blocks(rq_client:&reqwest::Client,year:i32,day:i32)->Result<(),io::Error>{
    
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    let body = rq_client.get(url)
        .send()
        .await.map_err(|e: reqwest::Error|  io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .bytes()
        .await.map_err(|e: reqwest::Error|  io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let soup = Soup::from_reader(body.as_ref());
    if let Ok(s) = soup{
        let _ = s.tag("code").find_all().
        sorted_by(|x,y|{
            y.text().len().cmp(&x.text().len())
        })
        .enumerate()
        .for_each(|t|{
            //println!("Found Code block,{}",t.1.text());
            let save_text = t.1.text();
            let file_path = format!("./inputs/{year}/{day}/");
            let file_name = format!("codeblock_{}",t.0);
            let _ = save_to_file(&save_text,&file_name,&file_path).unwrap();
        });
    };
    Ok(())

}


pub async fn download_day(rq_client:&reqwest::Client,year:i32,day:i32)->Result<(),io::Error>{
    
   download_code_blocks(rq_client, year, day).await?;
   download_input_file(rq_client, year, day).await?;

   Ok(())
}
