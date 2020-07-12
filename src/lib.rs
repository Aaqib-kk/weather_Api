
// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "weather"]

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::io::Read;

#[get("/")]
pub fn hello() -> String {
    let path = Path::new("api.json");
    let display = path.display();

    println!("{:?}{:?}", path, display);

    let mut file = match File::create(path) { 
        Ok(file) => file,
        Err(_) => panic!("File could not create the File"),
    };
    match reqwest::get("http://api.openweathermap.org/data/2.5/weather?q=karachi&appid=c20207b125e70d607e3b6e1d2bc5d566") {
        Ok(mut res) => {
            match res.text() {
                Ok(text) => match file.write_all(text.as_bytes()) {
                    Ok(_) => println!("Data write in File!"),
                    Err(e) => panic!("The Error {}", e), 
                }
                Err(_) => println!("The response is not received from server")
            }
        }
        Err(_) => println!("The Server could not extablished the connection"),
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("the file open error: {}",why.description()),
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let json = Json::from_str(&buffer).unwrap();

    let result = format!("The temperature of karachi is: {}", json.find_path(&["main"]).unwrap());
    result
}


