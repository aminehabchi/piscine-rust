pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}


pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level{
        Security::Unknown       => match server{
                                        Ok(m) => return m.to_string(),
                                        Err(e)=> return e.to_string(),
                                    },
        Security::Message       => match server{
                                        Ok(m) => return m.to_string(),
                                        Err(_e)=> return "ERROR: program stops".to_string(),
                                    },
        Security::Warning       => match server{
                                        Ok(m) => return m.to_string(),
                                        Err(_e)=> return "WARNING: check the server".to_string(),
                                    },
        Security::NotFound      =>  match server{
                                        Ok(m) => return m.to_string(),
                                        Err(e)=> return format!("Not found: {}", e),
                                    },
        Security::UnexpectedUrl => match server{
                                        Ok(m) => return m.to_string(),
                                        Err(e)=> return e.to_string(),
                                    },
    }
}