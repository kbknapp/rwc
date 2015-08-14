use std::fs::File;
use std::io::Read;

static IN_FILE: &'static str = "test_input.txt";

fn main() {
    // Open and read the file
    let mut file = File::open(IN_FILE).ok().expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file");
   
    // Count the lines (types for clarity)
    let lines: u64 = s.lines()
                      .map(|_| 1)
                      .fold(0, |acc, n| acc + n);
    
    // Display the result
    println!("{} {}", lines, IN_FILE);
}

