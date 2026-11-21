use std::collections::HashMap;

use dashmap::DashMap;
use tokio::sync::{Semaphore, SemaphorePermit, mpsc::UnboundedReceiver};

pub struct RateLimiter {
    buckets: DashMap<&'static str, Semaphore>,
}

impl RateLimiter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            buckets: DashMap::new(),
        }
    }

    pub fn acquire_permit(&self, bucket: &'static str) -> SemaphorePermit<'_> {
        let entry = self.buckets.entry(bucket);
        entry.or_insert(Semaphore::new())
    }
}
