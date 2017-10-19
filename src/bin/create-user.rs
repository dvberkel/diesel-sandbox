extern crate diesel;
extern crate dotenv;
extern crate diesel_sandbox as ds;

use diesel::prelude::*;
use dotenv::dotenv;
use std::io::stdin;

use ds::model::{User, FreshUser};
use ds::schema::users;
use ds::database::establish_connection;

fn store_user(conn: &PgConnection, fresh_user: &FreshUser) -> User {
    diesel::insert(fresh_user).into(users::table)
        .get_result(conn)
        .expect("Error saving new user")
}

fn main() {
    dotenv().ok();
    let connection = establish_connection();

    println!("What would you like your username to be?");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)]; // Drop the newline character

    let fresh_user = FreshUser::new(username);

    let user = store_user(&connection, &fresh_user);
    println!("\nSaved user {:?}", user);
}
