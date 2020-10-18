use crate::expr::Expr;
use crate::value::Value;
use crate::block::Block;
use tokenizer::{FromTokens, TokenStream, Token, TokenKind};


pub enum Stmt {
    Return(Expr),
    For(Value, Expr, Block),
    While(Expr, Block),
    Binding(Value, Expr),
    Assignment(Value, Expr),
}

impl FromTokens for Stmt {
    type Error = ();

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        use tokenizer::TokenKind::*;
        let first = tokens.first().ok_or(())?;
        match first.kind {
            Identifier => {
                match first.literal.as_str() {
                    "return" => Self::return_from_tokens(tokens),
                    "for" => Self::for_from_tokens(tokens),
                    "while" => Self::while_from_tokens(tokens),
                    "let" => Self::binding_from_tokens(tokens),
                    _ => {
                        if let Some(Token { kind: Eq, .. }) = tokens.second() {
                            // looks like an assignment
                            Self::assignment_from_tokens(tokens)
                        } else {
                            Err(())
                        }
                    }
                }
            }
            _ => Err(())
        }
    }
}

impl Stmt {
    fn return_from_tokens(tokens: &TokenStream) -> Result<Self, ()> {
        tokens.consume(); // consume return
        Ok(Self::Return(Expr::from_tokens(tokens).map_err(|_| ())?))
    }

    fn for_from_tokens(tokens: &TokenStream) -> Result<Self, ()> {
        tokens.consume(); // consume for
        if let Some(Token { kind: TokenKind::Identifier, literal, .. }) = tokens.first() {
            // the name of the token
            let variable = Value::Named(literal.to_owned());

            let maybe_in = tokens.expect(TokenKind::Identifier).ok_or(())?;
            if maybe_in.literal != "in" {
                return Err(());
            }

            let expr = Expr::from_tokens(tokens).map_err(|_| ())?;

            tokens.expect(TokenKind::LeftBrace).ok_or(())?;

            let block = Block::from_tokens(tokens).map_err(|_| ())?;

            tokens.expect(TokenKind::RightBrace).ok_or(())?;

            Ok(Self::For(variable, expr, block))
        } else {
            Err(())
        }
    }

    fn while_from_tokens(tokens: &TokenStream) -> Result<Self, ()> {
        tokens.consume();
        let expr = Expr::from_tokens(tokens).map_err(|_| ())?;
        tokens.expect(TokenKind::LeftBrace).ok_or(())?;
        let block = Block::from_tokens(tokens).map_err(|_| ())?;
        tokens.expect(TokenKind::RightBrace).ok_or(())?;
        Ok(Self::While(expr, block))
    }

    fn binding_from_tokens(tokens: &TokenStream) -> Result<Self, ()> {
        tokens.consume(); // consume let
        let name = tokens.expect(TokenKind::Identifier).ok_or(())?;
        tokens.expect(TokenKind::Eq).ok_or(())?;
        let expr = Expr::from_tokens(tokens).map_err(|_| ())?;
        Ok(Self::Binding(Value::Named(name.literal.to_owned()), expr))
    }

    fn assignment_from_tokens(tokens: &TokenStream) -> Result<Self, ()> {
        let name = tokens.expect(TokenKind::Identifier).ok_or(())?;
        tokens.expect(TokenKind::Eq).ok_or(())?;
        let expr = Expr::from_tokens(tokens).map_err(|_| ())?;
        Ok(Self::Assignment(Value::Named(name.literal.to_owned()), expr))
    }
}