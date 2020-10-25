pub mod schema;
pub mod models;
pub mod dto;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::SqliteConnection;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use xml::EventReader;
use xml::reader::XmlEvent;


pub fn establish_connection() -> SqliteConnection {
    let config = Config::get_config();

    let database_url = config
        .expect("Could not locate database_url in config.")
        .database_url;
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub struct Time {
    time : i32 //Store time doubled to avoid floats.
}

impl Time{
    /*
    The Time struct doesn't actually store time. 
    Just used to convert between an ISO format and the web format. 
    And it only really cares about the hour/minute
    */
    pub fn new(time : &str) -> Option<Time>{
        let mut is_ok = true;
        let split_time: Vec<&str> = time.split(" ").collect();
        if split_time.len() == 2{
            //YYYY-MM-DD
            let split_time_a: Vec<&str> = split_time.get(0).unwrap().split("-").collect();
            if !(split_time_a.len() == 3){
                is_ok=false;
                println!("Fail part a len");
            }else{
                //YYYY
                if let Ok(value) = split_time_a.get(0).unwrap().parse::<i32>(){
                    if !(value>=0&&value<=9999&&split_time_a.get(0).unwrap().len()==4){
                        is_ok=false;
                        println!("Fail year");
                    }
                }else{is_ok=false; println!("Fail year");}
                //MM
                if let Ok(value) = split_time_a.get(1).unwrap().parse::<i32>(){
                    if !(value>=1&&value<=12&&split_time_a.get(1).unwrap().len()==2){
                        is_ok=false; println!("fail month");
                    }
                }else{is_ok=false; println!("fail month");}
                //DD
                if let Ok(value) = split_time_a.get(2).unwrap().parse::<i32>(){
                    if !(value>=1&&value<=31&&split_time_a.get(2).unwrap().len()==2){
                        is_ok=false; println!("fail day");
                    }
                }else{is_ok=false; println!("fail day");}
            }
            //HH:MM:SS
            let split_time_b: Vec<&str> = split_time.get(1).unwrap().split(":").collect();
            if !(split_time_b.len() == 3){
                is_ok=false; println!("fail part b len");
            }else{
                //HH
                if let Ok(value) = split_time_b.get(0).unwrap().parse::<i32>(){
                    if !(value>=0&&value<=23&&split_time_b.get(0).unwrap().len()==2){
                        is_ok=false; println!("fail hour");
                    }
                }else{is_ok=false; println!("fail hour");}
                //MM
                if let Ok(value) = split_time_b.get(1).unwrap().parse::<i32>(){
                    if !((value==0||value==30)&&split_time_b.get(1).unwrap().len()==2){
                        is_ok=false; println!("fail minute");
                    }
                }else{is_ok=false; println!("fail minute");}
                //SS
                if let Ok(value) = split_time_b.get(2).unwrap().parse::<i32>(){
                    if !(value==0&&split_time_b.get(2).unwrap().len()==2){
                        is_ok=false; println!("fail second");
                    }
                }else{is_ok=false; println!("fail second");}
            }

            if is_ok {
                let mut calc_time = split_time_b.get(0).unwrap().parse::<i32>().unwrap();
                if split_time_b.get(1).unwrap().parse::<i32>().unwrap()==30 {
                    calc_time = (2*calc_time)+1;
                    return Some(Time {time: calc_time});
                }else{
                    calc_time = 2*calc_time;
                    return Some(Time {time: calc_time});
                }
            }
        }
        println!("Fail inital len");
        return None;
    }

    //Straightforward
    pub fn to_string(&self) -> String{
        if(&self.time % 2)==1{
            
            return format!("1970-01-01 {}:30:00", &self.time/2);
        }else{
            return format!("1970-01-01 {}:00:00", &self.time/2);
        }
    }

    //Floaty goodness
    pub fn to_float(&self) -> f32{
        return (self.time as f32) / 2.0;
    }

    //Impose basic reqs on duration.
    pub fn is_duration_sane(&self, duration: f32) -> bool{
        if (((duration*2.0) as i32) + self.time)>48 || (((duration*2.0) as i32) + self.time)<0 || duration <= 0.0 {
            return false;
        } else{
            return true
        }
    }
}

pub struct Config{
    pub ui_name: String,
    pub ui_logo: String,
    pub ui_colors: Vec<(String, String)>,
    pub server_address : String, 
    pub bind_port : String,
    pub database_url : String
}

impl Config{
    pub fn defaults() -> String{
        return 

r##"
<public-component>
	<ui>
		<name>AlexBox</name>
		<logo>logo.png</logo>
		<tile-colors>
            <color hex-color="#87D962">Green</color>
            <color hex-color="#F2D544">Yellow</color>
            <color hex-color="#D99759">Orange</color>
            <color hex-color="#D96459">Red</color>
            <color hex-color="#D962C5">Pink</color>
		</tile-colors>
	</ui>
</public-component>
<private-component>
	<server-address>0.0.0.0</server-address>
    <bind-port>1985</bind-port>
    <database-url>alexbox.db</database-url>
</private-component>
"##.to_string();

    }


    pub fn get_config() -> Result<Config, String>{
        let mut elements = Vec::new();
        let mut ui_name : std::string::String = "alexbox".to_string();
        let mut ui_logo : std::string::String = "logo.png".to_string();
        let mut ui_colors : Vec<(String, String)> = Vec::new();
        let mut server_address : std::string::String = "0.0.0.0".to_string();
        let mut bind_port : std::string::String = "1984".to_string();
        let mut database_url : std::string::String = "alexbox.db".to_string();

        let read_test = File::open("config.xml");
        if let Err(error) = read_test{
            println!("Could not open config.xml!");
            if error.kind() == std::io::ErrorKind::NotFound{
                println!("Not found... Attempting to create.");
                let write_attempt = File::create("config.xml");
                if let Ok(mut writer) = write_attempt{
                    if let Ok(()) = write!(writer, "{}", Config::defaults()){
                        println!("File creation successful.");
                    }else{
                        println!("File creation failed, running defaults.");
                    }
                }
            }
        }

        let file = File::open("config.xml").unwrap();
        let file = BufReader::new(file);
        let parser = EventReader::new(file);

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    match name.local_name.as_str() {
                        "public-component" => {
                            if elements.last() == None {
                                elements.push(Element::PublicComponent);
                            }
                        }
                        "ui" => {
                            if elements.last() == Some(&Element::PublicComponent) {
                                elements.push(Element::Ui);
                            }
                        }
                        "name" => {
                            if elements.last() == Some(&Element::Ui) {
                                elements.push(Element::Name);
                            }
                        }
                        "logo" => {
                            if elements.last() == Some(&Element::Ui) {
                                elements.push(Element::Logo);
                            }
                        }
                        "tile-colors" => {
                            if elements.last() == Some(&Element::Ui) {
                                elements.push(Element::TileColors);}
                            }
                        "color" => {
                            if elements.last() == Some(&Element::TileColors) {
                                elements.push(Element::Color(attributes));
                            }
                        }
                        "private-component" => {
                            if elements.last() == None {
                                elements.push(Element::PrivateComponent);
                            }
                        }
                        "server-address" => {
                            if elements.last() == Some(&Element::PrivateComponent) {
                                elements.push(Element::ServerAddress);
                            }
                        }
                        "bind-port" => {
                            if elements.last() == Some(&Element::PrivateComponent) {
                                elements.push(Element::BindPort);
                            }
                        }
                        "database-url" => {
                            if elements.last() == Some(&Element::PrivateComponent) {
                                elements.push(Element::DatabaseUrl);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::Characters(string)) =>{
                    let string = string.trim().to_string();
                    match elements.last(){
                        Some(Element::Name) => {ui_name = string;},
                        Some(Element::Logo) => {ui_logo = string;},
                        Some(Element::Color(attributes)) => {
                            if attributes.len()==1{
                                if let Some(hex_color)=attributes.get(0){
                                    if hex_color.name.local_name.as_str() == "hex-color"{
                                        ui_colors.push((string, hex_color.value.clone()));
                                    }
                                }
                            }
                        },
                        Some(Element::ServerAddress) => {server_address = string;},
                        Some(Element::BindPort) => {bind_port = string;},
                        Some(Element::DatabaseUrl) => {database_url = string;},
                        _ => {}
                    }
                },
                Ok(XmlEvent::EndElement { name: _ }) => {
                    elements.pop();
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        return Ok(Config {
            ui_name: ui_name,
            ui_logo: ui_logo,
            ui_colors: ui_colors,
            server_address: server_address,
            bind_port: bind_port,
            database_url: database_url
        })
    }
}

#[derive(PartialEq, Eq)] 
enum Element {
    PublicComponent,
    Ui,
    Name,
    Logo,
    TileColors,
    Color(Vec<xml::attribute::OwnedAttribute>),
    PrivateComponent,
    ServerAddress,
    BindPort,
    DatabaseUrl
}