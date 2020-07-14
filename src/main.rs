mod lib;
use std::env;
use std::fs::File;
use std::io::Read;

fn process_word(ht: &mut lib::hash_table::HashTable, word: &str) -> () {
    let w_string = String::from(word);
    if let Some(val) = ht.lookup(&w_string) {
        ht.insert(w_string, val + 1);
    } else {
        ht.insert(w_string, 1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Too many arguments. Expected 1");
    } else if args.len() < 2 {
        panic!("Expected an argument (file name)");
    } else {
        let file_path = &args[1];
        let mut file = File::open(file_path).expect("File not found!");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't read the file!");

        let words =
            lib::str_cutter::StrCutter::new(&contents[..], &['.', ' ', ',', '!', '?', '\n']);

        let mut ht = lib::hash_table::HashTable::new();
        for w in words {
            process_word(&mut ht, w)
        }

        let pairs = ht.get_key_value_pairs();
        for (word, value) in pairs {
            println!("{}: {}", word, value);
        }
    }
}
