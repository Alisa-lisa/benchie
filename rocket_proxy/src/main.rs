#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;


use diesel::prelude::*;
use rocket_contrib::json::{Json};
use rocket::config::{Config, Environment, Value};
use serde_derive::{Serialize};
mod db;
mod models;
mod schema;
use crate::models::PastDestination;
use std::collections::HashMap;
use crate::schema::past_destinations::dsl::*;


#[derive(Serialize)]
struct Address {
    address: Vec<String>,
}

#[get("/<passenger_id>/<location>")]
fn index(passenger_id: i32, location: String, connection: db::DbConn) -> Json<Address> {
    let mut res = Address{address: Vec::new()};
    let addresses = past_destinations
        .filter(passenger.eq(passenger_id))
        .filter(city.eq(location)).load::<PastDestination>(&*connection).expect("Error loading");
    for addrr in addresses {
        res.address.push(format!("{} {}", addrr.street.unwrap_or_default(), addrr.street_number.unwrap_or_default()));
    }
    Json(res)
}

#[get("/health")]
fn health() -> Json<String> {
    Json("Oloo, me healthy!".to_string())
}

fn main() {
    let database_url = "postgres://postgres:postgres@localhost:5432/pastdestinations";
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    // This is the same as the following TOML:
    // my_db = { url = "database.sqlite" }
    database_config.insert("url", database_url);
    databases.insert("postgres_db", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .port(8089)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    rocket::custom(config).attach(db::DbConn::fairing()).mount("/", routes![index]).launch();
}
