use std::str::FromStr;

use uuid;

#[derive(Debug, Clone)]
pub struct NotificationID {
    value: uuid::Uuid,
}

impl NotificationID {
    pub fn new(value: Option<&String>) -> anyhow::Result<Self> {
        Ok(match value {
            Some(s) => Self {
                value: uuid::Uuid::from_str(s)?,
            },
            None => Self {
                value: uuid::Uuid::new_v4(),
            },
        })
    }

    pub fn get_value(&self) -> uuid::Uuid {
        self.value
    }
}
