# Overview

## Purpose of the Application

The main purpose of this application is to apply the knowledge gained from working with the programming languages **C++**, **Java**, and **Rust**, as part of the **CSE310 – Applied Programming** course.

This is the **third stage**, which focuses on developing a **task manager application in Rust**. The application reads from and writes to the same log file (`my_task.txt`) used by the C++ and Java versions, allowing task persistence across multiple programming environments.

By the end of the course, the final result will be a **multi-language task manager**.

[Software demo video](https://www.youtube.com/watch?v=7eyndKGmjtI&t=3157s)

---

# Development Environment

This is a **Rust** console-based application built using:

- **Rust 1.x** (latest stable version)
- **Cargo** (Rust’s build and package manager)
- **VS Code** with the Rust Analyzer extension
- **Standard Rust library** for file I/O, collections, and console interaction
- **GitHub** for source control and versioning

---

## Main Features

- **Add new tasks:** Prompts the user to enter a task description, stores it in memory, and appends it to the file.
- **Mark tasks as completed:** Allows the user to enter a task ID to mark it as completed.
- **Delete tasks:** Removes tasks by their ID.
- **List all tasks:** Displays all tasks with their IDs and completion status from the file.
- **Persistent storage:** All tasks are saved to `my_task.txt` so that they are available in future sessions.

---

# Rust Concepts Demonstrated

- **Variables and Expressions:**  
  Uses both mutable (`let mut`) and immutable (`let`) variables; applies arithmetic and logical expressions.
  
- **Conditionals and Loops:**  
  Implements `if`, `if let`, `match`, `while`, and `for` loops for menu logic and task iteration.
  
- **Functions (Ownership and References):**  
  Demonstrates Rust’s ownership model and borrowing through:
  - `pub fn add(&mut self, description: String)`
  - `pub fn complete(&mut self, id: i32)`
  - `pub fn list(&self)`

- **Structs and Impl (Object-Oriented Design):**  
  Uses `struct Task` and `struct TaskManager` with `impl` blocks to encapsulate data and related functions.

- **Data Structures:**  
  Uses a `Vec<Task>` for dynamic task storage.

- **File I/O:**  
  Reads and writes from a plain text file using `std::fs::{File, OpenOptions}` and `std::io::{BufRead, BufReader, Write}`.

- **Modules:**  
  The program is organized into multiple files (`main.rs`, `manager.rs`, `menu.rs`, `task.rs`) using Rust’s module system.

---

# Useful Websites

- [Rust Official Documentation](https://doc.rust-lang.org/)
- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)
- [W3Schools Rust Tutorial](https://www.w3schools.io/languages/rust-tutorial/)
- [Ownership and Borrowing in Rust](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Structs and Methods](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust Modules](https://doc.rust-lang.org/book/ch07-00-modules.html)
- [Rust Vectors](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Rust File I/O](https://doc.rust-lang.org/std/fs/index.html)

---

# Additional Learning Resources

- [Rust Tutorial – Full Course for Beginners (freeCodeCamp)](https://www.youtube.com/watch?v=MsocPEZBd-M)
- [Learn Rust Programming – Full Tutorial (Edureka)](https://www.youtube.com/watch?v=zF34dRivLOw)
- [Rust Crash Course](https://www.youtube.com/watch?v=ygL_xcavzQ4)

---
