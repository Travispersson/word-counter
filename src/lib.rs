pub mod hash_table {

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    const INIT_NUM_OF_BUCKETS: usize = 17;
    const LOAD_FACTOR: f64 = 0.75;

    #[derive(Clone)]
    struct Bucket {
        bucket: Vec<(String, usize)>, // key == word and value is count
    }
    pub struct HashTable {
        buckets: Vec<Bucket>,
        size: usize,
    }

    impl HashTable {
        pub fn new() -> Self {
            HashTable {
                // just for testing atm -- will resort back to Vec::new() when resize func is
                // implemented
                buckets: vec![Bucket { bucket: Vec::new() }; INIT_NUM_OF_BUCKETS],
                size: 0,
            }
        }

        fn get_hash_for_key(&self, key: &String) -> usize {
            let mut h = DefaultHasher::new();
            println!("length of vec{}", self.buckets.len());
            key.hash(&mut h);
            (h.finish() % self.buckets.len() as u64) as usize
        }

        pub fn insert(&mut self, key: String, val: usize) -> Option<usize> {
            //TODO check if we should resize

            let hashed_key: usize = self.get_hash_for_key(&key);
            let bucket = &mut self.buckets[hashed_key];

            //TODO handle collision - don't alow duplicate keys

            bucket.bucket.push((key, val));
            self.size += 1;
            None
        }
        pub fn resize(&mut self) {}
        pub fn remove(&mut self) -> () {} //return  Option<removed val> | None
        pub fn lookup(&self, key: String) -> Option<usize> {
            let index = self.get_hash_for_key(&key);
            let bucket = &self.buckets[index];

            for (ref key, ref val) in bucket {
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

    // implement an iterator that won't consume the Bucket
    impl<'a> IntoIterator for &'a Bucket {
        type Item = &'a (String, usize);
        type IntoIter = BucketIterator<'a>;

        fn into_iter(self) -> Self::IntoIter {
            BucketIterator {
                bucket: &self.bucket,
                at_index: 0,
            }
        }
    }

    struct BucketIterator<'a> {
        bucket: &'a Vec<(String, usize)>,
        at_index: usize,
    }
    impl<'a> Iterator for BucketIterator<'a> {
        type Item = &'a (String, usize);
        fn next(&mut self) -> Option<Self::Item> {
            let entry = self.bucket.get(self.at_index);
            self.at_index += 1;
            entry // get() returns Option<(String, usize)> so entry is either Some(x,y) or None
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
    #[test]
    fn lookup_ht_test() {
        // also checks that we can iterate through bucket
        let mut ht: HashTable = HashTable::new();
        ht.insert(String::from("Potato"), 10);
        let val = ht.lookup(String::from("Potato"));
        assert_eq!(val, None);
    }
}
