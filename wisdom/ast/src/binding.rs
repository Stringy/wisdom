use crate::expr::Expr;
use crate::number::Number;
use tokenizer::{FromTokens, TokenStream};
use tokenizer::TokenKind;

pub struct Binding {
    name: String,
    value: Expr,
}

impl FromTokens for Binding {
    type Error = ();

    fn from_tokens(iter: &mut TokenStream) -> Result<Self, Self::Error> {
        let let_ident = iter.expect(TokenKind::Identifier).ok_or(())?;
        if let_ident.literal != "let" {
            return Err(());
        }
        iter.skip_whitespace();

        let name_ident = iter.expect(TokenKind::Identifier).ok_or(())?;

        iter.skip_whitespace();

        let _ = iter.expect(TokenKind::Equals).ok_or(())?;

        iter.skip_whitespace();

        let expr = Expr::from_tokens(iter)?;

        Ok(Self {
            name: name_ident.literal.clone(),
            value: expr,
        })
    }
}
