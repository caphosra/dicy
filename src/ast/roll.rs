use std::fmt::{Display, Formatter};

use rand::{thread_rng, Rng};

use crate::ast::AstNode;

///
/// A node in the AST that represents a roll.
///
pub struct RollNode {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
    pub result: Option<Vec<i32>>,
}

impl Display for RollNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sum: i32 = self.result.as_ref().unwrap().iter().sum();
        let comma_separated = self
            .result
            .as_ref()
            .unwrap()
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{} [{}]", sum, comma_separated)
    }
}

impl AstNode for RollNode {
    fn visit(&mut self) -> i32 {
        self.evaluate().iter().sum()
    }

    fn clear(&mut self) {
        self.result = None;
        self.left.clear();
        self.right.clear();
    }
}

impl RollNode {
    ///
    /// Initializes a new RollNode with the given left and right children.
    ///
    pub fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Self {
        Self {
            left,
            right,
            result: None,
        }
    }

    ///
    /// Evaluates the roll and returns the result.
    /// It also saves the result to the instance.
    ///
    pub fn evaluate(&mut self) -> &Vec<i32> {
        if self.result.is_some() {
            self.result.as_ref().unwrap()
        } else {
            let left = self.left.visit();
            let right = self.right.visit();

            let mut rng = thread_rng();
            let mut result = Vec::new();
            for _ in 0..left {
                result.push(rng.gen_range(1..=right));
            }
            self.result = Some(result);

            self.result.as_ref().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::constant::ConstantNode;

    #[test]
    fn test_fmt() {
        let mut node = RollNode::new(
            Box::new(ConstantNode::new(3)),
            Box::new(ConstantNode::new(1)),
        );
        node.evaluate();

        assert_eq!(format!("{}", node), "3 [1, 1, 1]");
    }

    #[test]
    fn test_visit() {
        let mut node = RollNode::new(
            Box::new(ConstantNode::new(3)),
            Box::new(ConstantNode::new(1)),
        );

        let result = node.visit();
        assert_eq!(result, 3);
        assert_eq!(node.visit(), result);
    }
}
