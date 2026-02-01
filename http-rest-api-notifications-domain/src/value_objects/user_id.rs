use std::str::FromStr;

use uuid;

#[derive(Debug, Clone)]
pub struct UserID {
    value: uuid::Uuid,
}

impl UserID {
    pub fn new<S: Into<String>>(value: Option<S>) -> anyhow::Result<Self> {
        Ok(match value {
            Some(s) => Self {
                value: uuid::Uuid::from_str(&s.into())?,
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
