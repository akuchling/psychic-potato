
use std::fmt;
use std::fs::{OpenOptions, File};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};  // Include the `Error` type.
use std::path::PathBuf;

use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(tasks)
}


pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Remove the task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    // Truncate the file.
    file.set_len(0)?;

    // Write the modified task list back into the file.
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new().read(true).open(journal_path)?;
    // Parse the file and collect the tasks.
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}
