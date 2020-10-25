extern crate alexbox;
use warp;
use std::process::Command;
use std::collections::HashMap;


pub async fn post_playnow(input : HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(command) = input.get("command"){
        println!("");
        println!("-Play Now Command Received-");
        println!("---");
        println!("Command:");
        println!("{}",command);
        let result = Command::new("sh").args(&["-c",&format!("{}",command)]).status();
        println!("---");
        println!("Result:");
        println!("{:?}", result);

        return Ok(warp::reply::with_status(warp::reply::json(&String::from("Here goes nothing.")), warp::http::StatusCode::OK))
    }else{
        return Ok(warp::reply::with_status(warp::reply::json(&String::from("Missing command.")), warp::http::StatusCode::BAD_REQUEST))
    }
}
