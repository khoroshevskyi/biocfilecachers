pub mod models;
pub mod schema;

use biocrs::biocache::*;
use dotenvy::dotenv;
use std::env;

fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut bcache = BioCache::new(&database_url);

    // bcache.show_posts();

    let result = bcache.get("74862af4cba96c99e73d2a062b6389af");
    println!("{:?}", result);
    // let mut conn = establish_connection(&database_url);
    //
    // println!("Hello, world 2!");
    // let _ = run_migrations(&mut conn);
    //
    // // coment create_post(&mut conn, "fsdsa", "body_qf23", "this/is/my.path.txt");
    // show_posts(&mut conn);
}
