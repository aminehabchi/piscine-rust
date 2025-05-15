pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap_or_else(|| panic!()),

        Security::Message => server.unwrap_or_else(|| panic!("ERROR: program stops")),

        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),

        Security::NotFound => {
            server.unwrap_or_else(|err| format!("Not found: {}", err)).to_string()
        }

        Security::UnexpectedUrl => {
            server.unwrap_or_else(|err| panic!("{}", err));
            server.unwrap().to_string()
        }
    }
}