extern crate dotenv;
extern crate diesel;
extern crate diesel_sandbox as ds;

use diesel::prelude::*;
use dotenv::dotenv;
use ds::database::establish_connection;
use ds::model::User;

fn main() {
    dotenv().ok();
    use ds::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .order(id.desc())
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user);
    }
}
