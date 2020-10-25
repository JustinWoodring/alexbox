extern crate alexbox;
use alexbox::Config;
use alexbox::dto::config_dto::GetConfigDTO;

pub async fn get_config() -> Result<impl warp::Reply, warp::Rejection> {
    let config = Config::get_config().expect("Error accessing the config.");
    let response = GetConfigDTO {
        ui_name: config.ui_name,
        ui_logo: config.ui_logo,
        ui_colors: config.ui_colors
    };
    return Ok(warp::reply::json(&response));
}