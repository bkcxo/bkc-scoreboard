#[macro_use] extern crate rocket;

use rocket::response::Redirect;

use rocket_dyn_templates::{Template, context};
use tera::Tera;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Result, Value};
use std::any::Any;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/", hi(name = "Your Name")))
}

#[get("/hi/<name>")]
pub fn hi(name: &str) -> Template {
    Template::render("index", context! {
        title: "Hello",
        name: Some(name),
        items: vec!["One", "Two", "Three"],
    })
}

// #[derive(Deserialize, Debug)]
// struct Data {
//     golf_scores: {
//         brooke: [u8],
//         max: [u8],
//     },
// }
#[derive(Serialize, Deserialize)]
struct Scores {
    brooke: Vec<u8>,
    max: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct Data {
    golf_scores: Scores,
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: f32) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/add_game/<brooke_score>/<max_score>")]
fn add_game(brooke_score: u8, max_score: u8) -> String {
    // Open the file in read-only mode with buffer.
    let file = File::open("foo.json").unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let mut u: Data = serde_json::from_reader(reader).unwrap();

    u.golf_scores.brooke.push(brooke_score);
    u.golf_scores.max.push(max_score);
    let filewrite = File::create("foo.json").unwrap();

    let mut writer = BufWriter::new(filewrite);
    serde_json::to_writer(&mut writer, &u).unwrap();
    writer.flush().unwrap();

    let brooke_total: u8 = u.golf_scores.brooke.iter().sum();
    let max_total: u8 = u.golf_scores.max.iter().sum();
    // print!("{}", u.golf_scores.brooke);
    format!("Brooke has score {} and max has score {}!", brooke_total, max_total)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, add_game, hi, index]).attach(Template::fairing())
}
