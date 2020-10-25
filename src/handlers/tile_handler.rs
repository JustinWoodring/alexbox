extern crate alexbox;
extern crate diesel;

use warp;
use self::alexbox::*;
use self::alexbox::Time;
use self::alexbox::dto::tile_dto::*;
use self::models::*;
use self::diesel::prelude::*;
use std::collections::HashMap;
use chrono::prelude::*;

/*
Please sir, I want some more tiles...
*/
pub async fn get_tile() -> Result<impl warp::Reply, warp::Rejection> {
    use alexbox::schema::tiles::dsl::*;
    let connection = establish_connection();

    let results = tiles
        .load::<Tile>(&connection)
        .expect("Error loading posts");

    let mut response = Vec::new();

    for tile in results {
        if let Some(my_time) = Time::new(&tile.start_time){
            let tile: GetTileDTO = GetTileDTO {
                id: tile.id,
                title: tile.title,
                command: tile.command,
                day: tile.day,
                start_time: my_time.to_float(),
                duration: tile.duration,
                color: tile.color
            };
            response.push(tile);
        }
    }

    return Ok(warp::reply::json(&response));
}

pub async fn get_current_tile() -> Result<impl warp::Reply, warp::Rejection> {
    use alexbox::schema::tiles::dsl::*;
    let connection = establish_connection();

    let results = tiles
        .load::<Tile>(&connection)
        .expect("Error loading posts");
    
    //Get system time.
    let local: DateTime<Local> = Local::now();

    //Get current minute for creating localtime.
    let mut min = local.format("%M").to_string();

    if min.parse::<u32>().unwrap() < 30 {
        min = "00".to_string();
    }else {
        min = "30".to_string();
    }

    //Get local day (0-6) and time.
    let localday = local.format("%w").to_string();
    let localtime = Time::new(&format!("{}:{}:00",&local.format("%Y-%m-%d %H").to_string(),min)).unwrap();

    for tile in results {
        //If we can read time then
        if let Some(my_time) = Time::new(&tile.start_time){
            //If tile times match up to local time then
            if 
                my_time <= localtime && 
                localtime.to_float() <= (my_time.to_float()+tile.duration) && 
                localday.parse::<i32>().unwrap() == tile.day
            {
                //create dto
                let response: GetCurrentTileDTO = GetCurrentTileDTO {
                    id: tile.id,
                    title: tile.title,
                    command: tile.command,
                    start_time: my_time.to_float(),
                    duration: tile.duration,
                    system_time: local.format("%H:%M").to_string()
                };

                //return it.
                return Ok(warp::reply::json(&response));
            }
        }
    }


    //Or return nothing.
    return Ok(warp::reply::json(&()));
}


/*Posted content may or may not be an acceptable format
 and if it is then we have to check the database and array
 of submissions for overlap*/
pub async fn post_tile(input : Vec<HashMap<String, String>>) -> Result<impl warp::Reply, warp::Rejection> {
    use alexbox::schema::tiles::dsl::*;
    let connection = establish_connection();

    let mut new_tiles : Vec<PostTileDTO> = Vec::new();
    let mut ok = true;
    for tile in input {
        //new title
        let mut new_title = "";
        if let Some(value) = tile.get("title"){
            new_title = value;
            if new_title.len() > 50{
                ok=false;
            }
        }else{ok=false;}

        //new command
        let mut new_command = "";
        if let Some(value) = tile.get("command"){
            new_command = value;
            if new_command.len() > 1000{
                ok=false;
            }
        }else{ok=false;}

        //new day
        let mut new_day = -1;
        if let Some(value) = tile.get("day"){
            if let Ok(value) = value.parse::<i32>(){
                new_day = value;
            }else{ok=false;}
        }else{ok=false;}

        if !(new_day >=0 && new_day <=6){
            ok = false;
        }

        //new time
        let mut new_time = "";
        if let Some(value) = tile.get("start_time"){
            new_time = value;
        }else{ok=false;}
        //new duration
        let mut new_duration = -0.5;
        if let Some(value) = tile.get("duration"){
            if let Ok(value) = value.parse::<f32>(){
                new_duration = value;
            }else{ok=false;}
        }else{ok=false;}

        let mut new_color = 0;
        if let Some(value) = tile.get("color"){
            if let Ok(value) = value.parse::<i32>(){
                if value > 0 && value <= 20 {
                    new_color = value;
                }else{
                    ok=false;
                }
            }else{ok=false;}
        }else{ok=false;}

        if ok{
            match Time::new(new_time) {
                Some(new_time_struct) => {
                    //Run our conformance checks or return an error
                    if !(new_time_struct.is_duration_sane(new_duration)){
                        return Ok(warp::reply::with_status(warp::reply::json(&String::from("Duration is too long")), warp::http::StatusCode::BAD_REQUEST));
                    }
                }
                None =>{
                    return Ok(warp::reply::with_status(warp::reply::json(&String::from("Time is malformed")), warp::http::StatusCode::BAD_REQUEST));
                }
            }

            //What we will hopefully submit.
            let new = PostTileDTO {
                title : new_title.to_string(),
                command: new_command.to_string(),
                day: new_day,
                start_time: new_time.to_string(),
                duration: new_duration,
                color: new_color
            };

            //Compare against tiles in database if overlap return complaint.
            if let Ok(current_db_tiles) = tiles.order(start_time.desc()).load::<Tile>(&connection) {
                for db_tile in current_db_tiles {
                    let db_time = Time::new(&db_tile.start_time).unwrap(); //Database shouldn't contain malformed data. Cross my fingers.
                    let post_time = Time::new(&new.start_time).unwrap(); //The conformance tests on 102 let us unwrap safely.
                    if db_tile.day == new.day {
                        if post_time > db_time && (db_tile.duration + db_time.to_float()) > post_time.to_float() {
                            return Ok(warp::reply::with_status(warp::reply::json(&String::from("Some db tile overruns a submitted tile")), warp::http::StatusCode::CONFLICT));
                        }else if post_time < db_time && (new.duration + post_time.to_float()) > db_time.to_float() {
                            return Ok(warp::reply::with_status(warp::reply::json(&String::from("A submitted tile overruns some db tile")), warp::http::StatusCode::CONFLICT));
                        }
                        else if post_time == db_time{
                            return Ok(warp::reply::with_status(warp::reply::json(&String::from("A submitted tile starts at the same time as some db tile")), warp::http::StatusCode::CONFLICT));
                        }
                    }
                }
            }else{
                //Return error if we can't access database.
                return Ok(warp::reply::with_status(warp::reply::json(&"".to_string()), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
            }
            
            //Compare against other submitted tiles.
            for other_tile in &new_tiles{
                let other_time = Time::new(&other_tile.start_time).unwrap(); //If its in the array it was already conformance tested.
                let post_time = Time::new(&new.start_time).unwrap(); //If we are down here the same as above is true.
                if other_tile.day == new.day {
                    if post_time > other_time && (other_tile.duration + other_time.to_float()) > post_time.to_float() {
                        return Ok(warp::reply::with_status(warp::reply::json(&String::from("Some submitted tile overruns another submitted tile")), warp::http::StatusCode::CONFLICT));
                    }else if post_time < other_time && (new.duration + post_time.to_float()) > other_time.to_float() {
                        return Ok(warp::reply::with_status(warp::reply::json(&String::from("A submitted tile overruns some submitted tile")), warp::http::StatusCode::CONFLICT));
                    }
                    else if post_time == other_time{
                        return Ok(warp::reply::with_status(warp::reply::json(&String::from("A submitted tile starts at the same time as another submitted tile")), warp::http::StatusCode::CONFLICT));
                    }
                }
            }

            new_tiles.push(new);
        }
    }

    if ok {
        use schema::tiles::dsl::*;
        match diesel::insert_into(tiles).values(&new_tiles).execute(&connection){
            Ok(_) => {return Ok(warp::reply::with_status(warp::reply::json(&String::from("Successfully created tiles")), warp::http::StatusCode::CREATED))},
            Err(_) => {return Ok(warp::reply::with_status(warp::reply::json(&"".to_string()), warp::http::StatusCode::INTERNAL_SERVER_ERROR))}
        }
    }else{
        return Ok(warp::reply::with_status(warp::reply::json(&String::from("Something was invalid.")), warp::http::StatusCode::BAD_REQUEST));
    }
}


/*The PUT method is really complicated because 
1 we have to do the extensive data processing done in post and parse changes into Tile objects.
2 then we have to see if the records to be modified exist
3 then we have to see if they overlap potentially modified values from the database.
4 then we have to check the potential changes to see if they overlap with unmodified values from the database

Then finally we actually update the records.
*/
pub async fn put_tile(input : Vec<HashMap<String, String>>) -> Result<impl warp::Reply, warp::Rejection> {
    use alexbox::schema::tiles::dsl::*;
    let connection = establish_connection();
    let mut unprocessed_tiles1 : Vec<Tile> = Vec::new();
    let mut unprocessed_tiles2 : Vec<Tile> = Vec::new();
    let mut unprocessed_tiles3 : Vec<Tile> = Vec::new();
    let mut no_post_overlap_tiles : Vec<Tile> = Vec::new();
    let mut no_post_overlap_tiles2 : Vec<Tile> = Vec::new();
    let mut tiles_to_update : Vec<Tile> = Vec::new();

    //STEP 1 & 2 - Attempt to parse into Tile objects and see if the records actually exist.
    for tile in input{
        let mut tile_id = -1;
        if let Some(value) = tile.get("id"){
            if let Ok(value) = value.parse::<i32>(){
                tile_id = value;
            }else{return Ok(warp::reply::with_status(warp::reply::json(&"Id is improperly formatted".to_string()), warp::http::StatusCode::BAD_REQUEST));}
        }else{return Ok(warp::reply::with_status(warp::reply::json(&"Could not retrieve id from request".to_string()), warp::http::StatusCode::BAD_REQUEST));}

        let mut new_title = "";
        if let Some(value) = tile.get("title"){
            new_title = value;
            if new_title.len() > 50{
                return Ok(warp::reply::with_status(warp::reply::json(&String::from("Title is too long or too short")), warp::http::StatusCode::BAD_REQUEST));
            }
        }else{return Ok(warp::reply::with_status(warp::reply::json(&String::from("Title could not be found")), warp::http::StatusCode::BAD_REQUEST));}

        //new command
        let mut new_command = "";
        if let Some(value) = tile.get("command"){
            new_command = value;
            if new_command.len() > 1000{
                return Ok(warp::reply::with_status(warp::reply::json(&String::from("command command was too long or short")), warp::http::StatusCode::BAD_REQUEST));
            }
        }else{return Ok(warp::reply::with_status(warp::reply::json(&String::from("command command could not be found")), warp::http::StatusCode::BAD_REQUEST));}

        //new day
        let mut new_day = -1;
        if let Some(value) = tile.get("day"){
            if let Ok(value) = value.parse::<i32>(){
                new_day = value;
            }else{return Ok(warp::reply::with_status(warp::reply::json(&String::from("Could not be parse day")), warp::http::StatusCode::BAD_REQUEST));}
        }else{return Ok(warp::reply::with_status(warp::reply::json(&String::from("Could not retreive day")), warp::http::StatusCode::BAD_REQUEST));}

        if !(new_day >=0 && new_day <=6){
            return Ok(warp::reply::with_status(warp::reply::json(&String::from("Day is not a day")), warp::http::StatusCode::BAD_REQUEST));
        }

        //new time
        let mut new_time = "";
        if let Some(value) = tile.get("start_time"){
            new_time = value;
        }else{return Ok(warp::reply::with_status(warp::reply::json(&String::from("Could not find time")), warp::http::StatusCode::BAD_REQUEST));}
        //new duration
        let mut new_duration = -0.5;
        if let Some(value) = tile.get("duration"){
            if let Ok(value) = value.parse::<f32>(){
                new_duration = value;
            }else{return Ok(warp::reply::with_status(warp::reply::json(&String::from("Could not parse duration")), warp::http::StatusCode::BAD_REQUEST));}
        }else{return Ok(warp::reply::with_status(warp::reply::json(&String::from("Could not find duration")), warp::http::StatusCode::BAD_REQUEST));}

        let mut new_color = 0;
        if let Some(value) = tile.get("color"){
            if let Ok(value) = value.parse::<i32>(){
                if value > 0 && value <= 20 {
                    new_color = value;
                }else{
                    return Ok(warp::reply::with_status(warp::reply::json(&String::from("Could not parse color")), warp::http::StatusCode::BAD_REQUEST));
                }
            }else{return Ok(warp::reply::with_status(warp::reply::json(&String::from("Could not parse color")), warp::http::StatusCode::BAD_REQUEST));}
        }else{return Ok(warp::reply::with_status(warp::reply::json(&String::from("Could not retrieve color")), warp::http::StatusCode::BAD_REQUEST));}

        //Run time checks.
        match Time::new(new_time) {
            Some(new_time_struct) => {
                //Run our conformance checks or return an error
                if !(new_time_struct.is_duration_sane(new_duration)){
                    return Ok(warp::reply::with_status(warp::reply::json(&String::from("Duration is too long")), warp::http::StatusCode::BAD_REQUEST));
                }
            }
            None =>{
                return Ok(warp::reply::with_status(warp::reply::json(&String::from("Time is malformed")), warp::http::StatusCode::BAD_REQUEST));
            }
        }

        //See if the tile actually exists in the database.
        if let Ok(tile_ids) = tiles.select(id).filter(id.eq(tile_id)).load::<i32>(&connection) {
            if !(tile_ids.len() == 1){
                return Ok(warp::reply::with_status(warp::reply::json(&"Requested item isn't in the database!".to_string()), warp::http::StatusCode::NOT_FOUND));
            }
        }else{
            return Ok(warp::reply::with_status(warp::reply::json(&"".to_string()), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        //Turn it into a tile
        let new = Tile {
            id: tile_id,
            title : new_title.to_string(),
            command: new_command.to_string(),
            day: new_day,
            start_time: new_time.to_string(),
            duration: new_duration,
            color: new_color
        };

        let new2 = Tile {
            id: tile_id,
            title : new_title.to_string(),
            command: new_command.to_string(),
            day: new_day,
            start_time: new_time.to_string(),
            duration: new_duration,
            color: new_color
        };

        let new3 = Tile {
            id: tile_id,
            title : new_title.to_string(),
            command: new_command.to_string(),
            day: new_day,
            start_time: new_time.to_string(),
            duration: new_duration,
            color: new_color
        };

        if let Ok(current_db_tiles) = tiles.order(start_time.desc()).load::<Tile>(&connection) {
            let mut exists = false;
            for db_tile in current_db_tiles {
                if new.id==db_tile.id{
                    exists = true;
                }
            }
            if exists != true{
                return Ok(warp::reply::with_status(warp::reply::json(&"ID not found".to_string()), warp::http::StatusCode::BAD_REQUEST));
            }
        }else{
            //Return error if we can't access database.
            return Ok(warp::reply::with_status(warp::reply::json(&"".to_string()), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        //Stick it in the unprocessed tiles
        unprocessed_tiles1.push(new);
        unprocessed_tiles2.push(new2);
        unprocessed_tiles3.push(new3);
    }

    //STEP 3 - See if the tiles overlap modified database tiles.
    for tile in unprocessed_tiles1{
        for tile2 in &unprocessed_tiles2{
            if tile2.id != tile.id{
                let tile2_time = Time::new(&tile2.start_time).unwrap(); //Database shouldn't contain malformed data. Cross my fingers.
                let tile_time = Time::new(&tile.start_time).unwrap(); //The conformance tests on 102 let us unwrap safely.
                if tile2.day == tile.day {
                    if tile_time > tile2_time && (tile2.duration + tile2_time.to_float()) > tile_time.to_float() {
                        return Ok(warp::reply::with_status(warp::reply::json(&String::from("Some modified tile overruns another modified tile")), warp::http::StatusCode::CONFLICT));
                    }else if tile_time < tile2_time && (tile.duration + tile_time.to_float()) > tile2_time.to_float() {
                        return Ok(warp::reply::with_status(warp::reply::json(&String::from("A modified tile overruns some other modified tile")), warp::http::StatusCode::CONFLICT));
                    }
                    else if tile_time == tile2_time{
                        return Ok(warp::reply::with_status(warp::reply::json(&String::from("A modified tile starts at the same time as some other modified tile")), warp::http::StatusCode::CONFLICT));
                    }
                }
            }
        }
        no_post_overlap_tiles.push(tile);
    }
    for tile in unprocessed_tiles3{
        no_post_overlap_tiles2.push(tile);
    }

    //STEP 4
    //Cut it down database to include only unmodified values.
    let mut slimmed_db_tiles : std::vec::Vec<Tile> = Vec::new();
    if let Ok(current_db_tiles) = tiles.order(start_time.desc()).load::<Tile>(&connection) {
        for db_tile in current_db_tiles {
            let mut fail = false;
            for post_tile in &no_post_overlap_tiles2{
                if  db_tile.id == post_tile.id {
                    fail = true;
                }
            }
            if !fail {
                &slimmed_db_tiles.push(db_tile);
            }
        }
    }else{
        //Return error if we can't access database.
        return Ok(warp::reply::with_status(warp::reply::json(&"".to_string()), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
    }
    for tile in no_post_overlap_tiles{
        for db_tile in &slimmed_db_tiles{
            let db_time = Time::new(&db_tile.start_time).unwrap(); //Database shouldn't contain malformed data. Cross my fingers.
            let post_time = Time::new(&tile.start_time).unwrap(); //The conformance tests on 102 let us unwrap safely.
            if db_tile.day == tile.day {
                if post_time > db_time && (db_tile.duration + db_time.to_float()) > post_time.to_float() {
                    return Ok(warp::reply::with_status(warp::reply::json(&String::from("Some db tile overruns a modified tile")), warp::http::StatusCode::CONFLICT));
                }else if post_time < db_time && (tile.duration + post_time.to_float()) > db_time.to_float() {
                    return Ok(warp::reply::with_status(warp::reply::json(&String::from("A modified tile overruns some db tile")), warp::http::StatusCode::CONFLICT));
                }
                else if post_time == db_time{
                    return Ok(warp::reply::with_status(warp::reply::json(&String::from("A modified tile starts at the same time as some db tile")), warp::http::StatusCode::CONFLICT));
                }
            }
        }
        tiles_to_update.push(tile);
    }

    //UPDATE THE DATABASE
    for update in tiles_to_update{
        let response = diesel::update(tiles.filter(id.eq(update.id))).set(update).execute(&connection);
        match response{
            Ok(_) => {},
            Err(_) => {return Ok(warp::reply::with_status(warp::reply::json(&"Data may now be truncated".to_string()), warp::http::StatusCode::INTERNAL_SERVER_ERROR));}
        }
    }
    return Ok(warp::reply::with_status(warp::reply::json(&"If I did update something it went well.".to_string()), warp::http::StatusCode::OK));
}

//Only need that ID
pub async fn delete_tile(input : Vec<HashMap<String, String>>) -> Result<impl warp::Reply, warp::Rejection> {
    use alexbox::schema::tiles::dsl::*;
    let connection = establish_connection();
    let mut tiles_to_delete: Vec<i32> = Vec::new();

    //Checking the tiles for conformance.
    for tile in input{
        #[allow(unused_assignments)]
        let mut tile_id = -1;
        if let Some(value) = tile.get("id"){
            if let Ok(value) = value.parse::<i32>(){
                tile_id = value;
            }else{return Ok(warp::reply::with_status(warp::reply::json(&"Id is improperly formatted".to_string()), warp::http::StatusCode::BAD_REQUEST));}
        }else{return Ok(warp::reply::with_status(warp::reply::json(&"Could not retrieve id from delete request".to_string()), warp::http::StatusCode::BAD_REQUEST));}

        //Check to see if the same id is supposed to be deleted more than once.
        for tile_to_be_deleted_id in &tiles_to_delete{
            if &tile_id == tile_to_be_deleted_id {
                return Ok(warp::reply::with_status(warp::reply::json(&"Multiple delete requests try to delete the same id".to_string()), warp::http::StatusCode::BAD_REQUEST));
            }
        }

        //Check and see if IDs that should be deleted are even in database.
        if let Ok(tile_ids) = tiles.select(id).filter(id.eq(tile_id)).load::<i32>(&connection) {
            if !(tile_ids.len() == 1){
                return Ok(warp::reply::with_status(warp::reply::json(&"Requested item isn't in the database!".to_string()), warp::http::StatusCode::NOT_FOUND));
            }
        }else{
            return Ok(warp::reply::with_status(warp::reply::json(&"Couldn't check on IDs".to_string()), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        
        //Append to the deletion list.
        &tiles_to_delete.push(tile_id);
    }

    //Begin deleting - ONCE WE START THIS THINGS ARE IN MOTION THAT CAN'T BE UNDONE!!!
    for delete_id in tiles_to_delete{
        let response = diesel::delete(tiles.filter(id.eq(delete_id))).execute(&connection);
        match response{
            Ok(_) => {},
            Err(_) => {return Ok(warp::reply::with_status(warp::reply::json(&"Data may now be truncated".to_string()), warp::http::StatusCode::INTERNAL_SERVER_ERROR));}
        }
    }

    //If we made it this far something happened right.
    return Ok(warp::reply::with_status(warp::reply::json(&"Deleted something, maybe?".to_string()), warp::http::StatusCode::OK));
}