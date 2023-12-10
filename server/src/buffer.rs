use std::sync::{Arc, Mutex, atomic::AtomicBool};

use aether_common::hey;

pub struct SuperBuff<T> {
    buffer: Arc<Mutex<Vec<T>>>,
    has_next: AtomicBool,
}

impl<T> SuperBuff<T> {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            has_next: AtomicBool::new(false),
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
        
        // set has to true
        self.has_next.store(true, std::sync::atomic::Ordering::Relaxed);

        // wrote successfully
        true
    }
    
    // gets the next value in the buffer if it exists and removes it
    pub fn pop(&mut self) -> Option<T> {
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
            self.has_next.store(false, std::sync::atomic::Ordering::Relaxed);
            return None;
        }
        
        // remove the value and set has_next if needed
        let removed = buf.remove(0);
        if buf.is_empty() {
            self.has_next.store(false, std::sync::atomic::Ordering::Relaxed);
        }

        Some(removed)
    }

    // check if the buffer contains data
    pub fn has_next(&self) -> bool {
        self.has_next.load(std::sync::atomic::Ordering::Relaxed)
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
            buffer: Arc::clone(&self.buffer),
            has_next: AtomicBool::new(false),
        }
    }
}