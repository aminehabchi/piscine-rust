use std::cell::{ RefCell, Cell };

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers::default()
    }

    pub fn new_worker(&self, cmd: String) -> (usize, Thread) {
        self.states.borrow_mut().push(true);
        let pid = self.states.borrow().len() - 1;
        (pid, Thread::new(pid, cmd, self))
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        !self.states.borrow()[id]
    }

    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();

        if !states[id] {
            panic!("{} is already dropped", id);
        } else {
            states[id] = false;
            let old = self.drops.get();
            self.drops.set(old + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    cmd: String,
    parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new(pid: usize, cmd: String, parent: &'a Workers) -> Thread<'a> {
        Thread { pid, cmd, parent }
    }

    pub fn skill(self) {
        self.parent.add_drop(self.pid);
    }
}
