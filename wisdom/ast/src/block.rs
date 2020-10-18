use tokenizer::{FromTokens, TokenStream, TokenKind};
use crate::stmt::Stmt;

pub struct Block {
    stmts: Vec<Stmt>,
}

impl FromTokens for Block {
    type Error = ();

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        let mut stmts = Vec::new();
        tokens.expect(TokenKind::LeftBrace).ok_or(())?;

        while tokens.expect(TokenKind::RightBrace).is_none() {
            stmts.push(Stmt::from_tokens(tokens)?);
        }

        Ok(Self {
            stmts
        })
    }
}
