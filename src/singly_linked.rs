use std::rc::Rc;

#[derive(Debug)]
struct Node<T: std::fmt::Debug> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

impl<T: std::fmt::Debug> Node<T> {
    fn new(value: T, next: Option<Rc<Node<T>>>) -> Self {
        Self { value, next }
    }
}

#[derive(Debug)]
pub struct List<T: std::fmt::Debug> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
    size: usize,
}

impl<T: std::fmt::Debug> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if index == 0 {
            if let None = self.head {
                self.head = Some(Rc::new(Node::new(value, None)));
                self.tail = Some(Rc::clone(self.head.as_ref().unwrap()));
            } else {
                let old_head = std::mem::replace(&mut self.head, None);
                self.head = Some(Rc::new(Node::new(value, old_head)));
            }
        } else if index == self.size {
            let node = Some(Rc::new(Node::new(value, None)));
            let old_tail = std::mem::replace(&mut self.tail, None);
        }
        self.size += 1;
    }

    pub fn push(&mut self, value: T) {
        Self::insert(self, self.size, value);
    }

    pub fn enq(&mut self, value: T) {
        Self::insert(self, 0, value);
    }

    pub fn remove(&mut self, index: usize) {}

    pub fn pop(&mut self) {}

    pub fn deq(&mut self) {}

    pub fn position(&self, value: T) {}

    pub fn get(&self, index: usize) {}
}
