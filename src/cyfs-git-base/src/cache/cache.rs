use lru::LruCache;
// use cyfs_lib::*;
use cyfs_base::*;

use std::{num::NonZeroUsize, sync::Mutex};

use once_cell::sync::OnceCell;

// let mut cache = LruCache::new(2);

pub static LRU_CACHE: OnceCell<CyfsGitCache> = OnceCell::new();

pub struct CyfsGitCache {
    cache: Mutex<LruCache<String, String>>,
}

impl CyfsGitCache {
    pub fn new() -> BuckyResult<()> {
        let cache: LruCache<String, String> =
            LruCache::new(unsafe { NonZeroUsize::new_unchecked(10000) });
        let _ = LRU_CACHE.set(CyfsGitCache {
            cache: Mutex::new(cache),
        });

        Ok(())
    }

    pub fn put(key: &str, value: &str) -> BuckyResult<()> {
        let instance = LRU_CACHE.get();
        if instance.is_none() {
            return Err(BuckyError::new(
                BuckyErrorCode::ErrorState,
                "empty cache instance",
            ));
        }
        let mut cache = instance.unwrap().cache.lock().unwrap();
        cache.put(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn get(key: &str) -> BuckyResult<Option<String>> {
        let instance = LRU_CACHE.get();
        if instance.is_none() {
            return Err(BuckyError::new(
                BuckyErrorCode::ErrorState,
                "empty cache instance",
            ));
        }
        let mut cache = instance.unwrap().cache.lock().unwrap();
        let result = cache.get(key);
        if result.is_none() {
            return Ok(None);
        } else {
            return Ok(Some(result.unwrap().to_string()));
        }
    }
}
