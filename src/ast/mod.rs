use std::fmt::Display;

///
/// A node in the AST. All nodes must implement this trait.
///
pub trait AstNode: Display {
    ///
    /// Visits its children and itself to evaluate the value.
    ///
    fn visit(&mut self) -> i32;

    ///
    /// Clears the result of its children and itself.
    ///
    fn clear(&mut self);
}

pub mod constant;
pub mod mul;
pub mod roll;
