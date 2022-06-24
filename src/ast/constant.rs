use std::fmt::{Display, Formatter};

use crate::ast::AstNode;

///
/// A node in the AST that represents a constant.
///
pub struct ConstantNode {
    pub value: i32,
}

impl Display for ConstantNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AstNode for ConstantNode {
    fn visit(&mut self) -> i32 {
        self.value
    }

    fn clear(&mut self) {}
}

impl ConstantNode {
    ///
    /// Initializes a new ConstantNode with the given value.
    ///
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let mut node = ConstantNode::new(10);
        assert_eq!(node.visit(), 10);
    }
}
