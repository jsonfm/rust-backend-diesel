use diesel::prelude::*;

pub mod lib;
pub mod models;
pub mod schema;

//
use lib::establish_connection;
use models::Post;
use schema::posts::dsl::*;

fn main() {
    let conn = &mut establish_connection();
    let response = posts.load::<Post>(conn).expect("Failed loading posts!");
    for post in response {
        println!("{}", post.title)
    }
}
