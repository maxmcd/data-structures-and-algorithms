// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
#[derive(Debug)]
pub struct Node<T: PartialOrd> {
    is_black: bool,
    value: T,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Debug)]
pub struct BinaryTree<T: PartialOrd> {
    nodes: Vec<Node<T>>,
    head: Option<usize>,
}

impl<T: PartialOrd> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree {
            nodes: Vec::new(),
            head: None,
        }
    }
    pub fn add(&mut self, value: T) {
        let node = if self.head.is_none() {
            self.head = Some(0);
            Node {
                is_black: true,
                value: value,
                parent: None,
                left: None,
                right: None,
            }
        } else {
            self._add(0, value)
        };
        self.nodes.push(node);
    }

    fn _add(&mut self, index: usize, value: T) -> Node<T> {
        if value > self.nodes[index].value {
            if self.nodes[index].right.is_some() {
                let index = self.nodes[index].right.unwrap();
                self._add(index, value)
            } else {
                self.nodes[index].right = Some(self.nodes.len());
                Node {
                    is_black: false,
                    value: value,
                    parent: Some(index),
                    left: None,
                    right: None,
                }
            }
        } else {
            if self.nodes[index].left.is_some() {
                let index = self.nodes[index].left.unwrap();
                self._add(index, value)
            } else {
                self.nodes[index].left = Some(self.nodes.len());
                Node {
                    is_black: false,
                    value: value,
                    parent: Some(index),
                    left: None,
                    right: None,
                }
            }
        }
    }
}

// black aunt rotate
// red aunt color flip

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_binary_basic() {
        let mut tree: BinaryTree<u8> = BinaryTree::new();
        tree.add(5);
        tree.add(2);
        tree.add(1);
        tree.add(3);
        tree.add(6);
        tree.add(8);
        println!("{:?}", tree);
    }
}
