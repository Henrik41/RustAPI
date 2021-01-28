#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod schema;
mod models;

fn main() {
    println!("Hello, world!");
}
