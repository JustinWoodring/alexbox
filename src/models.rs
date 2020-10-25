extern crate diesel;
use self::super::schema::tiles;

#[derive(Queryable, Clone, AsChangeset)]
#[table_name = "tiles"]
#[primary_key("id")]
pub struct Tile {
    pub id: i32,
    pub title: String,
    pub command: String,
    pub day: i32,
    pub start_time: String,
    pub duration: f32,
    pub color: i32
}