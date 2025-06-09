mod tokens;
pub use tokens::Token;

use logos::{Logos, Span};

pub struct Lexer {
    tokens: Vec<(Token, Span)>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let lexer = Token::lexer(source);
        let tokens: Vec<(Token, Span)> = lexer
            .spanned()
            .filter_map(|(token, span)| match token {
                Ok(token) => Some((token, span)),
                _ => None,
            })
            .collect();

        Lexer { tokens }
    }

    pub fn get_tokens(self) -> Vec<(Token, Span)> {
        self.tokens
    }
}
