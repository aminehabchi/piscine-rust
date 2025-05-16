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
    pub fn new(field_name: String, field_value: String, err: String) -> Self {
        FormError {
            err: err.to_string(),
            form_values: (field_name.to_string(), field_value.to_string()),
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
            return Err(FormError::new(
                "first_name".to_string(),
                self.name.to_string(),
                "Username is empty".to_string(),
            ));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.to_string(),
                "Password should be at least 8 characters long".to_string(),
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
                "password".to_string(),
                self.password.to_string(),
                "Password should be a combination of ASCII numbers, letters and symbols".to_string(),
            ));
        }

        Ok(())
    }
}
