#[macro_export]
macro_rules! get_scheduler {
    () => {
        service_route::get_scheduler()
        .and_then(service_handler::get_scheduler)
    }
}