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
    let mut _i = 1;
    let mut output = 0;
    for line in contents.lines() {
        let mut winner = true ;
        let words = line.split(";"); 
        for word in words{
            let sum = extract_nb_by_set(word);
            if sum > 14 {
                winner = false;
            }
        }
        if winner {
            output += _i;
        }
        _i +=1;
    }
    println!("{}",output);
    

}

fn extract_nb_by_set(s : &str) -> u8 {
    let mut sum : u8 = 0;
    let parts = s.split(" ");
    for part in parts {
        if part.len() <= 2 &&  part.len() > 0 && !(part.contains(':')){
            let byte : u8  = part.trim().parse().expect("Type a number");
            sum = sum + byte
        }
    }
    sum 
        

}
