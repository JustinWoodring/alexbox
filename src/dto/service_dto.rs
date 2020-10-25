use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetServiceDTO{
    pub name: String,
    pub running: bool
}