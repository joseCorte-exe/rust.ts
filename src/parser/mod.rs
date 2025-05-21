use crate::ast::{Expression, Statement, Type};
use crate::lexer::Token;
use logos::Span;

pub struct Parser {
    tokens: Vec<(Token, Span)>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }
        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek() {
            Token::ConsoleLog => self.parse_console_log(),
            Token::Let | Token::Const => self.parse_variable_declaration(),
            Token::If => self.parse_if_statement(),
            _ => None,
        }
    }

    fn parse_console_log(&mut self) -> Option<Statement> {
        self.advance(); // Consume 'console.log'
        self.expect(Token::OpenParen)?;
        let expr = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::Semicolon)?;
        Some(Statement::ConsoleLog(expr))
    }

    fn parse_variable_declaration(&mut self) -> Option<Statement> {
        let is_const = matches!(self.advance(), Token::Const);
        let name = if let Token::Identifier(name) = self.advance().clone() {
            name
        } else {
            return None;
        };

        self.expect(Token::Colon)?;
        let type_annotation = self.parse_type()?;

        let value = if self.match_token(Token::Semicolon) {
            None
        } else {
            let expr = self.parse_expression()?;
            self.expect(Token::Semicolon)?;
            Some(expr)
        };

        Some(Statement::VariableDeclaration {
            name,
            type_annotation,
            value,
        })
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        self.advance(); // Consume 'if'
        self.expect(Token::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;

        let mut then_branch = Vec::new();
        while !self.check(Token::CloseBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                then_branch.push(stmt);
            }
        }
        self.expect(Token::CloseBrace)?;

        let else_branch = if self.match_token(Token::Else) {
            self.expect(Token::OpenBrace)?;
            let mut else_statements = Vec::new();
            while !self.check(Token::CloseBrace) && !self.is_at_end() {
                if let Some(stmt) = self.parse_statement() {
                    else_statements.push(stmt);
                }
            }
            self.expect(Token::CloseBrace)?;
            Some(else_statements)
        } else {
            None
        };

        Some(Statement::IfStatement {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        match self.advance() {
            Token::StringLiteral(s) => Some(Expression::StringLiteral(s)),
            Token::Number(n) => Some(Expression::NumberLiteral(n)),
            Token::Identifier(name) => Some(Expression::Identifier(name)),
            _ => None,
        }
    }

    fn parse_type(&mut self) -> Option<Type> {
        match self.advance() {
            Token::Identifier(name) => match name.as_str() {
                "string" => Some(Type::String),
                "number" => Some(Type::Number),
                "boolean" => Some(Type::Boolean),
                _ => None,
            },
            _ => None,
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].0.clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].0.clone()
    }

    fn check(&self, token: Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek() == token
        }
    }

    fn match_token(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, token: Token) -> Option<()> {
        if self.check(token) {
            self.advance();
            Some(())
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
