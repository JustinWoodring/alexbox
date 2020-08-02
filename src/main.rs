extern crate diesel;
extern crate alexbox;
extern crate xml;
use alexbox::establish_connection;
use alexbox::Config;
use warp::Filter;
use std::net::SocketAddr;
mod routes;

#[macro_use]
extern crate diesel_migrations;

embed_migrations!();
// Files in handlers/ are what implements "Result<impl warp::Reply, warp::Rejection>"
// It will be similar to controllers in Express and you will edit the folder most of time with models/
mod handlers; // This is the payload of this framework.
mod api; // get_tile! is usable with this in main.rs

use self::{
    routes::{
        playnow_route,
        tile_route,
        config_route
    },
    handlers::{
        playnow_handler,
        tile_handler,
        config_handler
    },
};


#[tokio::main]
async fn main() {
    let config = Config::get_config().expect("Failure to read config.");
    let server_address = config.server_address;
    let bind_port = config.bind_port;

    let connection = establish_connection();

    // This will run the necessary migrations.
    embedded_migrations::run(&connection).expect("Couldn't run migrations.");
    // Return react.
    let index = warp::fs::dir("static_files");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    /*let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));*/

    let routes = index.or((get_current_tile!()).or(get_tile!()).or(post_tile!()).or(put_tile!()).or(delete_tile!())).or(get_config!()).or(post_playnow!());

    let socket_addr : SocketAddr = (server_address.to_string()+":"+&bind_port).parse().unwrap();

    println!("Attempting to listen on {}:{}",server_address.as_str(),bind_port.as_str());
    println!("Starting up...");
    warp::serve(routes)
        .run(socket_addr)
        .await;
}
