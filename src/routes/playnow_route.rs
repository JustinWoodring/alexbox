extern crate alexbox;
use std::collections::HashMap;

use warp::{
    filters::BoxedFilter,
    Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("playnow")
        .boxed()
}

pub fn post_playnow() -> BoxedFilter<(HashMap<String, String>, )> {
    warp::post() // 1. Only accept POST
        .and(path_prefix()) // 2. That starts with /playnow
        .and(warp::body::content_length_limit(1024 * 1))
        .and(warp::body::json())
        .boxed()
}