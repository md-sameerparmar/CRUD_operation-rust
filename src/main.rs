use regex::Regex;
use std::io;

// fn validation_check(username: &str) -> bool {
//     let mut is_valid = true;
    
//     if Regex::new("^[^_].*").unwrap().is_match(username) {
//         // print!("{}",is_valid);
//         is_valid = false;
//     };
//     if Regex::new("^[a-zA-Z0-9_]{3,12}$").unwrap().is_match(username) {
//         print!("{}",is_valid);
//         is_valid = true;
//     };

//     is_valid
// }

fn main() {
    println!("Enter a username : ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let re = Regex::new(r"^[^_].*{3,12}").unwrap();
    // ([A-Z])\w+^(?!_)(?=.*[!@#$%^&*()])(?=.*[A-Z])(?=.*[0-9])(?=.*[a-z])[A-Za-z0-9!@#$%^&*()]{3,12}$
    let username = username.trim();
    

    if re.is_match(username) {
        println!("Input is valid!");
    } else {
        println!("Input is not valid!");
    }
}