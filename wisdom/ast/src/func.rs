use common::Position;
use tokenizer::{FromTokens, Token, TokenStream};

use crate::{Ident, Stmt, Typ};
use crate::error::ErrorKind::ExpectedIdent;
use crate::error::ParserError;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Function {
    pub ident: Ident,
    pub args: Vec<ArgSpec>,
    pub ret_typ: Option<Typ>,
    pub block: Block,
    pub position: Position,
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ArgSpec {
    pub name: Ident,
    pub typ: Option<Typ>,
    pub position: Position,
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub position: Position,
}

impl FromTokens for Function {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        use tokenizer::TokenKind::*;
        let mut args: Vec<ArgSpec> = Vec::new();
        // fn_tok is used purely for its position in the source. it's used for the overall function location
        let fn_tok = tokens.expect_ident("fn").ok_or(ParserError::new(ExpectedIdent("fn"), tokens.position()))?;
        let name = tokens.expect(Identifier).ok_or(ParserError::new(Identifier, tokens.position()))?;
        tokens.expect(LeftParen).ok_or(ParserError::new(RightParen, tokens.position()))?;
        while let None = tokens.expect(RightParen) {
            args.push(ArgSpec::from_tokens(tokens)?);
            if let Some(Token { kind: Comma, .. }) = tokens.peek() {
                tokens.consume();
            }
        }
        // TODO: add return types
        let block = Block::from_tokens(tokens)?;
        Ok(Self {
            ident: Ident {
                name: name.literal.clone(),
                position: name.position.clone(),
            },
            args: args.clone(),
            ret_typ: None,
            block,
            position: fn_tok.position.clone(),
        })
    }
}

impl FromTokens for ArgSpec {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        use tokenizer::TokenKind::*;

        let name = tokens.expect(Identifier).ok_or(
            ParserError::new(Identifier, tokens.position())
        )?;

        let typ = if let Some(Token { kind: Colon, .. }) = tokens.peek() {
            tokens.consume();
            let typ = tokens.expect(Identifier).ok_or(
                ParserError::new(Identifier, tokens.position())
            )?;
            Some(Typ {
                ident: Ident {
                    name: typ.literal.clone(),
                    position: typ.position.clone(),
                }
            })
        } else {
            None
        };

        Ok(Self {
            name: Ident {
                name: name.literal.clone(),
                position: name.position.clone(),
            },
            typ,
            position: name.position.clone(),
        })
    }
}

impl FromTokens for Block {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        use tokenizer::TokenKind::*;
        let start = tokens.expect(LeftBrace).ok_or(
            ParserError::new(LeftBrace, tokens.position())
        )?;

        let mut stmts = Vec::new();
        while let None = tokens.expect(RightBrace) {
            stmts.push(Stmt::from_tokens(tokens)?);
        }

        Ok(Self {
            stmts,
            position: start.position.clone(),
        })
    }
}