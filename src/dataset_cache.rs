use std::collections::HashMap;
use std::vec::Vec;
//use std::ops::{Index, IndexMut};

pub struct DatasetCache {
    time: u64,
    existing:   HashMap<String, (usize, u64)>,
    data    :   Vec<u8>,
    size    :   usize
    
}

#[derive(Debug)]
pub enum DatasetCacheResult<'a> {
    Valid(&'a [u8]),
    Invalid(&'a mut [u8])
}

impl DatasetCache {
    pub fn new(size: usize, count: usize) -> Self {
        let mut existing = HashMap::new();
        for i in 0..count {
            existing.insert(format!("{:?}",i), (i * size, 0));
        }        

        DatasetCache {
            time: 0,
            existing,
            data    : vec![0; size * count],
            size
        }
    }

    pub fn evict(&mut self, key: &str) -> usize {
        let (lru_key, &(mut lru_val)) = self.existing.iter().min_by_key(|kvp| kvp.1.1).unwrap();
        let reee = lru_key.to_owned();

        self.existing.remove(&reee);

        self.time = self.time + 1;
        lru_val.1 = self.time;
        self.existing.insert(key.to_string(), lru_val);
        
        lru_val.0
    }
    
    pub fn access(&mut self, key: &str) -> Option<&[u8]> {
        match self.existing.get_mut(key) {
            Some((i, t)) => {
                self.time += 1;
                *t = self.time;
                Some(&self.data[*i..*i+self.size])
            }
            None => None
        }
    }

    pub fn access_mut(&mut self, key: &str) -> DatasetCacheResult {
        match self.existing.get_mut(key) {
            Some((i, t)) => {
                self.time += 1;
                *t = self.time;
                DatasetCacheResult::Valid(&self.data[*i..*i+self.size])
            },
            None => {
                let i = self.evict(key);
                DatasetCacheResult::Invalid(&mut self.data[i..i+self.size])
            }
        }
    }
}