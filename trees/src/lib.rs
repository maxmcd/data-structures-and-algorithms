use std::fmt;

#[derive(Debug)]
struct Node<T: PartialOrd> {
    value: T,
    // color: bool,
    left: Link<T>,
    right: Link<T>,
}

#[derive(Debug)]
struct Link<T: PartialOrd>(Option<Box<Node<T>>>);

impl<T: PartialOrd> Link<T> {
    fn create_or_add(&mut self, value: T) {
        if self.0.is_some() {
            self.0.as_mut().unwrap().add(value);
        } else {
            self.0 = Some(Box::new(Node::new(value)));
        }
    }
}

impl<T: PartialOrd> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: value,
            left: Link(None),
            right: Link(None),
        }
    }
    fn add(&mut self, value: T) {
        if value > self.value {
            self.right.create_or_add(value);
        } else {
            self.left.create_or_add(value);
        }
    }
}

impl<T: PartialOrd> Link<T>
where
    T: std::fmt::Display,
{
    fn dsp_or_none(&self, depth: usize) -> String {
        match self.0.as_ref() {
            Some(n) => n.child_disp(depth + 1),
            None => String::new(),
        }
    }
}

impl<T: PartialOrd> Node<T>
where
    T: std::fmt::Display,
{
    fn child_disp(&self, depth: usize) -> String {
        String::from(format!(
            "\n{}{}{}{}",
            " ".repeat(depth),
            self.value,
            self.right.dsp_or_none(depth),
            self.left.dsp_or_none(depth)
        ))
    }
}

impl<T: PartialOrd> fmt::Display for Node<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.child_disp(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut head: Node<u8> = Node::new(5u8);
        head.add(2);
        head.add(1);
        head.add(3);
        head.add(6);
        head.add(8);
        println!("{}", head);
    }
}
