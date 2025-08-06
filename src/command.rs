use std::env;

pub enum Command {
    Add {
        description: String,
        deadline: Option<String>,
    },
    List,
    Done {
        id: u32,
    },
    Help,
}

impl Command {
    pub fn parse_args() -> Self {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Command::Help;
        }

        match args[1].as_str() {
            "add" => {
                if args.len() < 3 {
                    return Command::Help;
                }
                let (description, deadline) = if args.len() >= 4 {
                    let deadline = args.last().unwrap().to_string();
                    let description = args[2..args.len() - 1].join(" ");
                    (description, Some(deadline))
                } else {
                    let description = args[2..].join(" ");
                    (description, None)
                };
                Command::Add {
                    description,
                    deadline,
                }
            }
            "list" => Command::List,
            "done" => {
                if args.len() < 3 {
                    return Command::Help;
                }

                match args[2].parse::<u32>() {
                    Ok(id) => Command::Done { id },
                    Err(_) => Command::Help,
                }
            }
            _ => Command::Help,
        }
    }
}
