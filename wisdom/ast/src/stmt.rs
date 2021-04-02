use common::{Position};
use tokenizer::{FromTokens, TokenStream};

use crate::{Expr, Function};
use crate::error::ErrorKind::UnexpectedEOL;
use crate::error::ParserError;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Stmt {
    pub position: Position,
    pub kind: StmtKind,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
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
            // TODO: look into semi-colon processing - when do we need them?
            let _ = tokens.expect(SemiColon);
            Ok(Stmt {
                position: tok.position.clone(),
                kind: stmt_kind,
            })
        } else {
            Err(ParserError::new(UnexpectedEOL, tokens.position()))
        }
    }
}

