pub mod biocache;
pub mod common;
pub mod models;
pub mod schema;

use crate::biocache::*;
use crate::models::{Resource};
use common::*;
use dotenvy::dotenv;
use std::env;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_path: &Path = Path::new(database_url.as_str());

    let mut bcache = BioCache::new(database_path);

    // let new_resource = NewResource::new("unique_name1", "path/1.txt", None, None, None, None);
    //
    // bcache.add(new_resource);

    // bcache.show_resources();

    // let result = bcache.get("unique_name1");
    // println!("{:?}", result);

    let results: Vec<Resource> = bcache.list_resources(Some(50 as i64));
    print_resources(results);
    // bcache.remove("unique_name1");
}
