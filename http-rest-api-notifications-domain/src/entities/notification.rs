use crate::value_objects::{CronExpression, Email, NotificationID, UserID};
use anyhow;

pub struct Notification {
    pub id: NotificationID,
    pub rev: Option<String>,
    pub user_id: UserID,
    pub emails: Vec<Email>,
    pub cron_expression: CronExpression,
    pub title: Option<String>,
    pub body: String,
}

impl Notification {
    pub fn new(
        user_id: String,
        emails: Vec<String>,
        cron_expression: String,
        title: Option<String>,
        body: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            id: NotificationID::new(None)?,
            rev: None,
            user_id: UserID::new(Some(user_id))?,
            emails: emails
                .iter()
                .map(|s| Email::new(s))
                .collect::<anyhow::Result<Vec<Email>>>()?,
            cron_expression: CronExpression::new(cron_expression)?,
            title: title,
            body: body.into(),
        })
    }

    pub fn get_user_id(&self) -> UserID {
        self.user_id.clone()
    }

    pub fn get_cron_expression(&self) -> String {
        self.cron_expression.get_value()
    }

    pub fn get_emails(&self) -> Vec<Email> {
        self.emails.clone()
    }
}

#[cfg(test)]
mod tests {
    use string_from::Str;
    use uuid;

    use super::*;

    #[test]
    fn invalid_user_uuid() {
        assert!(Notification::new(
            String::from("invalid uuid"),
            vec![Str!("john@doe.com")],
            String::from("* * * * * * *"),
            Some(String::from("Hello!")),
            String::from("World!"),
        )
        .is_err());
    }

    #[test]
    fn invalid_cron_expression() {
        assert!(Notification::new(
            uuid::Uuid::new_v4().to_string(),
            vec![Str!("john@doe.com")],
            String::from("*"),
            Some(String::from("Hello!")),
            String::from("World!"),
        )
        .is_err());
    }

    #[test]
    fn invalid_emails() {
        assert!(Notification::new(
            uuid::Uuid::new_v4().to_string(),
            vec![Str!("incorrect email")],
            String::from("* * * * * * *"),
            Some(String::from("Hello!")),
            String::from("World!"),
        )
        .is_err());
    }
}
