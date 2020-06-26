pub mod hash_table {
    pub struct HashTable;

    impl HashTable {
        pub fn new() -> Self {
            Self
        }

        pub fn insert(&mut self) -> bool {}
        pub fn remove(&mut self) -> () {} //return  Option<removed val> | None
        pub fn lookup(&self) -> () {} //return  Option<looked up val> | None
        pub fn get_key_value_pairs(&self) -> () {} // return vec<(K,V)>
    }
}

#[cfg(test)]
mod tests {
    use super::hash_table;
}
