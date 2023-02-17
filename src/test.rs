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
    let mut input_username = String::new();
    println!("Enter a username:");
    io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");
    let username = input_username.trim().to_string();
    create_user(&mut users, 1, username);

    // Read a user
    let user = read_user(&users, 1);
    match user {
        Some(u) => println!("Found user: {}", u.username),
        None => println!("User not found"),
    }

    // Update a user
    let result = update_user(&mut users, 1, String::from("johnsmith"));
    if result {
        println!("User updated successfully");
    } else {
        println!("User not found");
    }

    // Delete a user
    let result = delete_user(&mut users, 1);
    if result {
        println!("User deleted successfully");
    } else {
        println!("User not found");
    }
}
