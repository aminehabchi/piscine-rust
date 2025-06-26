#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Box<Option<Worker>>;

#[derive(Debug)]
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
            next:Box::new(self.grade.take()),
        };

        self.grade = Box::new(Some(new_worker));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|node| {
            let name = node.name.clone();
            self.grade = node.next;
            name
        })
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
