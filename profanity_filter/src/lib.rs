

pub fn check_ms(message: &str) -> Result<&str, &str> {
   let lower=message.to_lowercase();
  match lower.find("stupid"){
    Some(_)=> Ok(message),
    None=> Err("ERROR: illegal"),
  }
}