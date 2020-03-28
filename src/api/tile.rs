#[macro_export]
macro_rules! get_tile {
    () => {
        tile_route::get_tile()
        .and_then(tile_handler::get_tile)
    }
}

#[macro_export]
macro_rules! post_tile {
    () => {
        tile_route::post_tile()
        .and_then(tile_handler::post_tile)
    }
}

#[macro_export]
macro_rules! put_tile {
    () => {
        tile_route::put_tile()
        .and_then(tile_handler::put_tile)
    }
}

#[macro_export]
macro_rules! delete_tile {
    () => {
        tile_route::delete_tile()
        .and_then(tile_handler::delete_tile)
    }
}