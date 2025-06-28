use std::rc::Rc;
use std::cell::RefCell;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger, // trait object needs lifetime
    pub value: usize,
    pub max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker {
            logger,
            value: 0,
            max,
        }
    }

    // You want to update value based on the Rc<RefCell<usize>>
    pub fn set_value(&self, tracker: &Rc<RefCell<usize>>) {
         // update internal value from tracker

        let percentage: f32 = (Rc::strong_count(tracker) as f32) / (self.max as f32);

        if percentage >= 1.0 {
            self.logger.error("You are over your quota!");
        } else if percentage >= 0.7 {
            self.logger.warning(
                &format!(
                    "You have used over {}% of your quota! Proceed with caution.",
                    (percentage * 100.0) as usize
                )
            );
        } else {
            self.logger.info(
                &format!("You are using up to {}% of your quota.", (percentage * 100.0) as usize)
            );
        }
    }

    pub fn peek(&self, tracker: &Rc<RefCell<usize>>) {
        let percentage: f32 = (Rc::strong_count(tracker) as f32) / (self.max as f32);
        let msg = format!(
            "Info: you are using up to {}% of your quota",
            (percentage * 100.0) as usize
        );
        self.logger.info(&msg)
    }
}