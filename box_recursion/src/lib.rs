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
        let new_worker = Box::new(
            Some(Worker {
                role,
                name,
                next: Box::new(None),
            })
        );
        if self.grade.as_ref().is_none() {
            self.grade = new_worker;
            return;
        }
        let mut current = &mut self.grade;
        while let Some(worker) = current.as_mut() {
            if worker.next.as_ref().is_none() {
                worker.next = new_worker;
                return;
            }
            current = &mut worker.next;
        }
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        if self.grade.as_ref().is_none() {
            return None;
        }
        if let Some(worker) = self.grade.as_mut() {
            if worker.next.as_ref().is_none() {
                let t = Some(worker.name.clone());
                self.grade = Box::new(None);
                return t;
            }
        }

        let mut current = &mut self.grade;
        let mut current1 = Box::new(None);
        if let Some(worker) = current.as_mut() {
            current1 = &mut worker.next;
        }
        while let Some(worker1) = current1.as_mut() {
            if worker1.next.as_ref().is_none() {
                // let t = Some(worker1.name.clone());
                // if let Some(worker) = current.as_mut() {
                //     worker.next = Box::new(None);
                // }
                return None;
            }
            current1 = &mut worker1.next;
            if let Some(worker) = current.as_mut() {
                current = &mut worker.next;
            }
        }

        None
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        if self.grade.as_ref().is_none() {
            return None;
        }

        let mut current = &self.grade;
        while let Some(worker) = current.as_ref() {
            if worker.next.as_ref().is_none() {
                return Some((worker.role.clone(), worker.name.clone()));
            }
            current = &worker.next;
        }
        None
    }
}
