extern crate alexbox;

use warp::{
    filters::BoxedFilter,
    Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("config")
        .boxed()
}

pub fn get_config() -> BoxedFilter<()> {
    warp::get() // 1. Only accept GET
        .and(path_prefix()) // 2. That starts with /config
        .boxed()
}