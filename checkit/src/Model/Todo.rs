use std::time::SystemTime;

pub struct Todo {
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Default for Todo {
    fn default() -> Todo {
        Todo {
            title: String::new(),
            description: String::new(),
            completed: false,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}

impl Todo {
    pub fn mark_as_completed(&mut self) {
        self.completed = true;
        self.updated_at = SystemTime::now();
    }

    pub fn update(&mut self, title: String, description: String) {
        self.title = title;
        self.description = description;
        self.updated_at = SystemTime::now();
    }
}