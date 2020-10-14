use crate::expr::Expr;
use crate::number::Number;
use tokenizer::{FromTokens, TokenStream};
use tokenizer::TokenKind;
use crate::error::{Error, ErrorKind};
use crate::error::ErrorKind::*;

#[derive(PartialOrd, PartialEq, Debug)]
pub struct Binding {
    name: String,
    value: Expr,
}

impl FromTokens for Binding {
    type Error = Error;

    fn from_tokens(tokens: &mut TokenStream) -> Result<Self, Self::Error> {
        let let_ident = tokens.expect(TokenKind::Identifier).ok_or(Error::from(InvalidToken))?;
        if let_ident.literal != "let" {
            return Err(InvalidToken.into());
        }

        let name_ident = tokens.expect_ignore_ws(TokenKind::Identifier).ok_or(Error::from(InvalidToken))?;
        tokens.expect_ignore_ws(TokenKind::Equals).ok_or(Error::from(InvalidToken))?;

        tokens.skip_whitespace();

        let expr = Expr::from_tokens(tokens)?;

        Ok(Self {
            name: name_ident.literal.clone(),
            value: expr,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_simple_binding() {
        let mut tokens = TokenStream::new("let foo = 1;");
        let bind = Binding::from_tokens(&mut tokens).unwrap();
        let expected = Binding {
            name: "foo".to_string(),
            value: Expr::Leaf(Value::Int(1)),
        };

        assert_eq!(bind, expected);
    }
}