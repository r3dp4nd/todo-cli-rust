use crate::tasks::Task;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn load_tasks(path: &str) -> Vec<Task> {
    if !Path::new(path).exists() {
        return Vec::new();
    }

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| Task::from_string(&line.unwrap()))
        .collect()
}

pub fn save_tasks(path: &str, tasks: &[Task]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    for task in tasks {
        writeln!(file, "{}", task.to_string()).unwrap();
    }
}
