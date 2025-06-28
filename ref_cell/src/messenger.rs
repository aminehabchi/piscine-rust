use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker { logger, max }
    }

    pub fn set_value(&self, tracker: &Rc<usize>) {
        let count = Rc::strong_count(tracker);
        let percentage = (count as f32 / self.max as f32) * 100.0;
        let rounded = percentage.round() as usize;

        if percentage >= 100.0 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70.0 {
            self.logger.warning(&format!(
                "you have used up over {}% of your quota! Proceeds with precaution",
                rounded
            ));
        } else {
            self.logger.info(&format!(
                "you are using up to {}% of your quota",
                rounded
            ));
        }
    }

    pub fn peek(&self, tracker: &Rc<usize>) {
        let count = Rc::strong_count(tracker);
        let percentage = (count as f32 / self.max as f32) * 100.0;
        let msg = format!("you are using up to {}% of your quota", percentage.round() as usize);
        self.logger.info(&msg);
    }
}
