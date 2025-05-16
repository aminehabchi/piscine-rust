use std::{error::Error, fmt::Display};

pub enum ParseErr {
    // expected public fields
    pub Empty,
    pub Malformed,
}

impl Display for ParseErr {
    
}

impl Error for ParseErr {

}

pub struct ReadErr {
    pub child_err :Box<dyn Error>,
}

impl Display for ReadErr {
}

impl Error for ReadErr {
}