#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Box<Option<Worker>>;

#[derive(Debug, Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        let l: Link = Box::new(None);
        WorkEnvironment {
            grade: l,
        }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let mut new_worker = Worker {
            role,
            name,
            next: Box::new(None),
        };

        if self.grade.as_ref().is_none() {
            self.grade = Box::new(Some(new_worker));
            return;
        }

        new_worker.next = std::mem::replace(&mut self.grade, Box::new(None));
        self.grade = Box::new(Some(new_worker));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        if self.grade.as_ref().is_none() {
            return None;
        }
        if let Some(worker) = self.grade.as_ref() {
            let name = worker.name.clone();
            match worker.next.as_ref() {
                Some(w) => {
                    self.grade = Box::new(Some(w.clone()));
                }
                None => {
                    self.grade = Box::new(None);
                }
            }
            return Some(name);
        }
        None
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        if self.grade.as_ref().is_none() {
            return None;
        }
        if let Some(worker) = self.grade.as_ref() {
            return Some((worker.name.clone(), worker.role.clone()));
        }
        None
    }
}
