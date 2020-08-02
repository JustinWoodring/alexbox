extern crate alexbox;
use alexbox::Config;
use warp;
use std::process::Command;
use std::collections::HashMap;


pub async fn post_playnow(input : HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(command) = input.get("command"){
        let config = Config::get_config().expect("Error accessing the config.");

        println!("");
        println!("-Play Now Command Received-");
        println!("---");
        println!("Command:");
        println!("{}{}{}",config.command_start,command,config.command_end);
        let result = Command::new("sh").args(&["-c",&format!("{}{}{}",config.command_start,command,config.command_end)]).status();
        println!("---");
        println!("Result:");
        println!("{:?}", result);

        return Ok(warp::reply::with_status(warp::reply::json(&String::from("Here goes nothing.")), warp::http::StatusCode::OK))
    }else{
        return Ok(warp::reply::with_status(warp::reply::json(&String::from("Missing command.")), warp::http::StatusCode::BAD_REQUEST))
    }
}
