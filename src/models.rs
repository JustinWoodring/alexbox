extern crate diesel;
use self::super::schema::tiles;

#[derive(Queryable, Clone, AsChangeset)]
#[table_name = "tiles"]
#[primary_key("id")]
pub struct Tile {
    pub id: i32,
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