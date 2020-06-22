#[macro_export]
macro_rules! post_playnow {
    () => {
        playnow_route::post_playnow()
        .and_then(playnow_handler::post_playnow)
    }
}