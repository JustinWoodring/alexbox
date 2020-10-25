extern crate alexbox;
use warp;
use std::process::Command;
use crate::alexbox::dto::service_dto::GetServiceDTO;


pub async fn get_scheduler() -> Result<impl warp::Reply, warp::Rejection> {
    let command = "systemctl status alexbox-scheduler";
    let result = Command::new("sh").args(&["-c",&format!("{}",command)]).output();
    if let Ok(result) = result{
        if result.status.success() {
            return Ok(warp::reply::with_status(warp::reply::json(&GetServiceDTO{name: "alexbox-scheduler".to_string(), running: true}), warp::http::StatusCode::OK))
        }
    }
    return Ok(warp::reply::with_status(warp::reply::json(&GetServiceDTO{name: "alexbox-scheduler".to_string(), running: false}), warp::http::StatusCode::OK))
}