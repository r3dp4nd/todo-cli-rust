pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            done: false,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{}", self.id, self.done, self.description)
    }

    pub fn from_string(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(3, ',').collect();
        if parts.len() != 3 {
            return None;
        }

        let id = parts[0].parse().ok()?;
        let done = parts[1].parse().ok()?;
        let description = parts[2].to_string();

        Some(Task {
            id,
            description,
            done,
        })
    }
}
