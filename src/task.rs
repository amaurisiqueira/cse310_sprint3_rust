#[derive(Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub completed: bool,
    pub description: String,
}

impl Task {
    pub fn new(id: i32, description: String) -> Self {
        Task {
            id,
            completed: false,
            description,
        }
    }
}
