use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    height: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + std::fmt::Debug> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }
}

pub struct AVLTree<T: Ord> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord + std::fmt::Debug> AVLTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(n) = current {
            current = match value.cmp(&n.value) {
                Ordering::Equal => return true,
                Ordering::Less => &n.left,
                Ordering::Greater => &n.right,
            }
        }
        false
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut current = &mut self.root;
        while let Some(n) = current {
            current = match value.cmp(&n.value) {
                Ordering::Less => &mut n.left,
                Ordering::Greater => &mut n.right,
                Ordering::Equal => return false,
            }
        }
        *current = Some(Box::new(Node::new(value)));
        self.size += 1;
        true
    }

    pub fn remove(&mut self, value: T) -> bool {
        if let None = self.root { return false; }

        let mut current = &mut self.root;
        if let Some(n) = current {
            match value.cmp(&n.value) {
                Ordering::Equal => println!("Hello"),
                _ => println!("Not Equal")
            }
        }
        false
    }
}
