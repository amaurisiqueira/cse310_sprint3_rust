# Requirements Report – Rust Task Manager

## Basic Requirements

- **Variables**  
  The program makes use of both mutable and immutable variables. Examples include:
  - `let option = get_menu_option();` (immutable)
  - `let mut desc = String::new();` (mutable)

- **Expressions**  
  Arithmetic and logical expressions are used throughout the program. Example:
  - `let new_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;`

- **Conditionals**  
  The code uses `if`, `if let`, and `match` expressions to handle user input and task management logic.

- **Loops**  
  The main program uses a `while` loop to keep the menu running. `for` loops are used to iterate over task vectors and file lines.

- **Functions (Ownership or Reference)**  
  The project defines and uses multiple custom functions, all demonstrating ownership or borrowing:
  - `pub fn new(filename: &str) -> Self`
  - `pub fn add(&mut self, description: String)`
  - `pub fn complete(&mut self, id: i32)`
  - `pub fn delete(&mut self, id: i32)`
  - `pub fn list(&self)`

## Additional Requirement Chosen

- **Use of a Data Structure**  
  The program uses a `Vec<Task>` to store all tasks. This dynamic vector is central to task storage, manipulation, and file persistence.

- **Use of Struct and Impl (Object-Oriented Design)**  
  Custom structs (`Task`, `TaskManager`) are used to encapsulate data. Associated functions are defined in `impl` blocks, modeling object-oriented behavior.

---

## How the Program Works

- The user interacts with a text-based menu in the terminal.
- The menu options include:
  - Add a new task
  - Mark a task as completed
  - Delete a task
  - List all tasks
  - Exit
- Task data is stored in a plain text file named `my_task.txt`, located at the root of the project.
- Each task is stored as a single line in the format: `id|completed|description`.
- The application uses modular files:
  - `main.rs` (main logic and menu control)
  - `menu.rs` (displays and handles the menu)
  - `task.rs` (defines the Task struct)
  - `manager.rs` (handles file I/O and task operations)
- All operations are performed using idiomatic Rust practices, including ownership, borrowing, and safe memory handling.

---

