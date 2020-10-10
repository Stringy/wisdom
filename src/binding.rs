use crate::expr::Expr;
use crate::traits::Executable;
use crate::number::Number;
use wisdom_tokenizer::cursor::{FromTokens, Tokens};
use wisdom_tokenizer::tokens::TokenKind;

pub struct Binding {
    name: String,
    value: Expr,
}

impl FromTokens for Binding {
    type Error = ();

    fn from_tokens(iter: &mut Tokens) -> Result<Self, Self::Error> {
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

impl Executable<Number> for Binding {
    type Err = ();

    fn execute(&self) -> Result<Number, Self::Err> {
        Ok(self.value.execute())
    }
}