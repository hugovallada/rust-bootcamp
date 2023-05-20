use std::{error::Error, fmt::Error};

fn main() {
    let username = get_username(1);
    // match username {
    //     Some(name) => println!("O nome é {name}"),
    //     None => {}
    // }

    if let Some(name) = username {
        println!("{name}")
    }
}

fn get_username(user_id: u32) -> Option<String> {
    let query = format!("GET username FROM users WHERE id = {user_id}");

    let db_result = query_db(query);
    if db_result.is_ok() {
        db_result.ok()
    } else {
        db_result.err()
    }
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty"))
    } else {
        Ok(String::from("Ferris"))
    }
}
