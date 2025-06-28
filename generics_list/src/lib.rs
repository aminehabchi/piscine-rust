#[derive(Clone, Debug)]
pub struct List<T: std::clone::Clone> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T: std::clone::Clone> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T: std::clone::Clone> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        match &self.head {
            Some(h) => {
                self.head = Some(Node {
                    value,
                    next: Some(Box::new(h.clone())),
                });
            }
            None => {
                self.head = Some(Node {
                    value,
                    next: None,
                });
            }
        }
    }

    pub fn pop(&mut self) {
        match &self.head {
            Some(h) => {
                self.head = match &h.next {
                    Some(n) => Some(*n.clone()),
                    None => None,
                };
            }
            None => {
                return;
            }
        };
    }

    pub fn len(&self) -> usize {
        match &self.head {
            Some(h) => {
                let mut x: usize = 1;
                let mut current = h;
                loop {
                    current = match &current.next {
                        Some(b) => &*b,
                        None => {
                            break;
                        }
                    };
                    x += 1;
                }
                return x;
            }
            None => 0,
        }
    }
}
