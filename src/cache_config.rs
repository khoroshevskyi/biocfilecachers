use std::path::{Path, PathBuf};
use regex::Regex;
use chrono::NaiveDateTime;


pub struct CacheConfig<'a> {
    cache_dir: &'a Path,
    max_size_bytes: Option<usize>,
    cleanup_interval: Option<NaiveDateTime>,
    rname_pattern: Option<Regex>,
    hash_algorithm: Option<HashAlgorithm>,
    compression: bool,
}

impl CacheConfig {
    pub fn new(cache_dir: &Path, max_size_bytes: Option<usize>, cleanup_interval: Option<NaiveDateTime>, rname_pattern: Option<Regex>, hash_algorithm: Option<HashAlgorithm>, compression: Option<Compression>) -> CacheConfig
    {
        return CacheConfig {
            cache_dir,
            max_size_bytes,
            cleanup_interval,
            rname_pattern: rname_pattern.unwrap_or(r"^[a-zA-Z0-9_-]+$"),
            hash_algorithm: hash_algorithm.unwrap_or("md5"),
            compression: compression.unwrap_or(false),
        }
    }
}