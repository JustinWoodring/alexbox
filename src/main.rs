extern crate diesel;
extern crate alexbox;
use alexbox::establish_connection;
use warp::Filter;
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
        tile_route,
    },
    handlers::{
        tile_handler
    },
};


#[tokio::main]
async fn main() {
    let connection = establish_connection();

    // This will run the necessary migrations.
    embedded_migrations::run(&connection).expect("Couldn't run migrations.");
    // Return react.
    let index = warp::fs::dir("static_files");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    /*let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));*/

    let routes = index.or(get_tile!()).or(post_tile!()).or(put_tile!()).or(delete_tile!());
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], 1987))
        .await;
}
