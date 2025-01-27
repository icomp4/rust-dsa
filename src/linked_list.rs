// Implementation of a linked list
// TODO - Add remove func

struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>
}

pub struct LinkedList<T> {
    head: Option<Node<T>>,
    length: u64
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0
        }
    }

    pub fn new_from(data: &[T]) -> Self
    where T: Clone
    {
        let mut list = Self::new();
        for item in data.iter().rev() {
            let node = Node{
                data: item.clone(),
                next: list.head.take().map(Box::new)
            };
            list.head = Some(node);
            list.length+=1;
        }
        list
    }

    pub fn add(&mut self, data: T) {
        let new_node = Node { data, next: None };

        if self.head.is_none() {
            self.head = Some(new_node);
            self.length += 1;
            return;
        }

        let mut current = self.head.as_mut().unwrap();
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = Some(Box::new(new_node));
        self.length += 1;
    }

    pub fn add_to_front(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take().map(Box::new)
        };
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn contains(&self, data: T) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            if node.data == data {
                return true;
            }
            current = node.next.as_deref()
        }
        false
    }

    pub fn print(&self)
    where T: std::fmt::Display
    {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            println!("{}", node.data);
            current = node.next.as_deref();
        }
    }

}