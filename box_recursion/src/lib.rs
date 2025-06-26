#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker {
            role,
            name,
            next: self.grade.take(),
        });
        self.grade = Some(new_worker);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(mut boxed) = self.grade.take() {
            self.grade = boxed.next.take();
            Some(boxed.name)
        } else {
            None
        }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        let mut current = self.grade.as_ref();

        while let Some(worker) = current {
            if worker.next.is_none() {
                return Some((worker.name.clone(),worker.role.clone()));
            }
            current = worker.next.as_ref();
        }

        None
    }
}
