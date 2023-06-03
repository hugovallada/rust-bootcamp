#![allow(dead_code, unused_variables)]

pub use auth_utils::models::Credentials;
use database::Status;
use rand::prelude::*;

mod database;

mod auth_utils;

pub fn authenticate(creds: Credentials) {
    let timeout = thread_rng().gen_range(100..500);
    println!("{timeout}");
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
