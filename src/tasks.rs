use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

    pub fn deadline_as_time(&self) -> Option<SystemTime> {
        let date_str = self.deadline.as_ref()?;

        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return None;
        }

        let year = parts[0].parse::<u64>().ok()?;
        let month = parts[1].parse::<u64>().ok()?;
        let day = parts[2].parse::<u64>().ok()?;

        let day_since_epoch = (year - 1970) * 365 + (month - 1) * 30 + (day - 1);

        Some(UNIX_EPOCH + Duration::from_secs(day_since_epoch as u64 * 24 * 60 * 60))
    }

    pub fn is_expired(&self) -> bool {
        if let Some(deadline_time) = self.deadline_as_time() {
            if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
                return deadline_time < UNIX_EPOCH + now;
            }
        }
        false
    }
}
