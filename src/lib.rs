pub mod hash_table {

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    const INIT_NUM_OF_BUCKETS: usize = 1;
    const LOAD_FACTOR: f64 = 0.75;

    #[derive(Clone)]
    struct Bucket<K, V> {
        bucket: Vec<(K, V)>,
    }
    pub struct HashTable<K, V> {
        buckets: Vec<Bucket<K, V>>,
        size: usize,
    }

    impl<K: Hash + Eq + Clone, V: Clone> HashTable<K, V> {
        pub fn new() -> Self {
            HashTable {
                buckets: vec![Bucket { bucket: Vec::new() }; INIT_NUM_OF_BUCKETS],
                size: 0,
            }
        }

        fn get_hash_for_key(&self, key: &K) -> usize {
            let mut h = DefaultHasher::new();
            key.hash(&mut h);
            (h.finish() % self.buckets.len() as u64) as usize
        }

        pub fn insert(&mut self, key: K, val: V) -> Option<V> {
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

        pub fn remove(&mut self, key: &K) -> Option<V> {
            let index = self.get_hash_for_key(key);
            let bucket = &mut self.buckets[index];

            let mut i = 0;
            for (ref item_key, _) in bucket.bucket.iter_mut() {
                if item_key == key {
                    self.size -= 1;
                    return Some(bucket.bucket.remove(i).1);
                }
                i += 1;
            }
            None
        }
        pub fn lookup(&self, key: &K) -> Option<&V> {
            let index = self.get_hash_for_key(key);
            let bucket = &self.buckets[index];

            for (ref item_key, ref val) in bucket {
                if item_key == key {
                    return Some(val);
                }
            }
            None
        }

        pub fn get_key_value_pairs(&self) -> Vec<(&K, &V)> {
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
    impl<'a, K, V> IntoIterator for &'a Bucket<K, V> {
        type Item = &'a (K, V);
        type IntoIter = BucketIterator<'a, K, V>;

        fn into_iter(self) -> Self::IntoIter {
            BucketIterator {
                bucket: &self.bucket,
                at_index: 0,
            }
        }
    }

    struct BucketIterator<'a, K, V> {
        bucket: &'a Vec<(K, V)>,
        at_index: usize,
    }
    impl<'a, K, V> Iterator for BucketIterator<'a, K, V> {
        type Item = &'a (K, V);
        fn next(&mut self) -> Option<Self::Item> {
            let entry = self.bucket.get(self.at_index);
            self.at_index += 1;
            entry
        }
    }
}

pub mod str_cutter {

    pub struct StrCutter<'a> {
        remainder: &'a str,
        delimiters: &'a [char],
    }

    impl<'a> StrCutter<'a> {
        pub fn new(text: &'a str, delimiters: &'a [char]) -> Self {
            Self {
                remainder: text,
                delimiters,
            }
        }
    }

    impl<'a> Iterator for StrCutter<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            // this will eat up all consecutive delimiters..
            loop {
                match self.remainder.chars().next() {
                    Some(c) => {
                        if self.delimiters.contains(&c) {
                            self.remainder = &self.remainder[(c.len_utf8() as usize)..]
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }

            //rest of implementation
            if let Some(next) = self.remainder.find(self.delimiters) {
                let up_to_delimiter = &self.remainder[..next];
                // new remainder is the index of found delimeter + size of char (4 bytes)
                self.remainder =
                    &self.remainder[(next + (self.delimiters[0].len_utf8() as usize))..];
                Some(up_to_delimiter)
            } else if self.remainder.is_empty() {
                None
            } else {
                let rest = self.remainder;
                self.remainder = "";
                Some(rest)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::hash_table::HashTable;
    #[test]
    fn insert_ht_test() {
        let mut ht = HashTable::new();
        ht.insert(String::from("Potato"), 10);
        assert_eq!(ht.size(), 1);
        assert_eq!(ht.lookup(&String::from("Potato")), Some(&10));
        ht.insert(String::from("Potato"), 20);
        assert_eq!(ht.lookup(&String::from("Potato")), Some(&20));
    }
    #[test]
    fn lookup_ht_test() {
        // also checks that we can iterate through bucket
        let mut ht = HashTable::new();
        ht.insert(String::from("Potato"), 10);
        let val = ht.lookup(&String::from("Potato"));
        assert_eq!(val, Some(&10));
    }

    #[test]
    fn ht_resize_test() {
        let mut ht = HashTable::new();
        ht.insert(String::from("Potato"), 1);
        ht.insert(String::from("Tomato"), 1);
        ht.insert(String::from("Dylan"), 1);
        ht.insert(String::from("Hamlet"), 1);
        ht.insert(String::from("Pillow"), 1);
        ht.insert(String::from("Century"), 1);
        assert_eq!(ht.lookup(&String::from("Potato")), Some(&1));
        assert_eq!(ht.lookup(&String::from("Tomato")), Some(&1));
        assert_eq!(ht.lookup(&String::from("Dylan")), Some(&1));
        assert_eq!(ht.lookup(&String::from("Hamlet")), Some(&1));
        assert_eq!(ht.lookup(&String::from("Pillow")), Some(&1));
        assert_eq!(ht.lookup(&String::from("Century")), Some(&1));
    }

    #[test]
    fn ht_remove_test() {
        let mut ht = HashTable::new();
        ht.insert(String::from("Potato"), 1);
        ht.insert(String::from("Tomato"), 1);
        assert_eq!(ht.lookup(&String::from("Tomato")), Some(&1));
        ht.remove(&String::from("Tomato"));
        assert_eq!(ht.lookup(&String::from("Tomato")), None);
    }
    #[test]
    fn ht_get_pairs_empty_test() {
        let mut ht = HashTable::new();
        ht.insert(String::from("Tomato"), 1);
        ht.remove(&String::from("Tomato"));
        assert_eq!(ht.get_key_value_pairs(), vec![]);
    }
    #[test]
    fn ht_get_pairs_test() {
        let mut ht = HashTable::new();
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
    use super::str_cutter::StrCutter;

    #[test]
    fn str_cutter_test() {
        let text = "a b c,d";
        let cut = StrCutter::new(text, &[' ', ',']);
        assert!(cut.eq(vec!["a", "b", "c", "d"].into_iter()));
        let text2 = "Hello stranger! This is, some, random gibberish to test our ?! string cutter!";
        let cut2 = StrCutter::new(text2, &['.', ' ', ',', '!', '?']);

        assert!(cut2.eq(vec![
            "Hello",
            "stranger",
            "This",
            "is",
            "some",
            "random",
            "gibberish",
            "to",
            "test",
            "our",
            "string",
            "cutter",
        ]
        .into_iter()));
    }
}
