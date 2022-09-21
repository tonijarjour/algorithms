use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T: std::fmt::Debug> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: std::fmt::Debug> Node<T> {
    fn new(value: T, next: Option<Rc<RefCell<Node<T>>>>) -> Self {
        Self { value, next }
    }
}

#[derive(Debug)]
pub struct List<T: std::fmt::Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
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

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), String> {
        let mut inserted = false;

        if index == 0 {
            if let None = self.head {
                // set head to a new node
                self.head = Some(Rc::new(RefCell::new(Node::new(value, None))));
                // tail references the same node
                self.tail = Some(Rc::clone(self.head.as_ref().unwrap()));
            } else {
                // retrieve the old head, replacing it with a new node
                let old_head = std::mem::replace(
                    &mut self.head,
                    Some(Rc::new(RefCell::new(Node::new(value, None)))),
                );
                // have the new head point to the old head
                self.head.as_ref().unwrap().borrow_mut().next = old_head;
            }

            inserted = true;
        } else if index == self.size {
            // set the old tail to point to a new node
            self.tail.as_ref().unwrap().borrow_mut().next =
                Some(Rc::new(RefCell::new(Node::new(value, None))));
            // create a another reference to that node
            let new_tail = Some(Rc::clone(
                &self.tail.as_ref().unwrap().borrow().next.as_ref().unwrap(),
            ));
            // set the tail to that reference
            self.tail = new_tail;

            inserted = true;
        }

        if inserted {
            self.size += 1;
            return Ok(());
        }

        Err(format!("failed to insert at {}", index))
    }

    pub fn push(&mut self, value: T) -> Result<(), String> {
        Self::insert(self, self.size, value)
    }

    pub fn enq(&mut self, value: T) -> Result<(), String> {
        Self::insert(self, 0, value)
    }

    pub fn remove(&mut self, index: usize) -> Result<T, String> {
        if self.size == 0 {
            return Err("remove on empty list".to_string());
        }

        if index >= self.size {
            return Err("index out of bounds".to_string());
        }

        let mut return_val = Err(format!("failed to remove at {}", index));

        if index == 0 {
            // take the head's next, to be set as new head
            let new_head = std::mem::replace(
                &mut self.head.as_ref().unwrap().borrow_mut().next,
                None,
            );

            // if remove while list has single node, set the tail to None
            // otherwise try_unwrap() will fail at taking ownership
            if self.size == 1 {
                self.tail = None;
            }

            // take the value in the head to be dropped
            let hold_head = std::mem::replace(&mut self.head, None);
            return_val = Ok(Rc::try_unwrap(hold_head.unwrap())
                .unwrap()
                .into_inner()
                .value);

            self.head = new_head;
        } else if index == self.size - 1 {
            // new reference to the first element in the list
            let mut current = Rc::clone(self.head.as_ref().unwrap());
            // find the second to last element, to be set as new tail
            for _ in 0..self.size - 2 {
                let next = Rc::clone(current.borrow().next.as_ref().unwrap());
                current = next;
            }
            // set its next value to none
            current.borrow_mut().next = None;

            // take the value in the tail to be dropped
            let hold_tail = std::mem::replace(&mut self.tail, None);
            return_val = Ok(Rc::try_unwrap(hold_tail.unwrap())
                .unwrap()
                .into_inner()
                .value);

            self.tail = Some(current);
        }

        if let Ok(_) = return_val {
            self.size -= 1
        };

        return_val
    }

    pub fn pop(&mut self) -> Result<T, String> {
        Self::remove(self, self.size - 1)
    }

    pub fn deq(&mut self) -> Result<T, String> {
        Self::remove(self, 0)
    }

    pub fn position(&self, value: T) {}

    pub fn get(&self, index: usize) {}
}
