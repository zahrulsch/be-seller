use crate::error::OhMyError;
use crate::prelude::*;

pub struct Utils;

impl Utils {
    pub fn email_to_path(email: &str) -> Result<String> {
        let Ok(expression) = regex::Regex::new(r"(\.[\w\d\.]+)$") else {
            return Err(OhMyError::new("regex_error", "gagal parsing regex"))
        };

        let em = expression.replace_all(email, "").to_string();
        Ok(em.replace('@', "_"))
    }
}
