use chrono::NaiveDateTime;
use regex::Regex;
use std::path::Path;

pub struct CacheConfig<'a> {
    cache_dir: &'a Path,
    max_size_bytes: Option<usize>,
    cleanup_interval: Option<NaiveDateTime>,
    rname_pattern: Option<Regex>,
    hash_algorithm: &'a str,
    compression: bool,
}

impl<'a> CacheConfig<'a> {
    pub fn new(
        cache_dir: &'a Path,
        max_size_bytes: Option<usize>,
        cleanup_interval: Option<NaiveDateTime>,
        rname_pattern: Option<Regex>,
        hash_algorithm: Option<&'a str>,
        compression: Option<bool>,
    ) -> CacheConfig<'a> {
        CacheConfig {
            cache_dir,
            max_size_bytes,
            cleanup_interval,
            rname_pattern: rname_pattern.or_else(|| Some(Regex::new("^[a-zA-Z0-9_-]+$").unwrap())),
            hash_algorithm: hash_algorithm.unwrap_or("md5"),
            compression: compression.unwrap_or(false),
        }
    }
}
