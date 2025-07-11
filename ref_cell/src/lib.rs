use std::collections::HashMap;
pub use std::rc::Rc;
pub use std::cell::RefCell;

pub mod messenger;
pub use messenger::*;

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(track_value: usize) -> Self {
        Worker {
            track_value: Rc::new(track_value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.all_messages.borrow_mut().push(msg.to_string());
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg[9..].to_string());
    }

    fn info(&self, msg: &str) {
        self.all_messages.borrow_mut().push(msg.to_string());
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg[6..].to_string());
    }

    fn error(&self, msg: &str) {
        self.all_messages.borrow_mut().push(msg.to_string());
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg[7..].to_string());
    }
}
