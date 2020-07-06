pub mod hash_table {

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    const INIT_NUM_OF_BUCKETS: usize = 1;
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
                buckets: vec![Bucket { bucket: Vec::new() }; INIT_NUM_OF_BUCKETS],
                size: 0,
            }
        }

        fn get_hash_for_key(&self, key: &String) -> usize {
            let mut h = DefaultHasher::new();
            key.hash(&mut h);
            (h.finish() % self.buckets.len() as u64) as usize
        }

        pub fn insert(&mut self, key: String, val: usize) -> Option<usize> {
            //TODO check if we should resize
            if (self.buckets.len() as f64) * LOAD_FACTOR <= (self.size as f64) {
                self.resize();
            }

            let hashed_key: usize = self.get_hash_for_key(&key);
            let bucket = &mut self.buckets[hashed_key];

            //TODO how to impl my own iterator for mutable references??
            for (ref item_key, ref mut item_val) in bucket.bucket.iter_mut() {
                if *item_key == key {
                    let old_val = std::mem::replace(item_val, val);
                    return Some(old_val);
                }
            }
            bucket.bucket.push((key, val));
            self.size += 1;
            None
        }
        pub fn resize(&mut self) {
            let new_size: usize = self.buckets.len() * 2;

            let mut new_buckets = vec![Bucket { bucket: Vec::new() }; new_size];

            for (key, val) in self
                .buckets
                .iter_mut()
                .flat_map(|bucket| bucket.bucket.drain(..))
            {
                let mut h = DefaultHasher::new();
                key.hash(&mut h);
                let index = (h.finish() % new_size as u64) as usize;
                new_buckets[index].bucket.push((key, val));
            }
            std::mem::replace(&mut self.buckets, new_buckets);
        }

        pub fn remove(&mut self, key: String) -> Option<usize> {
            let index = self.get_hash_for_key(&key);
            let bucket = &mut self.buckets[index];

            let mut i = 0;
            for (ref item_key, _) in bucket.bucket.iter_mut() {
                if *item_key == key {
                    self.size -= 1;
                    return Some(bucket.bucket.remove(i).1);
                }
                i += 1;
            }
            None
        }
        pub fn lookup(&self, key: String) -> Option<usize> {
            let index = self.get_hash_for_key(&key);
            let bucket = &self.buckets[index];

            for (ref item_key, ref val) in bucket {
                if *item_key == key {
                    return Some(*val);
                }
            }
            None
        }

        pub fn get_key_value_pairs(&self) -> Vec<(&String, &usize)> {
            let mut pairs = vec![];
            for bucket in &self.buckets {
                for (ref key, ref val) in bucket {
                    pairs.push((key, val));
                }
            }
            pairs
        }

        pub fn size(&self) -> usize {
            self.size
        }

        pub fn is_empty(&self) -> bool {
            self.size == 0
        }
    }

    // implement an iterator that won't consume the Bucket (over immutable references only)
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
        assert_eq!(ht.lookup(String::from("Potato")), Some(10));
        ht.insert(String::from("Potato"), 20);
        assert_eq!(ht.lookup(String::from("Potato")), Some(20));
    }
    #[test]
    fn lookup_ht_test() {
        // also checks that we can iterate through bucket
        let mut ht: HashTable = HashTable::new();
        ht.insert(String::from("Potato"), 10);
        let val = ht.lookup(String::from("Potato"));
        assert_eq!(val, Some(10));
    }

    #[test]
    fn ht_resize_test() {
        let mut ht: HashTable = HashTable::new();
        ht.insert(String::from("Potato"), 1);
        ht.insert(String::from("Tomato"), 1);
        ht.insert(String::from("Dylan"), 1);
        ht.insert(String::from("Hamlet"), 1);
        ht.insert(String::from("Pillow"), 1);
        ht.insert(String::from("Century"), 1);
        assert_eq!(ht.lookup(String::from("Potato")), Some(1));
        assert_eq!(ht.lookup(String::from("Tomato")), Some(1));
        assert_eq!(ht.lookup(String::from("Dylan")), Some(1));
        assert_eq!(ht.lookup(String::from("Hamlet")), Some(1));
        assert_eq!(ht.lookup(String::from("Pillow")), Some(1));
        assert_eq!(ht.lookup(String::from("Century")), Some(1));
    }

    #[test]
    fn ht_remove_test() {
        let mut ht: HashTable = HashTable::new();
        ht.insert(String::from("Potato"), 1);
        ht.insert(String::from("Tomato"), 1);
        assert_eq!(ht.lookup(String::from("Tomato")), Some(1));
        ht.remove(String::from("Tomato"));
        assert_eq!(ht.lookup(String::from("Tomato")), None);
    }
    #[test]
    fn ht_get_pairs_empty_test() {
        let ht: HashTable = HashTable::new();
        assert_eq!(ht.get_key_value_pairs(), vec![]);
    }
    #[test]
    fn ht_get_pairs_test() {
        let mut ht: HashTable = HashTable::new();
        ht.insert(String::from("Potato"), 1);
        ht.insert(String::from("Tomato"), 1);
        ht.insert(String::from("Dylan"), 1);
        assert_eq!(
            ht.get_key_value_pairs().sort(),
            [
                (String::from("Potato"), 1),
                (String::from("Tomato"), 1),
                (String::from("Dylan"), 1)
            ]
            .sort()
        )
    }
}
