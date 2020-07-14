mod lib;
use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = "/usr/share/dict/words";
    // let file_name = ""
    let mut file = File::open(file_name).expect("File not found!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read the file!");
    // println!("{}", contents);

    let words = lib::str_cutter::StrCutter::new(&contents[..], &['.', ' ', ',', '!', '?', '\n']);

    let mut ht = lib::hash_table::HashTable::new();
    for w in words {
        let w_string = String::from(w);
        if let Some(val) = ht.lookup(&w_string) {
            ht.insert(w_string, val + 1);
        } else {
            ht.insert(w_string, 1);
        }
    }

    let pairs = ht.get_key_value_pairs();
    for (word, value) in pairs {
        println!("{}: {}", word, value);
    }
}
