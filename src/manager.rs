use crate::task::Task;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub struct TaskManager {
    tasks: Vec<Task>,
    filename: String,
}

impl TaskManager {
    pub fn new(filename: &str) -> Self {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            filename: filename.to_string(),
        };
        manager.load();
        manager
    }

    fn load(&mut self) {
        println!("DEBUG: Nombre del archivo: '{}'", &self.filename);
        if let Ok(file) = File::open(&self.filename) {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 3 {
                    if let (Ok(id), Ok(completed)) = (
                        parts[0].trim().parse::<i32>(),
                        parts[1].trim().parse::<i32>(),
                    ) {
                        let description = parts[2].trim().to_string();
                        self.tasks.push(Task {
                            id,
                            completed: completed == 1,
                            description,
                        });
                    }
                }
            }
        }
    }

    pub fn save(&self) {
        if let Ok(mut file) = File::create(&self.filename) {
            for task in &self.tasks {
                let line = format!(
                    "{}|{}|{}\n",
                    task.id,
                    if task.completed { 1 } else { 0 },
                    task.description
                );
                let _ = file.write_all(line.as_bytes());
            }
        }
    }

    pub fn add(&mut self, description: String) {
        let new_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.tasks.push(Task::new(new_id, description));
    }

    pub fn complete(&mut self, id: i32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
        }
    }

    pub fn delete(&mut self, id: i32) {
        self.tasks.retain(|t| t.id != id);
    }

    pub fn list(&self) {
        println!(" ID | Completed | Description");
        println!("-------------------------------");
        for task in &self.tasks {
            println!(
                "{:>3} | {:<9} | {}",
                task.id,
                if task.completed { "Yes" } else { "No" },
                task.description
            );
        }
    }
}
