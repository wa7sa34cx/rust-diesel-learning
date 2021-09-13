pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::{NewPost, Post};
use schema::posts;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_posts() -> Vec<Post> {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts")
}

pub fn create_post<'a>(title: &'a str, body: &'a str) {
    let connection = establish_connection();

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&connection)
        .unwrap();
}

pub fn publish_post(publish_id: i32) {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    diesel::update(posts.find(publish_id))
        .set(published.eq(true))
        .execute(&connection)
        .unwrap();
}

pub fn delete_by_pattern(pattern: &str) {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");
}