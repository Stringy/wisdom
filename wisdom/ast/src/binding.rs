use crate::expr::Expr;
use crate::number::Number;
use tokenizer::{FromTokens, TokenStream};
use tokenizer::TokenKind;
use crate::error::{Error, ErrorKind};
use crate::error::ErrorKind::*;

pub struct Binding {
    name: String,
    value: Expr,
}

impl FromTokens for Binding {
    type Error = Error;

    fn from_tokens(iter: &mut TokenStream) -> Result<Self, Self::Error> {
        let let_ident = iter.expect(TokenKind::Identifier).ok_or(Error::from(InvalidToken))?;
        if let_ident.literal != "let" {
            return Err(InvalidToken.into());
        }
        iter.skip_whitespace();

        let name_ident = iter.expect(TokenKind::Identifier).ok_or(Error::from(InvalidToken))?;

        iter.skip_whitespace();

        let _ = iter.expect(TokenKind::Equals).ok_or(Error::from(InvalidToken))?;

        iter.skip_whitespace();

        let expr = Expr::from_tokens(iter)?;

        Ok(Self {
            name: name_ident.literal.clone(),
            value: expr,
        })
    }
}
