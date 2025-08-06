mod command;
mod file;
mod tasks;

use crate::command::Command;
use crate::tasks::Task;

fn print_usage() {
    println!("Uso:");
    println!("  add <descripcion>");
    println!("  list");
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
        Command::List => {
            if tasks.is_empty() {
                println!("No hay tareas.");
            } else {
                for task in &tasks {
                    let status = if task.done { "[✓]" } else { "[ ]" };
                    let deadline = task.deadline.as_deref().unwrap_or("sin fecha");
                    println!(
                        "{} {} - {} (vence: {})",
                        status, task.id, task.description, deadline
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
