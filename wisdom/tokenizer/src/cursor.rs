use std::str::Chars;
use std::iter::Peekable;

use crate::token::{Token, TokenKind};
use std::collections::VecDeque;
use crate::Position;

///
/// A Cursor is responsible for breaking up an input
/// string into Tokens. It emits a single token at a time
/// and can be used to construct iterators over the input.
///
pub struct Cursor<'a> {
    /// Total size of the input
    size: usize,
    /// End of the previous token
    prev: usize,
    /// Current index into the chars list
    idx: usize,
    /// The input chars
    chars: Chars<'a>,
    /// All chars consumed from self.chars
    /// reset after each token consumed
    consumed: Vec<char>,
    /// Current position in the source code
    position: Position,
}

impl<'a> Cursor<'a> {
    ///
    /// Constructs a new Cursor from the input string.
    ///
    pub fn new(input: &'a str) -> Self {
        Self {
            size: input.len(),
            prev: 0,
            idx: 0,
            chars: input.chars(),
            consumed: Vec::new(),
            position: Default::default(),
        }
    }

    ///
    /// Get the next character from the Cursor, for lookahead
    ///
    pub fn first(&self) -> char {
        self.nth(0)
    }

    ///
    /// Get the second next character from the Cursor, for lookahead
    ///
    pub fn second(&self) -> char {
        self.nth(1)
    }

    ///
    ///
    ///
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    ///
    /// Consume and return the next character in the string.
    /// If there are no more characters, this function will return
    /// None
    ///
    pub fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.idx += 1;
            self.consumed.push(ch);
            self.position.column += 1;
            if ch == '\n' {
                self.position.line += 1;
                self.position.column = 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    ///
    /// Gets a copy of the remaining characters in this Cursor.
    ///
    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    ///
    /// Gets the nth item in the iterator, without consuming up to
    /// that point, as you'd normally expect. Instead it will return
    /// a copy of the nth character or a NUL byte if n is too large.
    ///
    /// TODO: consider refactoring this potentially enormous and frequent copy.
    ///
    fn nth(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or('\0')
    }
}

impl Cursor<'_> {
    ///
    /// Advances the Cursor until it has consumed a complete Token
    /// and returns.
    ///
    /// ```
    /// use tokenizer::Cursor;
    /// use tokenizer::TokenKind;
    ///
    /// let mut cursor = Cursor::new("123");
    /// let tok = cursor.next_token();
    /// assert_eq!(tok.kind, TokenKind::Number);
    /// assert_eq!(tok.literal.as_str(), "123");
    /// ```
    ///
    pub fn next_token(&mut self) -> Token {
        use crate::token::TokenKind::*;

        self.prev = self.idx;

        let saved_position = self.position.clone();

        let ch = self.next().unwrap_or('\0');
        let kind = match ch {
            ch if ch.is_whitespace() => {
                self.consume_until(|c| !c.is_whitespace());
                Whitespace
            }

            ch if ch.is_numeric() => {
                self.consume_until(|c| !c.is_numeric());
                Number
            }

            ch if self.is_ident_start(ch) => {
                self.consume_until(|c| !c.is_alphanumeric());
                Identifier
            }

            // Handle all the single-character tokens
            '+' => Add,
            '-' => Sub,
            '*' => Mul,
            '/' => Div,
            '=' => Equals,
            ';' => SemiColon,

            _ => panic!("invalid character: {:?}", ch)
        };

        let token = Token {
            kind,
            literal: self.consumed.iter().collect(),
            position: saved_position,
        };
        self.consumed.clear();
        token
    }

    ///
    /// Checks whether the given character is a valid start to an identifier
    /// i.e. is alphabetic or is an underscore.
    ///
    /// Numeric characters are invalid for starting an identifier but
    /// are allowed within the body of the identifier.
    ///
    fn is_ident_start(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    ///
    /// Consumes from the Cursor until the function provided returns true.
    /// This is useful for seeking forward to the end of a token.
    ///
    /// The Cursor will end up in a state where func(c) == true where
    /// c == self.first()
    ///
    fn consume_until<F: FnOnce(char) -> bool + Copy>(&mut self, func: F) {
        let mut c = self.first();

        loop {
            if func(c) {
                break;
            }

            // consume the character
            self.next();
            // peek at the the next one
            c = self.first();
        }
    }

    ///
    /// The opposite behaviour to consume_until, this function will consume until
    /// the given function returns false
    ///
    fn consume_while<F: FnOnce(char) -> bool + Copy>(&mut self, func: F) {
        // TODO: is there a better way of wrapping the func like this?
        self.consume_until(|c| !func(c))
    }

    ///
    /// Returns the number of characters consumed since we last emitted
    /// a Token.
    ///
    fn len_consumed(&self) -> usize {
        self.idx - self.prev
    }
}

///
/// Creates a Token iterator from the input string.
///
/// ```
/// use tokenizer::tokenize;
/// assert_eq!(3, tokenize("1+2").collect::<Vec<_>>().len())
/// ```
///
pub fn tokenize(input: &str) -> impl Iterator<Item=Token> + '_ {
    let mut c = Cursor::new(input);
    std::iter::from_fn(move || {
        if c.is_eof() {
            None
        } else {
            Some(c.next_token())
        }
    })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cursor_simple() {
        let tokens = tokenize("1 + 1").collect::<Vec<Token>>();
        let expected = vec![
            Token { kind: TokenKind::Number, literal: "1".to_string(), position: Position { line: 1, column: 1 } },
            Token { kind: TokenKind::Whitespace, literal: " ".to_string(), position: Position { line: 1, column: 2 } },
            Token { kind: TokenKind::Add, literal: "+".to_string(), position: Position { line: 1, column: 3 } },
            Token { kind: TokenKind::Whitespace, literal: " ".to_string(), position: Position { line: 1, column: 4 } },
            Token { kind: TokenKind::Number, literal: "1".to_string(), position: Position { line: 1, column: 5 } },
        ];

        assert_eq!(&tokens[..], &expected[..]);
    }

    #[test]
    fn test_cursor_ident() {
        let tokens: Vec<Token> = tokenize("identifier").collect();
        let expected = vec![
            Token { kind: TokenKind::Identifier, literal: "identifier".to_string(), position: Position { line: 1, column: 1 } }
        ];

        assert_eq!(&tokens[..], &expected[..]);
    }
}