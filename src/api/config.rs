#[macro_export]
macro_rules! get_config {
    () => {
        config_route::get_config()
        .and_then(config_handler::get_config)
    }
}