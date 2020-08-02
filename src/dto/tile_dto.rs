use serde::{Deserialize, Serialize};

extern crate diesel;
use self::super::super::schema::tiles;

//DTOs
#[derive(Deserialize, Serialize)]
 pub struct GetTileDTO {
    pub id: i32,
    pub title: String,
    pub mpv: String,
    pub prempv: String,
    pub postmpv: String,
    pub loopmpv: String,
    pub shufflempv: String,
    pub day: i32,
    pub time: f32,
    pub duration: f32,
    pub color: i32
}

#[derive(Deserialize, Serialize)]
 pub struct GetCurrentTileDTO {
    pub id: i32,
    pub title: String,
    pub mpv: String,
    pub time: f32,
    pub duration: f32,
    pub systemTime: String,
}


#[derive(Deserialize, Serialize, Insertable)]
#[table_name="tiles"]
pub struct PostTileDTO {
    pub title: String,
    pub mpv: String,
    pub prempv: String,
    pub postmpv: String,
    pub loopmpv: String,
    pub shufflempv: String,
    pub day: i32,
    pub time: String,
    pub duration: f32,
    pub color: i32
}