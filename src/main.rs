mod file;
mod tasks;

use crate::tasks::Task;
use std::env;

fn print_usage() {
    println!("Uso:");
    println!("  add <descripcion>");
    println!("  list");
    println!("  done <id>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = args[1].as_str();
    let mut tasks = file::load_tasks("tasks.db");

    // Commands
    match command {
        "add" => {
            if args.len() < 3 {
                println!("Falta la descripción de la tarea.");
                return;
            }

            let (description, deadline) = if args.len() >= 4 {
                let deadline = args.last().unwrap().to_string();
                let description = args[2..args.len() - 1].join(" ");
                (description, Some(deadline))
            } else {
                let description = args[2..].join(" ");
                (description, None)
            };

            let task = Task::new(tasks.len() as u32, description, deadline);
            tasks.push(task);
            file::save_tasks("tasks.db", &tasks);
            println!("Tarea agregada.");
        }
        "list" => {
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
        "done" => {
            if args.len() < 3 {
                println!("Falta el ID de la tarea.");
            } else if let Ok(id) = args[2].parse::<u32>() {
                let mut found = false;
                for task in &mut tasks {
                    if task.id == id {
                        task.done = true;
                        found = true;
                        break;
                    }
                }

                if found {
                    file::save_tasks("tasks.db", &tasks);
                    println!("Tarea marcada como completada.");
                } else {
                    println!("Tarea con ID {} no encontrada.", id);
                }
            } else {
                println!("ID inválido.");
            }
        }
        _ => print_usage(),
    }
}
