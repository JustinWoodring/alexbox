pub mod schema;
pub mod models;
pub mod dto;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub struct Time {
    time : i32 //Store time doubled to avoid floats.
}

impl Time{
    /*
    The Time struct doesn't actually store time. 
    Just used to convert between an ISO format and the web format. 
    And it only really cares about the hour/minute
    */
    pub fn new(time : &str) -> Option<Time>{
        let mut is_ok = true;
        let split_time: Vec<&str> = time.split(" ").collect();
        if split_time.len() == 2{
            //YYYY-MM-DD
            let split_time_a: Vec<&str> = split_time.get(0).unwrap().split("-").collect();
            if !(split_time_a.len() == 3){
                is_ok=false;
                println!("Fail part a len");
            }else{
                //YYYY
                if let Ok(value) = split_time_a.get(0).unwrap().parse::<i32>(){
                    if !(value>=0&&value<=9999&&split_time_a.get(0).unwrap().len()==4){
                        is_ok=false;
                        println!("Fail year");
                    }
                }else{is_ok=false; println!("Fail year");}
                //MM
                if let Ok(value) = split_time_a.get(1).unwrap().parse::<i32>(){
                    if !(value>=1&&value<=12&&split_time_a.get(1).unwrap().len()==2){
                        is_ok=false; println!("fail month");
                    }
                }else{is_ok=false; println!("fail month");}
                //DD
                if let Ok(value) = split_time_a.get(2).unwrap().parse::<i32>(){
                    if !(value>=1&&value<=31&&split_time_a.get(2).unwrap().len()==2){
                        is_ok=false; println!("fail day");
                    }
                }else{is_ok=false; println!("fail day");}
            }
            //HH:MM:SS
            let split_time_b: Vec<&str> = split_time.get(1).unwrap().split(":").collect();
            if !(split_time_b.len() == 3){
                is_ok=false; println!("fail part b len");
            }else{
                //HH
                if let Ok(value) = split_time_b.get(0).unwrap().parse::<i32>(){
                    if !(value>=0&&value<=23&&split_time_b.get(0).unwrap().len()==2){
                        is_ok=false; println!("fail hour");
                    }
                }else{is_ok=false; println!("fail hour");}
                //MM
                if let Ok(value) = split_time_b.get(1).unwrap().parse::<i32>(){
                    if !((value==0||value==30)&&split_time_b.get(1).unwrap().len()==2){
                        is_ok=false; println!("fail minute");
                    }
                }else{is_ok=false; println!("fail minute");}
                //SS
                if let Ok(value) = split_time_b.get(2).unwrap().parse::<i32>(){
                    if !(value==0&&split_time_b.get(2).unwrap().len()==2){
                        is_ok=false; println!("fail second");
                    }
                }else{is_ok=false; println!("fail second");}
            }

            if is_ok {
                let mut calc_time = split_time_b.get(0).unwrap().parse::<i32>().unwrap();
                if split_time_b.get(1).unwrap().parse::<i32>().unwrap()==30 {
                    calc_time = (2*calc_time)+1;
                    return Some(Time {time: calc_time});
                }else{
                    calc_time = 2*calc_time;
                    return Some(Time {time: calc_time});
                }
            }
        }
        println!("Fail inital len");
        return None;
    }

    //Straightforward
    pub fn to_string(&self) -> String{
        if(&self.time % 2)==1{
            
            return format!("1970-01-01 {}:30:00", &self.time/2);
        }else{
            return format!("1970-01-01 {}:00:00", &self.time/2);
        }
    }

    //Floaty goodness
    pub fn to_float(&self) -> f32{
        return (self.time as f32) / 2.0;
    }

    //Impose basic reqs on duration.
    pub fn is_duration_sane(&self, duration: f32) -> bool{
        if (((duration*2.0) as i32) + self.time)>48 || (((duration*2.0) as i32) + self.time)<0 || duration <= 0.0 {
            return false;
        } else{
            return true
        }
    }
}