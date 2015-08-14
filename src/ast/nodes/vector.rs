use std::{fmt, iter, ops};
use ast::Node;
use utils::format_vec;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    vector: Vec<Node>,
}

impl Vector {
    pub fn new(vector: Vec<Node>) -> Vector {
        Vector { vector: vector }
    }

    pub fn len(&self) -> usize {
        self.vector.len()
    }
}

pub struct VectorIntoIterator<'a> {
    vector: &'a Vector,
    index: usize
}

impl<'a> iter::Iterator for VectorIntoIterator<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<&'a Node> {
        if self.index < self.vector.len() {
            let cur = &self.vector.vector[self.index];
            self.index += 1;
            Some(cur)
        } else {
            None
        }
    }
}

impl<'a> iter::IntoIterator for &'a Vector {
    type Item = &'a Node;
    type IntoIter = VectorIntoIterator<'a>;
    fn into_iter(self) -> VectorIntoIterator<'a> {
        VectorIntoIterator {
            vector: &self,
            index: 0
        }
    }
}

impl ops::Index<usize> for Vector {
    type Output = Node;

    fn index<'a>(&'a self, index: usize) -> &'a Node {
        &self.vector[index]
    }
}

impl ops::Index<ops::RangeFrom<usize>> for Vector {
    type Output = [Node];

    fn index(&self, index: ops::RangeFrom<usize>) -> &[Node] {
        &(**self)[index]
    }
}

impl ops::Deref for Vector {
    type Target = [Node];

    fn deref(&self) -> &[Node] {
        &self.vector
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_vec(&self.vector[..]))
    }
}
