use anyhow::Result;
use diesel::prelude::*;

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

use dotenvy::dotenv;
use std::env;

use crate::models::{NewResource, Resource};
use crate::schema::resource;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations(connection: &mut SqliteConnection) -> Result<()> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    let _ = connection.run_pending_migrations(MIGRATIONS);

    Ok(())
}

pub fn create_post(conn: &mut SqliteConnection, id_name: &str, text: &str, path: &str) -> Resource {
    let new_post = NewResource {
        rid: id_name,
        rname: text,
        fpath: None,
        etag: None,
        expires: None,
        rpath: path,
        rtype: None,
    };

    diesel::insert_into(resource::table)
        .values(&new_post)
        .returning(Resource::as_returning())
        .get_result(conn)
        .expect("Error saving new resource")
}

pub fn show_posts(conn: &mut SqliteConnection) -> () {
    use crate::schema::resource::dsl::*;

    let results = resource
        // .filter(published.eq(true))
        .limit(25)
        .select(Resource::as_select())
        .load(conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for resources in results {
        println!("{}", resources.id);
        println!("-----------\n");
        println!("{}", resources.rid);
        println!("{}", resources.rname);
        println!("{}", resources.fpath.unwrap_or_else(|| "".to_string()));
        println!("{}", resources.rpath);
    }
}
