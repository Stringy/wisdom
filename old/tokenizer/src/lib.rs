use std::io;
use crate::token::{Token, TokenKind};
use std::io::BufReader;
use std::str::Utf8Error;

extern crate utf8_chars;

use utf8_chars::BufReadCharsExt;
use crate::position::Position;
use std::cell::Cell;
use crate::cursor::Cursor;

pub mod token;
pub mod position;
mod cursor;

pub fn tokenize(mut input: &str) -> impl Iterator<Item=Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            Some(cursor.next_token())
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
