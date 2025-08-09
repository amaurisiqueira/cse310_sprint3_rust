mod manager;
mod menu;
mod task;

use crate::manager::TaskManager;
use crate::menu::get_menu_option;
use std::io::{self, Write};

const FILENAME: &str = "my_task.txt";

fn main() {
    let mut active = true;

    while active {
        let option = get_menu_option();
        let mut manager = TaskManager::new(FILENAME);

        match option {
            1 => {
                println!("Description: ");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                manager.add(desc.trim().to_string());
                manager.save();
            }
            2 => {
                print!("Enter ID to complete: ");
                let _ = io::stdout().flush();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                if let Ok(id) = id_str.trim().parse() {
                    manager.complete(id);
                    manager.save();
                }
            }
            3 => {
                print!("Enter ID to delete: ");
                let _ = io::stdout().flush();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                if let Ok(id) = id_str.trim().parse() {
                    manager.delete(id);
                    manager.save();
                }
            }
            4 => {
                manager.list();
                println!("Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            5 => {
                active = false;
            }
            _ => {}
        }
    }
}
