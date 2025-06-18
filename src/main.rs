pub mod biocache;
pub mod common;
pub mod models;
pub mod schema;

use crate::biocache::*;
use crate::models::{NewResource, Resource};
use common::*;
use dotenvy::dotenv;
use std::env;

fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut bcache = BioCache::new(&database_url);

    // let new_resource = NewResource::new("unique_name1", "path/1.txt", None, None, None, None);
    //
    // bcache.add(new_resource);

    // bcache.show_resources();

    // let result = bcache.get("unique_name1");
    // println!("{:?}", result);

    let results: Vec<Resource> = bcache.list_resources(None);
    print_resources(results);
    // bcache.remove("unique_name1");
}
