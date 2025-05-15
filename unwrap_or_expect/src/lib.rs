pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}


pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server {
        Ok(data) => data.to_string(),
        Err(e) => match security_level {
            Security::Unknown       => panic!(""),
            Security::Message       => panic!("ERROR: program stops"),
            Security::Warning       => "WARNING: check the server".to_string(),
            Security::NotFound      => "Not found: ".to_string()+ e,
            Security::UnexpectedUrl => panic!("{}", e),
        },
    }
}