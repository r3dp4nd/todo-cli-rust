mod command;
mod file;
mod tasks;

use crate::command::{Command, ListFilter};
use crate::tasks::Task;

fn print_usage() {
    println!("Uso:");
    println!("  add <descripcion>");
    println!("  list [all|pending|done]");
    println!("  done <id>");
}

fn main() {
    let cmd = Command::parse_args();
    let mut tasks = file::load_tasks("tasks.db");

    // Commands
    match cmd {
        Command::Add {
            description,
            deadline,
        } => {
            let task = Task::new(tasks.len() as u32, description, deadline);
            tasks.push(task);
            file::save_tasks("tasks.db", &tasks);
            println!("Tarea agregada.");
        }
        Command::List { filter } => {
            let filtered: Vec<&Task> = match filter {
                ListFilter::All => tasks.iter().collect(),
                ListFilter::Done => tasks.iter().filter(|t| t.done).collect(),
                ListFilter::Pending => tasks.iter().filter(|t| !t.done).collect(),
            };

            if filtered.is_empty() {
                println!("No hay tareas.");
            } else {
                for task in filtered {
                    let status = if task.done { "[✓]" } else { "[ ]" };
                    let deadline = task.deadline.as_deref().unwrap_or("sin fecha");
                    let expired = if !task.done && task.is_expired() {
                        " ⚠ expired"
                    } else {
                        ""
                    };
                    println!(
                        "{} {} - {} (vence: {}){}",
                        status, task.id, task.description, deadline, expired
                    );
                }
            }
        }
        Command::Done { id } => match tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.done = true;
                file::save_tasks("tasks.db", &tasks);
                println!("Tarea marcada como completada.");
            }
            None => println!("Tarea con ID {} no encontrada.", id),
        },
        _ => print_usage(),
    }
}
