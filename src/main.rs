use std::fs::File;
use std::io::Read;

static IN_FILE: &'static str = "test_input.txt";

fn main() {
    // Open and read the file
    let mut file = File::open(IN_FILE).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file");
   
    // Count the lines
    let lines = s.lines()
                 .count(); // usize

    // Display the result
    println!("{} {}", lines, IN_FILE);
}

