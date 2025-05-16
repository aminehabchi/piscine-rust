use chrono::prelude::*;

fn now_timestamp() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &str, field_value: String, err: &str) -> Self {
        FormError {
            err: err.to_string(),
            form_values: (field_name.to_owned(), field_value),
            date: now_timestamp(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        let mut letter = false;
        let mut number = false;
        let mut symbol = false;

        for ch in self.password.chars() {
            if ch.is_ascii_alphabetic() {
                letter = true;
            }
            if ch.is_ascii_digit() {
                number = true;
            }
            if !ch.is_ascii_alphanumeric() {
                symbol = true;
            }
        }

        if !letter || !number || !symbol {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}
