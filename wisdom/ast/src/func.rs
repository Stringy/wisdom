use crate::stmt::Stmt;
use tokenizer::{FromTokens, TokenStream, Token};
use tokenizer::TokenKind::*;
use crate::error::ParserError;
use crate::error::ErrorKind::*;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<ArgSpec>,
    pub body: Vec<Stmt>,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.args.iter().map(|a| a.name.clone()).collect::<Vec<String>>().join(", "))
    }
}

impl FromTokens for Function {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        // consume fn
        tokens.consume();

        if let Some(Token { literal, .. }) = tokens.expect(Identifier) {
            tokens.expect(LeftParen).ok_or(
                ParserError::new(ExpectedTokens(&[LeftParen]), tokens.position())
            )?;
            let mut args = Vec::new();
            loop {
                if let Some(Token { kind: RightParen, .. }) = tokens.peek() {
                    break;
                }

                args.push(ArgSpec::from_tokens(tokens)?);
                if let Some(Token { kind: Comma, .. }) = tokens.peek() {
                    tokens.consume();
                    continue;
                }
                break;
            }
            tokens.expect(RightParen).ok_or(
                ParserError::new(ExpectedTokens(&[RightParen]), tokens.position())
            )?;
            tokens.expect(LeftBrace).ok_or(
                ParserError::new(ExpectedTokens(&[LeftBrace]), tokens.position())
            )?;

            let mut stmts = Vec::new();
            loop {
                let token = tokens.peek();
                match token {
                    Some(Token { kind: RightBrace, .. }) => {
                        tokens.consume();
                        break;
                    },
                    _ => {
                        stmts.push(Stmt::from_tokens(tokens)?)
                    }
                }
            }

            Ok(Self {
                name: literal.to_owned(),
                args,
                body: stmts,
            })
        } else {
            Err(ParserError::new(ExpectedTokens(&[Identifier]), tokens.position()))
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ArgSpec {
    pub name: String
}

impl FromTokens for ArgSpec {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        let ident = tokens.expect(Identifier)
            .ok_or(
                ParserError::new(ExpectedTokens(&[Identifier]), tokens.position())
            )?;
        Ok(Self {
            name: ident.literal.to_owned()
        })
    }
}