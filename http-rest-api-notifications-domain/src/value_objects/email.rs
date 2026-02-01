use std::str::FromStr;

use anyhow;
use email_address;

#[derive(Debug, Clone)]
pub struct Email {
    value: email_address::EmailAddress,
}

impl Email {
    pub fn new<S: Into<String>>(value: S) -> anyhow::Result<Self> {
        Ok(Self {
            value: email_address::EmailAddress::from_str(
                value.into().as_str(),
            )?,
        })
    }
    pub fn get_value(&self) -> String {
        self.value.email()
    }
}
