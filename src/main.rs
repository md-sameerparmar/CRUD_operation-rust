use std::io;

// struct Uname{
//     username: String,
//     password : String,
// }

fn validate_username(username: &str) -> bool {
    let mut is_valid = true;
    
    // Check if the username starts with an underscore
    if username.chars().next() == Some('_') {
        is_valid = false;
        println!("\n- Username must not start with an underscore");
    }
    
    // check username must be between 3 to 12 characters long
    if username.len() < 3 || username.len() > 12{
        is_valid = false;
        println!("\n- Username must be between 3 to 12 characters long");
    }
    // Check if the username has at least one special character
    if !username.chars().any(|c| "!@#$%^&*()_".contains(c)) {
        is_valid = false;
        println!("\n- Username must contain at least one of ' !@#$%^&*()_ ' special character");
    }
    // Check if the username has at least one uppercase character
    if !username.chars().any(|c| c.is_ascii_uppercase()) {
        is_valid = false;
        println!("\n- Username must contain at least one uppercase character");

    }
    // Check if the username has at least one lowercase character
    if !username.chars().any(|c| c.is_ascii_lowercase()) {
        is_valid = false;
        println!("\n- Username must contain at least one lowercase character");
    }
    if !username.chars().any(|c| c.is_digit(10)) {
        is_valid = false;
        println!("\n- Username must contain at least one number");
    }
    is_valid
}

// fn read_user(users: &mut Vec<Uname>, username:String) -> Option<&Uname>{
//     users.iter().find(|user| user.username == username)
// }

fn read_user(username: String) {
    if username == username{
        println!("\n- Your current username is : {}", username);
    }
}

fn update_user(mut username:String){
    // println!("hello from Update {}", username);
    username = username;
    println!("\n- Username changed to {}", username);
}

fn main() {

    // Let mut users:Vec<Uname> = Vec::new();
    
    
        println!("Enter a username:");
        let mut input_username = String::new();
        io::stdin().read_line(&mut input_username).unwrap();
        let username = input_username.trim().to_string();
        
        if validate_username(&username) {
            println!("\n- {}, This username is valid.", username);
            loop{
                println!("\nEnter your Choice: \n- Press 1 for Read. \n- Press 2 for Update. \n- Press 3 for Delete. \n- Press 4 for Exit.");


                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap();
                let choice = choice.trim().to_string();

                match choice.as_str() {
                    "1" => read_user(username.to_string()),
                    "2" => {
                        println!("\nEnter a new username:");
                        let mut new_username = String::new();
                        io::stdin().read_line(&mut new_username).expect("Failed to read line");
                        let new_username = new_username.trim().to_string();
                        
                        if validate_username(&new_username) {
                            // println!("Valid username");
                            update_user(new_username);
                        }else{
                            println!("\nInvalid username");
                        }
                    },
                    "3" => {println!("\nPanding..");},
                    "4" => {break;}
        
                _ => {println!("\nInvalid Choice");}
    }
            }
            
        } else {
            println!("\n- {}, This username is not valid.", username);
        }
          
    // let choice_res = match choice.as_str() {
    //     let user = read_user();
    //     "1" => {
    //         let user = read_user();
    //     }
    //     _ => "Invalid Choice"
    // };
    // println!("\n- Your Choice is: {}", choice_res);

}





// 10 fab 23, friday
// between 1pm to 2pm