use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::ops::Deref;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

use crate::models::{NewResource, Resource};
use crate::schema::resource;
// use crate::cache_config::{CacheConfig};  // not used now

/// Establish connection with database.
///
/// # Arguments:
/// - database_url: database url path
pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Run diesel migrations on current connection to sqlite database
///
/// # Arguments:
/// - connection: diesel connection to the database
pub fn run_migrations(connection: &mut SqliteConnection) -> Result<()> {
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

pub fn show_posts(conn: &mut SqliteConnection) -> () {}

pub struct BioCache {
    pub connection: SqliteConnection,
    // config: CacheConfig
}

impl BioCache {
    pub fn new(cache_dir: &str) -> Self {
        let mut connection: SqliteConnection = establish_connection(cache_dir);
        let _ = run_migrations(&mut connection);

        BioCache { connection }
    }

    pub fn show_posts(&mut self) -> () {
        use crate::schema::resource::dsl::*;

        let results = resource
            // .filter(published.eq(true))
            .limit(25)
            .select(Resource::as_select())
            .load(&mut self.connection)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for resources in results {
            println!("id {}", resources.id);
            println!("-----------\n");
            println!("rid {}", resources.rid);
            println!("rname {}", resources.rname);
            println!(
                "fpath {}",
                resources.fpath.unwrap_or_else(|| "".to_string())
            );
            println!("rpath{}", resources.rpath);
        }
    }

    pub fn get(&mut self, rname1: &str) -> Option<Resource> {
        use crate::schema::resource::dsl::*;

        let result = resource
            .select(Resource::as_select())
            .limit(1)
            .filter(rname.eq(rname1))
            .load(&mut self.connection)
            .expect("Error in result list");

        let result1 = result.first();
        match result1 {
            Some(res) => Some(res.clone()),
            None => None,
        }
    }

    // Add a resource to the cache.
    //
    //         Args:
    //             rname:
    //                 Name to identify the resource in cache.
    //
    //             fpath:
    //                 Path to the source file.
    //
    //             rtype:
    //                 Type of resource.
    //                 One of ``local``, ``web``, or ``relative``.
    //                 Defaults to ``local``.
    //
    //             action:
    //                 How to handle the file ("copy", "move", or "asis").
    //                 Defaults to ``copy``.
    //
    //             tags:
    //                 Optional list of tags for categorization.
    //
    //             expires:
    //                 Optional expiration datetime.
    //                 If None, resource never expires.
    //
    //             ext:
    //                 Whether to use filepath extension when storing in cache.
    //                 Defaults to `False`.
    //
    //         Returns:
    //             The `Resource` object added to the cache.
    pub fn add(
        &mut self,
        rname: &str,
        fpath: &str,
        rtype: &str,
        action: &str,
        tags: Option(Vec(&str)),
        expires: Option<NaiveDateTime>,
        ext: bool,
    ) -> () {
    }

    pub fn add_batch() -> () {
        println!("Not implemented yet");
    }

    pub fn remove(&mut self) -> () {
        println!("Not implemented yet");
    }

    pub fn update(&mut self, rname: &str) -> () {
        println!("Not implemented yet");
    }

    pub fn list_resources(&mut self) -> () {
        println!("Not implemented yet");
    }

    pub fn get_cache_size(&mut self) -> () {
        println!("Not implemented yet");
    }

    pub fn export_metadata(&mut self) -> () {
        println!("Not implemented yet");
    }

    pub fn import_metadata(&mut self) -> () {
        println!("Not implemented yet");
    }

    pub fn verify_cache(&mut self) -> () {
        println!("Not implemented yet");
    }

    pub fn search(&mut self, query: &str) -> () {
        println!("Not implemented yet");
    }

    pub fn get_stats(&mut self) -> () {
        println!("Not implemented yet");
    }

    pub fn purge(&mut self, force: bool) -> () {
        println!("Not implemented yet");
    }
}
