use crate::expr::Expr;
use crate::value::Value;
use crate::block::Block;
use tokenizer::{FromTokens, TokenStream, Token, TokenKind};
use crate::error::ParserError;
use crate::error::ErrorKind::{UnexpectedEOL, InvalidToken, ExpectedTokens, ExpectedIdent, ExpectSemiColon};


#[derive(Clone, Debug)]
pub enum Stmt {
    Return(Expr),
    For(Value, Expr, Block),
    While(Expr, Block),
    Binding(Value, Expr),
    Assignment(Value, Expr),
    Call(Value, Vec<Expr>),
}

impl FromTokens for Stmt {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        use tokenizer::TokenKind::*;
        let first = tokens.first().ok_or(ParserError::new(UnexpectedEOL, None))?;
        match first.kind {
            Identifier => {
                match first.literal.as_str() {
                    "return" => Self::return_from_tokens(tokens),
                    "for" => Self::for_from_tokens(tokens),
                    "while" => Self::while_from_tokens(tokens),
                    "let" => Self::binding_from_tokens(tokens),
                    _ => {
                        match tokens.second() {
                            Some(Token { kind: Eq, .. }) => Self::assignment_from_tokens(tokens),
                            Some(Token { kind: LeftParen, .. }) => Self::function_call_from_tokens(tokens),
                            None => Err(ParserError::new(UnexpectedEOL, Some(first.position))),
                            Some(tok) => Err(ParserError::new(InvalidToken(tok.kind), Some(tok.position)))
                        }
                    }
                }
            }
            _ => Err(ParserError::new(UnexpectedEOL, None))
        }
    }
}

impl Stmt {
    fn return_from_tokens(tokens: &TokenStream) -> Result<Self, ParserError> {
        tokens.consume(); // consume return
        Ok(Self::Return(Expr::from_tokens(tokens)?))
    }

    fn for_from_tokens(tokens: &TokenStream) -> Result<Self, ParserError> {
        tokens.consume(); // consume for
        if let Some(Token { kind: TokenKind::Identifier, literal, .. }) = tokens.first() {
            // the name of the token
            let variable = Value::Named(literal.to_owned());

            let maybe_in = tokens.expect(TokenKind::Identifier).ok_or(
                ParserError::new(ExpectedTokens(&[TokenKind::Identifier]), tokens.position())
            )?;

            if maybe_in.literal != "in" {
                return Err(ParserError::new(ExpectedIdent("in"), Some(maybe_in.position)));
            }

            let expr = Expr::from_tokens(tokens)?;
            let block = Block::from_tokens(tokens)?;
            Ok(Self::For(variable, expr, block))
        } else {
            Err(ParserError::new(UnexpectedEOL, None))
        }
    }

    fn while_from_tokens(tokens: &TokenStream) -> Result<Self, ParserError> {
        tokens.consume();
        let expr = Expr::from_tokens(tokens)?;
        let block = Block::from_tokens(tokens)?;
        Ok(Self::While(expr, block))
    }

    fn binding_from_tokens(tokens: &TokenStream) -> Result<Self, ParserError> {
        tokens.consume(); // consume let
        let name = tokens.expect(TokenKind::Identifier).ok_or(
            ParserError::new(ExpectedTokens(&[TokenKind::Identifier]), tokens.position())
        )?;
        tokens.expect(TokenKind::Eq).ok_or(
            ParserError::new(ExpectedTokens(&[TokenKind::Eq]), tokens.position())
        )?;
        let expr = Expr::from_tokens(tokens)?;
        Ok(Self::Binding(Value::Named(name.literal.to_owned()), expr))
    }

    fn assignment_from_tokens(tokens: &TokenStream) -> Result<Self, ParserError> {
        let name = tokens.expect(TokenKind::Identifier).ok_or(
            ParserError::new(ExpectedTokens(&[TokenKind::Identifier]), tokens.position())
        )?;

        tokens.expect(TokenKind::Eq).ok_or(
            ParserError::new(ExpectedTokens(&[TokenKind::Eq]), tokens.position())
        )?;

        let expr = Expr::from_tokens(tokens)?;
        tokens.expect(TokenKind::SemiColon).ok_or(
            ParserError::new(ExpectSemiColon, tokens.position())
        )?;
        Ok(Self::Assignment(Value::Named(name.literal.to_owned()), expr))
    }

    fn function_call_from_tokens(tokens: &TokenStream) -> Result<Self, ParserError> {
        let name_tok = tokens.consume().ok_or(
            ParserError::new(UnexpectedEOL, None)
        )?;

        tokens.expect(TokenKind::LeftParen).ok_or(
            ParserError::new(ExpectedTokens(&[TokenKind::LeftParen]), tokens.position())
        )?;

        let mut exprs = Vec::new();
        loop {
            if let Some(Token { kind: TokenKind::RightParen, .. }) = tokens.first() {
                // we've reached the end
                break;
            }

            exprs.push(Expr::from_tokens(tokens)?);
            match tokens.expect(TokenKind::Comma) {
                Some(_) => continue,
                None => {
                    break;
                }
            }
        }

        tokens.expect(TokenKind::RightParen).ok_or(
            ParserError::new(ExpectedTokens(&[TokenKind::RightParen]), tokens.position())
        )?;

        tokens.expect(TokenKind::SemiColon).ok_or(
            ParserError::new(ExpectSemiColon, tokens.position())
        )?;

        Ok(Self::Call(Value::Named(name_tok.literal.to_owned()), exprs))
    }
}