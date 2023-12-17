use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file"); 
    let mut sum : u8 = 0;
    for line in contents.lines() {
        sum = sum + extract_first_last_number(line);
    }
    println!("{}",sum)
}

fn extract_first_last_number(s : &str) -> u8 {
    let bytes = s.as_bytes();
    let mut first : bool = true;
    let mut last : bool = true;
    let mut first_element : char = '0';
    let mut last_element : char = '0';
    for (i, &item) in bytes.iter().enumerate() {
        if item < 58 {
            if first {
                first_element = bytes[i] as char;
                first = false ;
            } else {
                last_element = bytes[i] as char;
                last = false;
            }
        }
    }
    if last {
        last_element = first_element;
    }
    let mut result = String::from(first_element);
    result.push(last_element);
    let output : u8 = result.trim().parse().expect("This is note a number");
    output
}