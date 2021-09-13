#[macro_use]
extern crate diesel;

mod db;

fn main() {
    db::create_post("Google", "Facebook");
    db::publish_post(1);

    let posts = db::get_posts();
    println!("{:?}", posts);

    // db::delete_by_pattern("%Goo%");
    // let posts = db::get_posts();
    // println!("{:?}", posts);

}
