use common::{Span, Position};
use crate::{Expr, Function, Ident, NodeId};
use tokenizer::{FromTokens, TokenStream};
use crate::error::ParserError;
use crate::error::ErrorKind::UnexpectedEOL;

#[derive(Debug, Clone)]
pub struct Stmt {
    pub position: Position,
    pub kind: StmtKind,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Expr(Expr),
    Fn(Function),
}

impl FromTokens for Stmt {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        if let Some(tok) = &tokens.peek() {
            use tokenizer::TokenKind::*;
            let stmt_kind = match tok.kind {
                Identifier => {
                    match tok.literal.as_str() {
                        "fn" => StmtKind::Fn(Function::from_tokens(tokens)?),
                        _ => StmtKind::Expr(Expr::from_tokens(tokens)?),
                    }
                }
                _ => StmtKind::Expr(Expr::from_tokens(tokens)?),
            };
            tokens.expect(SemiColon).ok_or(
                ParserError::new(SemiColon, tokens.position())
            );
            Ok(Stmt {
                position: tok.position.clone(),
                kind: stmt_kind,
            })
        } else {
            Err(ParserError::new(UnexpectedEOL, tokens.position()))
        }
    }
}

