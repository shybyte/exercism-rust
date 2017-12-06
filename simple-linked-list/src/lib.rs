struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            length: 0
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        let new_head = Node { data: element, next: self.head.take() };
        self.head = Some(Box::new(new_head));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut boxed_node) = self.head.take() {
            self.length -= 1;
            self.head = boxed_node.next.take();
            Some(boxed_node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut next = &self.head;
        while let &Some(ref boxed_node) = next {
            list.push(boxed_node.data.clone());
            next = &boxed_node.next;
        }
        list
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(items: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in items {
            list.push(item.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = vec![];
        while let Some(value) = self.pop() {
            vec.push(value);
        }
        vec.reverse();
        vec
    }
}
