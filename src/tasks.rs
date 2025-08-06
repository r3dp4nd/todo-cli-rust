pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
    pub deadline: Option<String>,
}

impl Task {
    pub fn new(id: u32, description: String, deadline: Option<String>) -> Self {
        Self {
            id,
            description,
            done: false,
            deadline,
        }
    }

    pub fn to_string(&self) -> String {
        let deadline_str = self.deadline.clone().unwrap_or_else(|| "-".to_string());
        format!(
            "{},{},{},{}",
            self.id, self.done, deadline_str, self.description
        )
    }

    pub fn from_string(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(4, ',').collect();
        if parts.len() != 4 {
            return None;
        }

        let id = parts[0].parse().ok()?;
        let done = parts[1].parse().ok()?;
        let deadline = if parts[2] == "-" {
            None
        } else {
            Some(parts[2].to_string())
        };
        let description = parts[3].to_string();

        Some(Task {
            id,
            description,
            done,
            deadline,
        })
    }
}
