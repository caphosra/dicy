use std::default::Default;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::error::VerboseError;
use nom::multi::separated_list1;
use nom::sequence::tuple;

use crate::ast::constant::ConstantNode;
use crate::ast::mul::MulNode;
use crate::ast::roll::RollNode;
use crate::ast::AstNode;

///
/// A parser for a language that represents dice rolls.
///
#[derive(Default)]
pub struct Parser;

impl Parser {
    ///
    /// Parses an expression into the node of AST.
    ///
    pub fn parse<'l>(
        &'l self,
        text: &'l str,
    ) -> nom::IResult<&'l str, Box<dyn AstNode>, VerboseError<&'l str>> {
        let (text, node) = self.parse_roll(text)?;
        Ok((text, node))
    }

    ///
    /// Parses a term into the node of AST.
    ///
    pub fn parse_term<'l>(
        &'l self,
        text: &'l str,
    ) -> nom::IResult<&'l str, Box<dyn AstNode>, VerboseError<&'l str>> {
        let roll_parser = |t| self.parse_roll(t);

        let (text, node) = separated_list1(tag("*"), roll_parser)(text)?;
        Ok((text, Box::new(MulNode::new(node))))
    }

    ///
    /// Parses a roll into the node of AST.
    ///
    pub fn parse_roll<'l>(
        &'l self,
        text: &'l str,
    ) -> nom::IResult<&str, Box<dyn AstNode>, VerboseError<&'l str>> {
        let factor_parser = |t| self.parse_factor(t);

        let (text, node) = alt((
            map(
                tuple((factor_parser, alt((tag("d"), tag("D"))), factor_parser)),
                |(left, _, right)| Box::new(RollNode::new(left, right)) as Box<dyn AstNode>,
            ),
            factor_parser,
        ))(text)?;

        Ok((text, node))
    }

    ///
    /// Parses a factor into the node of AST.
    ///
    pub fn parse_factor<'l>(
        &'l self,
        text: &'l str,
    ) -> nom::IResult<&'l str, Box<dyn AstNode>, VerboseError<&'l str>> {
        let (text, node) = map(digit1, |digits: &str| {
            Box::new(ConstantNode::new(digits.parse().unwrap()))
        })(text)?;

        Ok((text, node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_term() {
        let parser = Parser::default();
        let text = "3d1*19";
        let (text, mut node) = parser.parse_term(text).unwrap();
        assert_eq!(text, "");
        assert_eq!(node.visit(), 57);
    }

    #[test]
    fn test_parse_roll() {
        let parser = Parser::default();
        let text = "3d1";
        let (text, mut node) = parser.parse_roll(text).unwrap();
        assert_eq!(text, "");
        assert_eq!(node.visit(), 3);
    }

    #[test]
    fn test_parse_factor() {
        let parser = Parser::default();
        let text = "3";
        let (text, mut node) = parser.parse_factor(text).unwrap();
        assert_eq!(text, "");
        assert_eq!(node.visit(), 3);
    }
}
