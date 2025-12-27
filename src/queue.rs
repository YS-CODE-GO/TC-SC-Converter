use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed(String), // Duration string
    Error(String),
}

#[derive(Debug, Clone)]
pub struct ConversionTask {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub status: TaskStatus,
    pub start_time: Option<Instant>,
}

pub struct TaskQueue {
    pub tasks: Arc<Mutex<Vec<ConversionTask>>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_task(&self, path: PathBuf) {
        let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(ConversionTask {
            path,
            name,
            size,
            status: TaskStatus::Pending,
            start_time: None,
        });
    }

}
