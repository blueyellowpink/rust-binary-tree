use std::cmp::Ordering;

#[derive(Debug, Default)]
pub struct Tree<T: Default + Ord>(Option<Box<Node<T>>>);

impl<T: Default + Ord> Tree<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn value(&self) -> Option<&T> {
        if let Some(node) = &self.0 {
            return Some(&node.value);
        }

        None
    }

    pub fn left(&self) -> Option<&Self> {
        if let Some(node) = &self.0 {
            return Some(&node.left);
        }

        None
    }

    pub fn right(&self) -> Option<&Self> {
        if let Some(node) = &self.0 {
            return Some(&node.right);
        }

        None
    }

    pub fn insert(&mut self, value: T) {
        let mut current: &mut Tree<T> = self;

        while let Some(ref mut node) = current.0 {
            match value.cmp(&node.value) {
                Ordering::Greater => {
                    current = &mut node.right;
                }
                Ordering::Less => {
                    current = &mut node.left;
                }
                Ordering::Equal => {
                    current = &mut node.left;
                }
            }
        }

        current.0 = Some(Box::new(Node::new(value)));
    }
}

#[derive(Debug, Default)]
pub struct Node<T: Default + Ord> {
    pub value: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

impl<T: Default + Ord> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: Tree(None),
            right: Tree(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_add_new_node() {
        let mut tree = Tree::<u32>::new();
        tree.insert(5);
        tree.insert(6);
        tree.insert(7);
        tree.insert(2);

        assert_eq!(tree.left().unwrap().value().unwrap(), &2);
        assert_eq!(tree.right().unwrap().value().unwrap(), &6);
    }
}
