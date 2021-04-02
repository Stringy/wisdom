extern crate serde;


mod position;

pub use position::*;

pub trait Describable {
    fn description(&self) -> String;
}

///
/// WisdomError is a trait for defining Error types throughout
/// Wisdom, its library, and interpreters / the REPL.
///
pub trait WisdomError: Describable + Sized {
    ///
    /// Return the position in the source code that this error occurred
    ///
    fn position(&self) -> Position;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
