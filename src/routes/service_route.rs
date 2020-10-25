extern crate alexbox;

use warp::{
    filters::BoxedFilter,
    Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("service")
        .boxed()
}

pub fn get_scheduler() -> BoxedFilter<()> {
    warp::get() // 1. Only accept GET
        .and(path_prefix())
        .and(warp::path("scheduler").boxed()) // 2. That starts with /tile
        .boxed()
}