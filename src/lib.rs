pub mod hash_table {

    const INIT_NUM_OF_BUCKETS: u32 = 1;
    const LOAD_FACTOR: f64 = 0.75;

    struct Bucket {
        bucket: Vec<(String, u32)>, // key == word and value is count
    }
    pub struct HashTable {
        buckets: Vec<Bucket>,
        size: u32,
    }

    impl HashTable {
        pub fn new() -> Self {
            HashTable {
                buckets: Vec::new(),
                size: 0,
            }
        }

        pub fn insert(&mut self, key: String, val: u32) -> () {}
        pub fn remove(&mut self) -> () {} //return  Option<removed val> | None
        pub fn lookup(&self) -> () {} //return  Option<looked up val> | None
        pub fn get_key_value_pairs(&self) -> () {} // return vec<(K,V)>
        pub fn size(&self) -> u32 {
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
