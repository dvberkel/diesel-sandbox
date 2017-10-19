extern crate dotenv;
extern crate diesel;
extern crate diesel_sandbox as ds;

use std::env;
use std::str::FromStr;
use diesel::prelude::*;
use dotenv::dotenv;
use ds::database::establish_connection;
use ds::model::User;

fn main() {
    use ds::schema::users::dsl::*;
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let identifier = i32::from_str(&args[1]).expect("first argument to be an id");

    let connection = establish_connection();
    let maybe_user = users
        .filter(id.eq(identifier))
        .get_result::<User>(&connection)
        .optional()
        .expect(&format!("Error loading user with id {}", identifier));

    match maybe_user {
        Some(user) => println!("{:?}", user),

        None => println!("No user found"),
    }
}
