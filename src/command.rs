use std::env;

pub enum ListFilter {
    All,
    Done,
    Pending,
}

pub enum Command {
    Add {
        description: String,
        deadline: Option<String>,
    },
    List {
        filter: ListFilter,
    },
    Done {
        id: u32,
    },
    Edit {
        id: u32,
        new_desc: Option<String>,
        new_deadline: Option<String>,
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

                let candidate = args.last().unwrap().to_string();

                let (description, date) = if is_valid_date(&candidate) {
                    let description = args[2..args.len() - 1].join(" ");
                    (description, Some(candidate))
                } else {
                    let description = args[2..].join(" ");
                    (description, None)
                };

                Command::Add {
                    description,
                    deadline: date,
                }
            }
            "list" => {
                let filter = if args.len() >= 3 {
                    match args[2].as_str() {
                        "done" => ListFilter::Done,
                        "pending" => ListFilter::Pending,
                        _ => ListFilter::All,
                    }
                } else {
                    ListFilter::All
                };

                Command::List { filter }
            }
            "done" => {
                if args.len() < 3 {
                    return Command::Help;
                }

                match args[2].parse::<u32>() {
                    Ok(id) => Command::Done { id },
                    Err(_) => Command::Help,
                }
            }
            "edit" => {
                if args.len() < 3 {
                    return Command::Help;
                }

                let id = args[2].parse::<u32>().unwrap_or_else(|_| {
                    println!("ID valido");
                    std::process::exit(1)
                });

                let mut new_desc = None;
                let mut new_deadline = None;
                let mut i = 3;

                while i < args.len() {
                    match args[i].as_str() {
                        "--desc" => {
                            i += 1;
                            if i < args.len() {
                                new_desc = Some(args[i].clone());
                            }
                        }
                        "--date" => {
                            i += 1;
                            if i < args.len() {
                                let date = args[i].clone();
                                if is_valid_date(&date) {
                                    new_deadline = Some(date);
                                } else {
                                    println!("Formato de fecha inválido. Usa YYYY-MM-DD.");
                                    std::process::exit(1);
                                }
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                if new_desc.is_none() && new_deadline.is_none() {
                    println!("Nada que editar. Usa --desc o --date.");
                    std::process::exit(1);
                }

                Command::Edit {
                    id,
                    new_desc,
                    new_deadline,
                }
            }

            _ => Command::Help,
        }
    }
}

pub fn is_valid_date(date: &str) -> bool {
    if date.len() != 10 {
        return false;
    }

    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    let (year_str, month_str, day_str) = (parts[0], parts[1], parts[2]);

    if year_str.len() != 4 || month_str.len() != 2 || day_str.len() != 2 {
        return false;
    }

    let year = match year_str.parse::<u32>() {
        Ok(y) => y,
        Err(_) => return false,
    };
    let month = match month_str.parse::<u32>() {
        Ok(m) => m,
        Err(_) => return false,
    };
    let day = match day_str.parse::<u32>() {
        Ok(d) => d,
        Err(_) => return false,
    };

    if year < 1900 || month == 0 || month > 12 || day == 0 || day > 31 {
        return false;
    }

    true
}
