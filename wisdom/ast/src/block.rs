use tokenizer::{FromTokens, TokenStream, TokenKind};
use crate::stmt::Stmt;
use crate::error::ParserError;

#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl FromTokens for Block {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        let mut stmts = Vec::new();
        tokens.expect(TokenKind::LeftBrace).ok_or(
            ParserError::new(TokenKind::LeftBrace, tokens.position())
        )?;

        while tokens.expect(TokenKind::RightBrace).is_none() {
            stmts.push(Stmt::from_tokens(tokens)?);
        }

        Ok(Self {
            stmts
        })
    }
}
