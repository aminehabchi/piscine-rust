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
                                        Err(_) => panic!(),
                                    },
        Security::Message       => match server{
                                        Ok(m) => return m.to_string(),
                                        Err(_e)=> panic!("ERROR: program stops"),
                                    },
        Security::Warning       => match server{
                                        Ok(m) => return m.to_string(),
                                        Err(_)=> return "WARNING: check the server".to_string(),
                                    },
        Security::NotFound      =>  match server{
                                        Ok(m) => return m.to_string(),
                                        Err(e)=> return format!("Not found: {}", e),
                                    },
        Security::UnexpectedUrl => match server{
                                        Ok(url) => panic!("{}", url),
                                        Err(err) => err.to_string(),
                                    },
    }
}