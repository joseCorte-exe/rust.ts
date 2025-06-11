use crate::ast::{BinaryOperator, Expression, Statement, Type};
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
            Token::While => self.parse_while_statement(),
            Token::Identifier(_) => {
                if self.lookahead_is(Token::Equal) {
                    self.parse_assignment()
                } else {
                    // Se não for atribuição, evite loop consumindo o token (ex: expressão ou erro)
                    self.advance();
                    None
                }
            }
            _ => {
                // Avança para evitar loop infinito
                self.advance();
                None
            }
        }
    }


    fn parse_assignment_expression(&mut self) -> Option<Expression> {
        let expr = self.parse_binary_expression()?;

        if self.lookahead_is(Token::Equal) {
            if let Expression::Identifier(name) = expr {
                self.advance(); // consome '='
                let value = self.parse_assignment_expression()?; // Recursivo p/ associatividade direita
                return Some(Expression::Assignment {
                    name,
                    value: Box::new(value),
                });
            } else {
                eprintln!("Erro: lado esquerdo da atribuição não é um identificador.");
                return None;
            }
        }

        Some(expr)
    }

    fn parse_console_log(&mut self) -> Option<Statement> {
        self.advance(); // Consume 'console.log'
        self.expect(Token::OpenParen)?;

        let mut args = Vec::new();

        while self.peek() != Token::CloseParen {
            let expr = self.parse_expression()?;
            args.push(expr);

        if self.peek() == Token::Comma {
            self.advance();
        } else {
            break;
        }
    }

    self.expect(Token::CloseParen)?;
    self.expect(Token::Semicolon)?;

    Some(Statement::ConsoleLog(args))
}


    fn parse_variable_declaration(&mut self) -> Option<Statement> {
        let _is_const = matches!(self.advance(), Token::Const); // ou Let
        let name = if let Token::Identifier(name) = self.advance() {
            name
        } else {
            return None;
        };

        self.expect(Token::Colon)?;
        let type_annotation = self.parse_type()?;

        let value = if self.match_token(Token::Equal) {
            let expr = self.parse_expression()?;
            self.expect(Token::Semicolon)?;
            Some(expr)
        } else {
            self.expect(Token::Semicolon)?; // termina com ;
            None
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

    fn parse_while_statement(&mut self) -> Option<Statement> {
        self.advance(); // Consume 'while'
        self.expect(Token::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;

        let mut body = Vec::new();
        while !self.check(Token::CloseBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            }
        }
        self.expect(Token::CloseBrace)?;

        Some(Statement::WhileStatement { condition, body })
    }

    fn parse_assignment(&mut self) -> Option<Statement> {
        // Consome o identificador
        let name = if let Token::Identifier(name) = self.advance() {
            name
        } else {
            return None;
        };

        // Consome o '='
        self.expect(Token::Equal)?;

        // Pega a expressão depois do '='
        let value = self.parse_expression()?;

        // Consome o ';'
        self.expect(Token::Semicolon)?;

        Some(Statement::Assignment { name, value })
    }


    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_assignment_expression()
    }


    fn parse_binary_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_primary()?;

        while let Some(op) = self.parse_binary_operator() {
            let right = self.parse_primary()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_primary(&mut self) -> Option<Expression> {
    match self.advance() {
        Token::StringLiteral(s) => Some(Expression::StringLiteral(s)),
        Token::Number(n) => Some(Expression::NumberLiteral(n)),
        Token::Identifier(name) => Some(Expression::Identifier(name)),
        Token::OpenBracket => {
            let mut elements = Vec::new();
            while !self.check(Token::CloseBracket) && !self.is_at_end() {
                if let Some(expr) = self.parse_expression() {
                    elements.push(expr);
                    if !self.match_token(Token::Comma) {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.expect(Token::CloseBracket)?;
            Some(Expression::ArrayLiteral(elements))
        }
        _ => None,
    }
}


    fn parse_binary_operator(&mut self) -> Option<BinaryOperator> {
        match self.peek() {
            Token::Plus => {
                self.advance();
                Some(BinaryOperator::Add)
            }
            Token::Minus => {
                self.advance();
                Some(BinaryOperator::Subtract)
            }
            Token::Star => {
                self.advance();
                Some(BinaryOperator::Multiply)
            }
            Token::Slash => {
                self.advance();
                Some(BinaryOperator::Divide)
            }
            Token::LessThan => {
                self.advance();
                Some(BinaryOperator::LessThan)
            }
            Token::GreaterThan => {
                self.advance();
                Some(BinaryOperator::GreaterThan)
            }
            _ => None,
        }
    }

    fn parse_type(&mut self) -> Option<Type> {
    match self.advance() {
        Token::Identifier(name) => {
            let base_type = match name.as_str() {
                "string" => Type::String,
                "number" => Type::Number,
                "boolean" => Type::Boolean,
                _ => return None,
            };

            if self.check(Token::OpenBracket) {
                self.advance();
                self.expect(Token::CloseBracket)?;
                Some(Type::Array(Box::new(base_type)))
            } else {
                Some(base_type)
            }
        }
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
        self.tokens
            .get(self.current)
            .map(|t| t.0.clone())
            .unwrap_or(Token::Semicolon)
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current.saturating_sub(1))
            .map(|t| t.0.clone())
            .unwrap_or(Token::Semicolon)
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

    fn lookahead_is(&self, expected: Token) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.current + 1].0)
            == std::mem::discriminant(&expected)
    }


}
