#[macro_use]
extern crate diesel;

mod db;

fn main() {
    db::create_post("Google", "Facebook");

    let posts = db::get_posts();
    println!("{:?}", posts);
}
