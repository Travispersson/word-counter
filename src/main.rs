mod lib;

fn main() {
    use lib::str_cutter::StrCutter;
    let text2 = "Hello stranger! This is, some, random gibberish to test our ?! string cutter!";
    let cut2 = StrCutter::new(text2, &['.', ' ', ',', '!', '?']);
    for thing in cut2 {
        println!("{}", thing);
    }

    println!("Hello, world!");
}
