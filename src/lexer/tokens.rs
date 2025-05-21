use logos::{Logos, Span};
use std::num::ParseIntError;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("console.log")]
    ConsoleLog,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("let")]
    Let,

    #[token("const")]
    Const,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.slice().to_string())]
    StringLiteral(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i32>().ok())]
    Number(i32),

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}
