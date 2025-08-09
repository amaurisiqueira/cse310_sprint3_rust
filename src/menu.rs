use std::io::{self, Write};

pub fn show_menu() {
    print!("\x1B[2J\x1B[H"); // clear consola 
    println!("Main Sprint 3 - Amaurí Siqueira");
    println!("1 | Create a new task");
    println!("2 | Complete a task");
    println!("3 | Delete a task");
    println!("4 | List all tasks");
    println!("5 | Exit");
    print!("Choose an option (1-5): ");
    let _ = io::stdout().flush();
}

pub fn get_menu_option() -> i32 {
    loop {
        show_menu();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(option) = input.trim().parse::<i32>() {
                if (1..=5).contains(&option) {
                    return option;
                }
            }
        }
        println!("Invalid input. Please enter a number between 1 and 5.");
    }
}
