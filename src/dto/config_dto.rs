
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetConfigDTO {
    pub ui_name: String,
    pub ui_logo: String,
    pub ui_colors: Vec<(String, String)>
}