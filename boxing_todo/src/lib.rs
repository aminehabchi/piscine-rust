use std::fs;
use std::io;
use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        if content==""{
            let arr:Vec<Task>=Vec::new();
            return Ok(TodoList { title:"".to_string() ,tasks: arr })
        }
         let parsed = json::parse(&content).unwrap_or("".into());
        println!("parsed {:?}", parsed);
        println!("-------------");

        let arr:Vec<Task>=Vec::new();
        Ok(TodoList { title:"".to_string() ,tasks: arr })
    }
}