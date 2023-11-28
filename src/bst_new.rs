use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Display;
use std::mem;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<K: Ord + Display, V: Clone + Display> {
    key: K,
    value: V,
    left_child: Link<K, V>,
    right_child: Link<K, V>,
}

pub type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

#[derive(Debug)]
pub struct BinarySearchTree<K: Ord + Display, V: Clone + Display> {
    root: Link<K, V>,
}

#[allow(unused)]
impl<K: Ord + Display, V: Clone + Display> BinarySearchTree<K, V> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert_with_node(node: Link<K, V>, key: K, value: V) -> Link<K, V> {
        match node {
            None => Some(Rc::new(RefCell::new(Node {
                key,
                value,
                left_child: None,
                right_child: None,
            }))),
            Some(old_root) => {
                let mut mut_root = old_root.borrow_mut();
                match key.cmp(&mut_root.key) {
                    Ordering::Equal => mut_root.value = value,
                    Ordering::Less => {
                        mut_root.left_child =
                            Self::insert_with_node(mut_root.left_child.take(), key, value)
                    }
                    Ordering::Greater => {
                        mut_root.right_child =
                            Self::insert_with_node(mut_root.right_child.take(), key, value)
                    }
                }
                Some(old_root.clone())
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.root = Self::insert_with_node(self.root.take(), key, value);
    }

    fn len_of_node(node: &Link<K, V>) -> usize {
        match node {
            None => 0,
            Some(root) => {
                let immut_root = root.borrow();
                Self::len_of_node(&immut_root.left_child)
                    + 1
                    + Self::len_of_node(&immut_root.right_child)
            }
        }
    }

    pub fn len(&self) -> usize {
        Self::len_of_node(&self.root)
    }

    fn get_with_node(node: &Link<K, V>, key: &K) -> Link<K, V> {
        match node {
            None => None,
            Some(root) => {
                let immut_root = root.borrow();
                match key.cmp(&immut_root.key) {
                    Ordering::Equal => Some(root.clone()),
                    Ordering::Less => Self::get_with_node(&immut_root.left_child, key),
                    Ordering::Greater => Self::get_with_node(&immut_root.right_child, key),
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        Self::get_with_node(&self.root, key)
            .as_ref()
            .map(|node| node.borrow().value.clone())
    }

    fn min_of_node(node: &Link<K, V>) -> Link<K, V> {
        match node {
            None => None,
            Some(root) => {
                let left_child = &root.borrow().left_child;
                if left_child.is_none() {
                    node.clone()
                } else {
                    Self::min_of_node(left_child)
                }
            }
        }
    }

    pub fn min(&self) -> Option<V> {
        Self::min_of_node(&self.root).map(|node| node.borrow().value.clone())
    }

    fn max_of_node(node: &Link<K, V>) -> Link<K, V> {
        match node {
            None => None,
            Some(root) => {
                let right_child = &root.borrow().right_child;
                if right_child.is_none() {
                    node.clone()
                } else {
                    Self::max_of_node(right_child)
                }
            }
        }
    }

    pub fn max(&self) -> Option<V> {
        Self::max_of_node(&self.root).map(|node| node.borrow().value.clone())
    }

    fn inorder_by_node(node: &Link<K, V>) {
        if let Some(root) = node {
            let immut_root = root.borrow();
            Self::inorder_by_node(&immut_root.left_child);
            print!("[key:{},value:{}] ", immut_root.key, immut_root.value);
            Self::inorder_by_node(&immut_root.right_child);
        }
    }
    pub fn inorder(&self) {
        Self::inorder_by_node(&self.root);
        println!();
    }

    fn remove_in_node(mut node: &Link<K, V>, remove_key: &K) -> (bool, Link<K, V>) {
        match node {
            None => (false, None),
            Some(root) => {
                let mut mut_root = root.borrow_mut();
                match remove_key.cmp(&mut_root.key) {
                    Ordering::Equal => match (&mut_root.left_child, &mut_root.right_child) {
                        (None, None) => (true, None),
                        (Some(left), None) => (true, Some(left.clone())),
                        (None, Some(right)) => (true, Some(right.clone())),
                        (Some(_), Some(_)) => {
                            if let Some(suffix) = Self::min_of_node(&mut_root.right_child) {
                                let mut mut_suffix = suffix.borrow_mut();
                                mem::swap(&mut mut_suffix.key, &mut mut_root.key);
                                mem::swap(&mut mut_suffix.value, &mut mut_root.value);
                                let (removed, new_right_child) =
                                    Self::remove_in_node(&mut_root.right_child, remove_key);
                                mut_root.right_child = new_right_child;
                                (removed, Some(root.clone()))
                            } else {
                                unreachable!()
                            }
                        }
                    },
                    Ordering::Less => {
                        let (removed, new_left_child) =
                            Self::remove_in_node(&mut_root.left_child, remove_key);
                        mut_root.left_child = new_left_child;
                        (removed, Some(root.clone()))
                    }
                    Ordering::Greater => {
                        let (removed, new_right_child) =
                            Self::remove_in_node(&mut_root.right_child, remove_key);
                        mut_root.right_child = new_right_child;
                        (removed, Some(root.clone()))
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, removed: &K) -> bool {
        let (removed, new_root) = Self::remove_in_node(&self.root, removed);
        self.root = new_root;
        removed
    }

    pub fn width_first(&self) {
        let mut queue = VecDeque::new();
        queue.push_back(self.root.clone());
        while let Some(Some(node)) = queue.pop_front() {
            let immut_node = node.borrow();
            print!("[{},{}]", immut_node.key, immut_node.value);
            if immut_node.left_child.is_some() {
                queue.push_back(immut_node.left_child.clone());
            }
            if immut_node.right_child.is_some() {
                queue.push_back(immut_node.right_child.clone());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bst_new::BinarySearchTree;

    #[test]
    fn test_insert() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1, "1");
        bst.insert(2, "2");
        bst.insert(3, "3");
        assert_eq!(bst.len(), 3);
    }

    #[test]
    fn test_get() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1, "1");
        bst.insert(2, "2");
        bst.insert(3, "3");
        bst.insert(3, "4");
        assert_eq!(bst.get(&1), Some("1"));
        assert_eq!(bst.get(&2), Some("2"));
        assert_eq!(bst.get(&4), None);
        assert_eq!(bst.len(), 3);
    }

    #[test]
    fn test_delete() {
        let mut bst = BinarySearchTree::new();
        bst.insert(3, "3");
        bst.insert(2, "2");
        bst.insert(4, "4");
        bst.insert(6, "6");
        bst.insert(8, "8");
        bst.insert(7, "7");
        bst.insert(5, "5");
        bst.insert(1, "1");

        assert_eq!(bst.len(), 8);
        bst.inorder();
        assert!(bst.remove(&1));
        assert!(bst.remove(&8));
        assert_eq!(bst.len(), 6);
        bst.inorder();
    }

    #[test]
    fn test_wf() {
        let mut bst = BinarySearchTree::new();
        bst.insert(3, "3");
        bst.insert(2, "2");
        bst.insert(4, "4");
        bst.insert(6, "6");
        bst.insert(8, "8");
        bst.insert(7, "7");
        bst.insert(5, "5");
        bst.insert(1, "1");

        bst.width_first();
    }
}
