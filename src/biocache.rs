use anyhow::Result;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
// use std::ops::{Deref};
use std::path::{Path, PathBuf};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

use crate::models::{NewResource, Resource};
use crate::schema::resource;

/// Establish connection with database.
///
/// # Arguments:
/// - database_url: database url path
///
///
pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Run diesel migrations on current connection to sqlite database
///
/// # Arguments:
/// - connection: diesel connection to the database
///
pub fn run_migrations(connection: &mut SqliteConnection) -> Result<()> {
    let _ = connection.run_pending_migrations(MIGRATIONS);
    Ok(())
}

pub fn create_post(conn: &mut SqliteConnection, text: String, path: &str) -> Resource {
    let new_post = NewResource::new(&text, &path);

    diesel::insert_into(resource::table)
        .values(&new_post)
        .returning(Resource::as_returning())
        .get_result(conn)
        .expect("Error saving new resource")
}

pub struct BioCache {
    pub connection: SqliteConnection,
    // config: CacheConfig
}

impl BioCache {
    /// Establish connection with database.
    ///
    /// # Arguments:
    /// - cache_dir: path to the directory where cache should be stored
    ///
    /// # Returns
    /// Biocache object
    pub fn new(cache_dir: &Path) -> Self {
        let mut url_path: PathBuf = cache_dir.to_path_buf();
        url_path = url_path.join("BiocFileCache.sqlite");
        let mut connection: SqliteConnection = establish_connection(url_path.to_str().unwrap());
        let _ = run_migrations(&mut connection);

        BioCache { connection }
    }

    pub fn get(&mut self, name: &str) -> Option<Resource> {
        use crate::schema::resource::dsl::*;

        let result = resource
            .select(Resource::as_select())
            .limit(1)
            .filter(rname.eq(name))
            .load(&mut self.connection)
            .expect("Error in result list");

        let result1 = result.first();
        match result1 {
            Some(res) => Some(res.clone()),
            None => None,
        }
    }

    /// Add a resource to the cache.
    ///
    /// # Arguments:
    /// - new_resource: New Resource object from models module
    ///
    pub fn add(&mut self, new_resource: &NewResource) -> () {
        diesel::insert_into(resource::table)
            .values(new_resource)
            .returning(Resource::as_returning())
            .get_result(&mut self.connection)
            .expect("Error saving new resource");
    }

    pub fn remove(&mut self, name: &str) -> () {
        use crate::schema::resource::dsl::*;

        diesel::delete(resource.filter(rname.eq(name)))
            .execute(&mut self.connection)
            .expect("Error deleting posts");
    }

    pub fn list_resources(&mut self, limit: Option<i64>) -> Vec<Resource> {
        use crate::schema::resource::dsl::*;

        let limit: i64 = match limit {
            Some(limit) => limit,
            None => 25,
        };

        let results: Vec<Resource> = resource
            .limit(limit)
            .select(Resource::as_select())
            .load(&mut self.connection)
            .expect("Error loading posts");
        results
    }

    pub fn update(&mut self) -> () {
        println!("Not implemented yet");
    }

    pub fn add_batch() -> () {
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
        println!("{}", query);
        println!("Not implemented yet");
    }

    pub fn get_stats(&mut self) -> () {
        println!("Not implemented yet");
    }

    pub fn purge(&mut self, force: bool) -> () {
        println!("{}", force);
        println!("Not implemented yet");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cache_file() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut new_file_path = tempdir.keep();

        let _ = BioCache::new(&new_file_path);

        new_file_path.push("BiocFileCache.sqlite");
        assert!(new_file_path.exists());
    }

    #[test]
    fn test_add_retrieve() {
        let tempdir = tempfile::tempdir().unwrap();
        let new_file_path = tempdir.keep();

        let mut bcache: BioCache = BioCache::new(&new_file_path);

        let new_recourse = NewResource::new("recourse_name1", "comp/path/to/recourse_name1")
            .set_fpath("comp/path/to/recourse_name1");

        bcache.add(&new_recourse);

        let cached_resource = bcache.get("recourse_name1");

        assert_eq!(&new_recourse.rpath, &cached_resource.unwrap().rpath);
    }
}
