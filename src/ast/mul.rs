use std::fmt::{Display, Formatter};

use crate::ast::AstNode;

///
/// A node in the AST that represents a multiplication.
///
pub struct MulNode {
    pub children: Vec<Box<dyn AstNode>>,
}

impl Display for MulNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let joined = self
            .children
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<String>>()
            .join(" * ");
        write!(f, "{}", joined)
    }
}

impl AstNode for MulNode {
    fn visit(&mut self) -> i32 {
        let mut result = 1;
        for child in &mut self.children {
            result *= child.visit();
        }
        result
    }

    fn clear(&mut self) {
        for child in &mut self.children {
            child.clear();
        }
    }
}

impl MulNode {
    ///
    /// Initializes a new MulNode with the given children.
    ///
    pub fn new(children: Vec<Box<dyn AstNode>>) -> Self {
        Self { children }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::constant::ConstantNode;
    use crate::ast::roll::RollNode;

    #[test]
    fn test_fmt() {
        let mut node = MulNode::new(vec![
            Box::new(RollNode::new(
                Box::new(ConstantNode::new(3)),
                Box::new(ConstantNode::new(1)),
            )),
            Box::new(ConstantNode::new(19)),
        ]);
        node.visit();

        assert_eq!(format!("{}", node), "3 [1, 1, 1] * 19");
    }

    #[test]
    fn test_visit() {
        let mut node = MulNode::new(vec![
            Box::new(ConstantNode::new(3)),
            Box::new(ConstantNode::new(19)),
        ]);
        assert_eq!(node.visit(), 57);
    }
}
