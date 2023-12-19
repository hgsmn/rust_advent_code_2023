use std::env;
use std::fs;

fn main() {
    // init args handler
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // Read text file
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file"); 
    // Init output scrit sum
    let mut sum : u8 = 0;
    for line in contents.lines() {
        sum = sum + extract_first_last_number(line);
    }
    println!("{}",sum)
}

fn extract_first_last_number(s : &str) -> u8 {
    // Convert string to bytes to compare the ASCII code for filtering 
    let bytes = s.as_bytes();
    // Init boolean & elements 
    let mut first : bool = true;
    let mut last : bool = true;
    let mut first_element : char = '0';
    let mut last_element : char = '0';
    // Iterate over the string using enumerate fuction 
    // This function allows to output index and value as in Pythons
    for (i, &item) in bytes.iter().enumerate() {
        if item < 58 && item > 47{
            if first {
                // This allows to extract directly the char from the bytes ASCII code
                first_element = bytes[i] as char;
                first = false ;
            } else {
                // This allows to extract directly the char from the bytes ASCII code
                last_element = bytes[i] as char;
                last = false;
            }
        }
    }
    // If only one figure. 
    if last {
        last_element = first_element;
    }
    let mut result = String::from(first_element);
    result.push(last_element);
    let output : u8 = result.trim().parse().expect("This is not a number");
    output
}