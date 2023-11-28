pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

#[allow(unused)]
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let mut new_head = Box::new(Node { value, next: None });
        if let Some(old_head) = self.head.take() {
            new_head.next = Some(old_head);
        }
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            head.value
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod test {
    use crate::link_list::List;

    #[test]
    fn test_push_and_pop() {
        let mut list = List::new();
        assert_eq!(None, list.pop());
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut into_iter = list.into_iter();
        assert_eq!(Some(3), into_iter.next());
        assert_eq!(Some(2), into_iter.next());
        assert_eq!(Some(1), into_iter.next());
        assert_eq!(None, into_iter.next());
    }
}
