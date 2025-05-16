

pub fn check_ms(message: &str) -> Result<&str, &str> {
  if message==""{
    return Err("ERROR: illegal");
  }
   let lower=message.to_string().to_lowercase();
  match lower.find("stupid"){
    Some(_)=> Err("ERROR: illegal"),
    None=> Ok(message),
  }
}