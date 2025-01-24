// Implementation of a linked list

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            next: None
        }
    }

    pub fn add(&mut self, data: T) {
        match self.next {
            None => {
                self.next = Some(Box::new(Node::new(data)));
            }
            Some(ref mut next_node) => {
                next_node.add(data);
            }
        }
    }

    pub fn print(&self) where T: std::fmt::Display {
        let mut current = self;
        while let Some(ref next) = current.next {
            print!("{} -> ", current.data);
            current = next;
        }
        println!("{}", current.data);
     }
}