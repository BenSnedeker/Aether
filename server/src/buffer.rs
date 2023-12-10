use std::sync::{Arc, Mutex};

use aether_common::hey;

pub struct SuperBuff<T> {
    buffer: Arc<Mutex<Vec<T>>>,
}

impl<T> SuperBuff<T> {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new()))
        }
    }
    
    // returns true if writes successfully
    pub fn write(&mut self, value: T) -> bool {
        // wait until a lock is obtainable for the buffer
        let lock = self.buffer.lock();
        // handle errors
        if let Err(e) = lock {
            hey!("Failed to write data to buffer: Could not get lock! Error: {}", e);
            return false;
        }
        // write to the lock
        lock.unwrap().push(value);
        
        // wrote successfully
        true
    }
    
    // gets the next value in the buffer if it exists
    pub fn next(&mut self) -> Option<T> {
        // wait until a lock is obtainable for the buffer
        let lock = self.buffer.lock();
        // handle errors
        if let Err(e) = lock {
            hey!("Failed to write data to buffer: Could not get lock! Error: {}", e);
            return None;
        }
        
        // get the buffer and return the first value
        let mut buf = lock.unwrap();
        if buf.is_empty() {
            return None;
        }
        
        Some(buf.remove(0))
    }
    
    // gets the length of the buffer
    pub fn len(&self) -> usize {
        // wait until a lock is obtainable for the buffer
        let lock = self.buffer.lock();
        // handle errors
        if let Err(e) = lock {
            hey!("Failed to write data to buffer: Could not get lock! Error: {}", e);
            return 0;
        }
        
        // return the length of the buffer
        lock.unwrap().len()
    }
    
}

impl<T> Clone for SuperBuff<T> {
    fn clone(&self) -> Self {
        Self {
            buffer: Arc::clone(&self.buffer)
        }
    }
}