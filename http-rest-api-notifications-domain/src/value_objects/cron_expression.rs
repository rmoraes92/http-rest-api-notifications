use anyhow;
use chrono;
use cron_tab;

#[derive(Debug, Clone)]
pub struct CronExpression {
    value: String,
}

impl CronExpression {
    pub fn new<S: Into<String>>(value: S) -> anyhow::Result<Self> {
        let value = value.into();
        match cron_tab::Cron::new(chrono::Utc).add_fn(&value, || {}) {
            Ok(_) => Ok(Self { value }),
            Err(e) => anyhow::bail!("{}", e),
        }
    }
    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_expression() {
        assert!(CronExpression::new("*").is_err());
        assert!(CronExpression::new("*")
            .err()
            .unwrap()
            .to_string()
            .starts_with("Invalid cron expression: *"),);
    }
}
