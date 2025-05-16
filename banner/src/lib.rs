use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        let long_hand = format!("--{}", name);
        let short_hand = match name.chars().next() {
            Some(c) => format!("-{}", c),
            None => "-".to_string(), 
        };
        Flag {
            desc: d.to_string(),
            long_hand,
            short_hand,
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand,func);
        self.flags.insert(flag.long_hand,func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
       let func=self.flags.get(input).unwrap();
      let result=func(argv[0],argv[1]);
        match result {
            Ok(r)=> return Ok(r),
            Err(e)=> return Err(e.to_string()),
        }
    }
}

    pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let n1:f64= a.parse()?;
    let n2:f64= b.parse()?;
    let r:f64=n1/n2;
        Ok(r.to_string())
    }

    pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
        let n1:f64= a.parse()?;
        let n2:f64= b.parse()?;
        let r:f64=n1%n2;
        Ok(r.to_string())
    }