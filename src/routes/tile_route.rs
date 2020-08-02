extern crate alexbox;
use std::collections::HashMap;

use warp::{
    filters::BoxedFilter,
    Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("tile")
        .boxed()
}

pub fn get_current_tile() -> BoxedFilter<()> {
    warp::get() // 1. Only accept GET
        .and(path_prefix())
        .and(warp::path("current").boxed()) // 2. That starts with /tile
        .boxed()
}

pub fn get_tile() -> BoxedFilter<()> {
    warp::get() // 1. Only accept GET
        .and(path_prefix()) // 2. That starts with /tile
        .boxed()
}

pub fn post_tile() -> BoxedFilter<(Vec<HashMap<String, String>>, )> {
    warp::post() // 1. Only accept POST
        .and(path_prefix()) // 2. That starts with /tile
        .and(warp::body::content_length_limit(1024 * 8))
        .and(warp::body::json())
        .boxed()
}

pub fn put_tile() -> BoxedFilter<(Vec<HashMap<String, String>>, )> {
    warp::put() // 1. Only accept PUT
        .and(path_prefix()) // 2. That starts with /tile
        .and(warp::body::content_length_limit(1024 * 8))
        .and(warp::body::json())
        .boxed()
}

pub fn delete_tile() -> BoxedFilter<(Vec<HashMap<String, String>>, )> {
    warp::delete() // 1. Only accept DELETE
        .and(path_prefix()) // 2. That starts with /tile
        .and(warp::body::content_length_limit(1024 * 1))
        .and(warp::body::json())
        .boxed()
}