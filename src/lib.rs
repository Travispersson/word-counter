pub mod hash_table {

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    const INIT_NUM_OF_BUCKETS: u32 = 17;
    const LOAD_FACTOR: f64 = 0.75;

    struct Bucket {
        bucket: Vec<(String, u32)>, // key == word and value is count
    }
    pub struct HashTable {
        buckets: Vec<Bucket>,
        size: usize,
    }

    impl HashTable {
        pub fn new() -> Self {
            HashTable {
                buckets: Vec::new(),
                size: 0,
            }
        }

        fn get_hash_for_key(&self, key: &String) -> usize {
            let mut h = DefaultHasher::new();
            key.hash(&mut h);
            (h.finish() % self.buckets.len() as u64) as usize
        }

        pub fn insert(&mut self, key: String, val: usize) -> () {}
        pub fn remove(&mut self) -> () {} //return  Option<removed val> | None
        pub fn lookup(&self, key: String) -> Option<u32> {
            let index = self.get_hash_for_key(&key);
            let bucket = &self.buckets[index];

            // need to implement iterator trait for Bucket
            for (ref key, ref val) in bucket.iter() {
                println!("Key: {} and Val: {}", key, val);
            }
            None
        }

        pub fn get_key_value_pairs(&self) -> () {} // return vec<(K,V)>
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn is_empty(&self) -> bool {
            self.size == 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::hash_table::HashTable;
    #[test]
    fn insert_ht_test() {
        let mut ht: HashTable = HashTable::new();
        ht.insert(String::from("Potato"), 10);
        assert_eq!(ht.size(), 1);
    }
}
