use std::io;

struct User {
    id: i32,
    username: String,
}

fn create_user(users: &mut Vec<User>, id: i32, username: String) {
    let new_user = User { id, username };
    users.push(new_user);
}

fn read_user(users: &Vec<User>, id: i32) -> Option<&User> {
    users.iter().find(|user| user.id == id)
}

fn update_user(users: &mut Vec<User>, id: i32, new_username: String) -> bool {
    if let Some(user) = users.iter_mut().find(|user| user.id == id) {
        user.username = new_username;
        true
    } else {
        false
    }
}

fn delete_user(users: &mut Vec<User>, id: i32) -> bool {
    let index = users.iter().position(|user| user.id == id);
    if let Some(i) = index {
        users.remove(i);
        true
    } else {
        false
    }
}

fn main() {
    let mut users: Vec<User> = Vec::new();

    // Create a user
    create_user(&mut users, 1, String::from("johndoe"));

    loop {
        // Prompt user for CRUD operation
        println!("Enter 1 to read user, 2 to update user, 3 to delete user, or 4 to quit:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                // Read a user
                let user = read_user(&users, 1);
                match user {
                    Some(u) => println!("Found user: {}", u.username),
                    None => println!("User not found"),
                }
            }
            "2" => {
                // Update a user
                println!("Enter a new username:");
                let mut new_username = String::new();
                io::stdin()
                    .read_line(&mut new_username)
                    .expect("Failed to read line");
                let new_username = new_username.trim().to_string();
                let result = update_user(&mut users, 1, new_username);
                if result {
                    println!("User updated successfully");
                } else {
                    println!("User not found");
                }
            }
            "3" => {
                // Delete a user
                let result = delete_user(&mut users, 1);
                if result {
                    println!("User deleted successfully");
                } else {
                    println!("User not found");
                }
            }
            "4" => {
                // Quit
                break;
            }
            _ => {
                println!("Invalid choice, please try again");
            }
        }
    }
}
