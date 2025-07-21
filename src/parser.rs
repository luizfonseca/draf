//! Parser for the strongly typed TypeScript compiler
//!
//! This module implements a recursive descent parser that converts tokens
//! into an Abstract Syntax Tree (AST) for the Draf language.

use crate::ast::*;
use crate::error::{DrafError, DrafResult};
use crate::lexer::{Token, TokenKind};
use std::iter::Peekable;
use std::slice::Iter;

/// Parser for converting tokens to AST
pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    current_token: Option<&'a Token>,
}

impl<'a> Parser<'a> {
    /// Create a new parser from tokens
    pub fn new(tokens: &'a [Token]) -> Self {
        let mut parser = Self {
            tokens: tokens.iter().peekable(),
            current_token: None,
        };
        parser.advance(); // Initialize with first token
        parser
    }

    /// Advance to the next token
    fn advance(&mut self) {
        self.current_token = self.tokens.next();
    }

    /// Peek at the next token without consuming it
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek().copied()
    }

    /// Check if current token matches expected kind
    fn check(&self, kind: &TokenKind) -> bool {
        match self.current_token {
            Some(token) => &token.kind == kind,
            None => false,
        }
    }

    /// Consume current token if it matches expected kind
    fn consume(&mut self, kind: TokenKind, message: &str) -> DrafResult<&'a Token> {
        if let Some(token) = self.current_token {
            if token.kind == kind {
                let result = token;
                self.advance();
                Ok(result)
            } else {
                Err(DrafError::parse_error(
                    token.line,
                    token.column,
                    format!("Expected {}, found {}: {}", kind, token.kind, message),
                ))
            }
        } else {
            Err(DrafError::parse_error(
                0,
                0,
                format!("Expected {}, found end of file: {}", kind, message),
            ))
        }
    }

    /// Parse a complete program
    pub fn parse_program(&mut self) -> DrafResult<Program> {
        let mut statements = Vec::new();
        let start_location = self.current_location();

        while !self.is_at_end() {
            if self.check(&TokenKind::Newline) {
                self.advance(); // Skip newlines
                continue;
            }
            statements.push(self.parse_statement()?);
        }

        Ok(Program::new(statements, start_location))
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> DrafResult<Statement> {
        match &self.current_token {
            Some(token) => match &token.kind {
                TokenKind::Let | TokenKind::Const | TokenKind::Var => {
                    self.parse_variable_declaration()
                }
                TokenKind::Function => self.parse_function_declaration(),
                TokenKind::Interface => self.parse_interface_declaration(),
                TokenKind::Type => self.parse_type_alias(),
                TokenKind::If => self.parse_if_statement(),
                TokenKind::While => self.parse_while_statement(),
                TokenKind::For => self.parse_for_statement(),
                TokenKind::Return => self.parse_return_statement(),
                TokenKind::LeftBrace => self.parse_block_statement(),
                TokenKind::Semicolon => {
                    let location = self.current_location();
                    self.advance();
                    Ok(Statement::Empty { location })
                }
                _ => self.parse_expression_statement(),
            },
            None => Err(DrafError::parse_error(
                0,
                0,
                "Unexpected end of file while parsing statement",
            )),
        }
    }

    /// Parse variable declaration: let x: number = 42;
    fn parse_variable_declaration(&mut self) -> DrafResult<Statement> {
        let start_location = self.current_location();
        let kind_token = self.current_token.unwrap();
        let kind = match kind_token.kind {
            TokenKind::Let => VariableKind::Let,
            TokenKind::Const => VariableKind::Const,
            TokenKind::Var => VariableKind::Var,
            _ => unreachable!(),
        };
        self.advance();

        let name_token = self.consume(TokenKind::Identifier, "Expected variable name")?;
        let name = name_token.lexeme.clone();

        let type_annotation = if self.check(&TokenKind::Colon) {
            self.advance(); // consume ':'
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        let initializer = if self.check(&TokenKind::Equal) {
            self.advance(); // consume '='
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume_optional_semicolon();

        Ok(Statement::VariableDeclaration {
            name,
            type_annotation,
            initializer,
            kind,
            location: start_location,
        })
    }

    /// Parse function declaration (placeholder)
    fn parse_function_declaration(&mut self) -> DrafResult<Statement> {
        Err(DrafError::parse_error(
            self.current_token.unwrap().line,
            self.current_token.unwrap().column,
            "Function declarations not yet implemented",
        ))
    }

    /// Parse interface declaration (placeholder)
    fn parse_interface_declaration(&mut self) -> DrafResult<Statement> {
        Err(DrafError::parse_error(
            self.current_token.unwrap().line,
            self.current_token.unwrap().column,
            "Interface declarations not yet implemented",
        ))
    }

    /// Parse type alias (placeholder)
    fn parse_type_alias(&mut self) -> DrafResult<Statement> {
        Err(DrafError::parse_error(
            self.current_token.unwrap().line,
            self.current_token.unwrap().column,
            "Type aliases not yet implemented",
        ))
    }

    /// Parse if statement (placeholder)
    fn parse_if_statement(&mut self) -> DrafResult<Statement> {
        Err(DrafError::parse_error(
            self.current_token.unwrap().line,
            self.current_token.unwrap().column,
            "If statements not yet implemented",
        ))
    }

    /// Parse while statement (placeholder)
    fn parse_while_statement(&mut self) -> DrafResult<Statement> {
        Err(DrafError::parse_error(
            self.current_token.unwrap().line,
            self.current_token.unwrap().column,
            "While statements not yet implemented",
        ))
    }

    /// Parse for statement (placeholder)
    fn parse_for_statement(&mut self) -> DrafResult<Statement> {
        Err(DrafError::parse_error(
            self.current_token.unwrap().line,
            self.current_token.unwrap().column,
            "For statements not yet implemented",
        ))
    }

    /// Parse return statement (placeholder)
    fn parse_return_statement(&mut self) -> DrafResult<Statement> {
        Err(DrafError::parse_error(
            self.current_token.unwrap().line,
            self.current_token.unwrap().column,
            "Return statements not yet implemented",
        ))
    }

    /// Parse block statement (placeholder)
    fn parse_block_statement(&mut self) -> DrafResult<Statement> {
        Err(DrafError::parse_error(
            self.current_token.unwrap().line,
            self.current_token.unwrap().column,
            "Block statements not yet implemented",
        ))
    }

    /// Parse expression statement
    fn parse_expression_statement(&mut self) -> DrafResult<Statement> {
        let start_location = self.current_location();
        let expression = self.parse_expression()?;
        self.consume_optional_semicolon();

        Ok(Statement::ExpressionStatement {
            expression,
            location: start_location,
        })
    }

    /// Parse expression using precedence climbing
    fn parse_expression(&mut self) -> DrafResult<Expression> {
        self.parse_assignment()
    }

    /// Parse assignment expression
    fn parse_assignment(&mut self) -> DrafResult<Expression> {
        let expr = self.parse_logical_or()?;

        if self.check(&TokenKind::Equal) {
            let location = self.current_location();
            self.advance();
            let value = self.parse_assignment()?;
            return Ok(Expression::Assignment {
                target: Box::new(expr),
                value: Box::new(value),
                location,
            });
        }

        Ok(expr)
    }

    /// Parse logical OR expression
    fn parse_logical_or(&mut self) -> DrafResult<Expression> {
        let mut expr = self.parse_logical_and()?;

        while self.check(&TokenKind::OrOr) {
            let location = self.current_location();
            self.advance();
            let right = self.parse_logical_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::LogicalOr,
                right: Box::new(right),
                location,
            };
        }

        Ok(expr)
    }

    /// Parse logical AND expression
    fn parse_logical_and(&mut self) -> DrafResult<Expression> {
        let mut expr = self.parse_equality()?;

        while self.check(&TokenKind::AndAnd) {
            let location = self.current_location();
            self.advance();
            let right = self.parse_equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::LogicalAnd,
                right: Box::new(right),
                location,
            };
        }

        Ok(expr)
    }

    /// Parse equality expression
    fn parse_equality(&mut self) -> DrafResult<Expression> {
        let mut expr = self.parse_comparison()?;

        while let Some(token) = self.current_token {
            let operator = match token.kind {
                TokenKind::EqualEqual => BinaryOperator::Equal,
                TokenKind::NotEqual => BinaryOperator::NotEqual,
                _ => break,
            };

            let location = self.current_location();
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                location,
            };
        }

        Ok(expr)
    }

    /// Parse comparison expression
    fn parse_comparison(&mut self) -> DrafResult<Expression> {
        let mut expr = self.parse_term()?;

        while let Some(token) = self.current_token {
            let operator = match token.kind {
                TokenKind::Greater => BinaryOperator::GreaterThan,
                TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
                TokenKind::Less => BinaryOperator::LessThan,
                TokenKind::LessEqual => BinaryOperator::LessEqual,
                _ => break,
            };

            let location = self.current_location();
            self.advance();
            let right = self.parse_term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                location,
            };
        }

        Ok(expr)
    }

    /// Parse term expression (+ -)
    fn parse_term(&mut self) -> DrafResult<Expression> {
        let mut expr = self.parse_factor()?;

        while let Some(token) = self.current_token {
            let operator = match token.kind {
                TokenKind::Minus => BinaryOperator::Subtract,
                TokenKind::Plus => BinaryOperator::Add,
                _ => break,
            };

            let location = self.current_location();
            self.advance();
            let right = self.parse_factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                location,
            };
        }

        Ok(expr)
    }

    /// Parse factor expression (* / %)
    fn parse_factor(&mut self) -> DrafResult<Expression> {
        let mut expr = self.parse_unary()?;

        while let Some(token) = self.current_token {
            let operator = match token.kind {
                TokenKind::Slash => BinaryOperator::Divide,
                TokenKind::Star => BinaryOperator::Multiply,
                TokenKind::Percent => BinaryOperator::Modulo,
                _ => break,
            };

            let location = self.current_location();
            self.advance();
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                location,
            };
        }

        Ok(expr)
    }

    /// Parse unary expression
    fn parse_unary(&mut self) -> DrafResult<Expression> {
        if let Some(token) = self.current_token {
            let operator = match token.kind {
                TokenKind::Bang => UnaryOperator::LogicalNot,
                TokenKind::Minus => UnaryOperator::Minus,
                TokenKind::Plus => UnaryOperator::Plus,
                _ => return self.parse_primary(),
            };

            let location = self.current_location();
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                operator,
                operand: Box::new(operand),
                location,
            });
        }

        self.parse_primary()
    }

    /// Parse primary expression
    fn parse_primary(&mut self) -> DrafResult<Expression> {
        if let Some(token) = self.current_token {
            let location = self.current_location();

            match &token.kind {
                TokenKind::True => {
                    self.advance();
                    Ok(Expression::Literal {
                        value: LiteralValue::Boolean(true),
                        location,
                    })
                }
                TokenKind::False => {
                    self.advance();
                    Ok(Expression::Literal {
                        value: LiteralValue::Boolean(false),
                        location,
                    })
                }
                TokenKind::Null => {
                    self.advance();
                    Ok(Expression::Literal {
                        value: LiteralValue::Null,
                        location,
                    })
                }
                TokenKind::Undefined => {
                    self.advance();
                    Ok(Expression::Literal {
                        value: LiteralValue::Undefined,
                        location,
                    })
                }
                TokenKind::NumberLiteral => {
                    let value = token.lexeme.parse::<f64>().map_err(|_| {
                        DrafError::parse_error(token.line, token.column, "Invalid number literal")
                    })?;
                    self.advance();
                    Ok(Expression::Literal {
                        value: LiteralValue::Number(value),
                        location,
                    })
                }
                TokenKind::StringLiteral => {
                    // Remove quotes from string literal
                    let value = token.lexeme[1..token.lexeme.len() - 1].to_string();
                    self.advance();
                    Ok(Expression::Literal {
                        value: LiteralValue::String(value),
                        location,
                    })
                }
                TokenKind::Console => {
                    self.advance(); // consume 'console'
                    self.consume(TokenKind::Dot, "Expected '.' after 'console'")?;

                    let method_token = self.consume(
                        TokenKind::Identifier,
                        "Expected method name after 'console.'",
                    )?;
                    let method = match method_token.lexeme.as_str() {
                        "log" => ConsoleMethod::Log,
                        "info" => ConsoleMethod::Info,
                        "warn" => ConsoleMethod::Warn,
                        "error" => ConsoleMethod::Error,
                        "debug" => ConsoleMethod::Debug,
                        _ => {
                            return Err(DrafError::parse_error(
                                method_token.line,
                                method_token.column,
                                format!("Unknown console method: {}", method_token.lexeme),
                            ));
                        }
                    };

                    self.consume(TokenKind::LeftParen, "Expected '(' after console method")?;

                    let mut arguments = Vec::new();
                    if !self.check(&TokenKind::RightParen) {
                        loop {
                            arguments.push(self.parse_expression()?);
                            if !self.check(&TokenKind::Comma) {
                                break;
                            }
                            self.advance(); // consume ','
                        }
                    }

                    self.consume(
                        TokenKind::RightParen,
                        "Expected ')' after console arguments",
                    )?;

                    Ok(Expression::ConsoleCall {
                        method,
                        arguments,
                        location,
                    })
                }
                TokenKind::Identifier => {
                    let name = token.lexeme.clone();
                    self.advance();
                    Ok(Expression::Identifier { name, location })
                }
                TokenKind::LeftParen => {
                    self.advance(); // consume '('
                    let expr = self.parse_expression()?;
                    self.consume(TokenKind::RightParen, "Expected ')' after expression")?;
                    Ok(expr)
                }
                _ => Err(DrafError::parse_error(
                    token.line,
                    token.column,
                    format!("Unexpected token: {}", token.kind),
                )),
            }
        } else {
            Err(DrafError::parse_error(
                0,
                0,
                "Unexpected end of file in expression",
            ))
        }
    }

    /// Parse type annotation
    fn parse_type_annotation(&mut self) -> DrafResult<TypeAnnotation> {
        if let Some(token) = self.current_token {
            let location = self.current_location();

            match &token.kind {
                TokenKind::Number => {
                    self.advance();
                    Ok(TypeAnnotation::Number { location })
                }
                TokenKind::String => {
                    self.advance();
                    Ok(TypeAnnotation::String { location })
                }
                TokenKind::Boolean => {
                    self.advance();
                    Ok(TypeAnnotation::Boolean { location })
                }
                TokenKind::Any => {
                    self.advance();
                    Ok(TypeAnnotation::Any { location })
                }
                TokenKind::Null => {
                    self.advance();
                    Ok(TypeAnnotation::Null { location })
                }
                TokenKind::Undefined => {
                    self.advance();
                    Ok(TypeAnnotation::Undefined { location })
                }
                TokenKind::Void => {
                    self.advance();
                    Ok(TypeAnnotation::Void { location })
                }
                TokenKind::Never => {
                    self.advance();
                    Ok(TypeAnnotation::Never { location })
                }
                TokenKind::Identifier => {
                    let name = token.lexeme.clone();
                    self.advance();
                    Ok(TypeAnnotation::Named { name, location })
                }
                _ => Err(DrafError::parse_error(
                    token.line,
                    token.column,
                    format!("Expected type, found {}", token.kind),
                )),
            }
        } else {
            Err(DrafError::parse_error(
                0,
                0,
                "Expected type, found end of file",
            ))
        }
    }

    /// Get current source location
    fn current_location(&self) -> SourceLocation {
        if let Some(token) = self.current_token {
            SourceLocation::new(token.line, token.column, token.start, token.end)
        } else {
            SourceLocation::new(0, 0, 0, 0)
        }
    }

    /// Check if we're at the end of input
    fn is_at_end(&self) -> bool {
        match self.current_token {
            Some(token) => token.kind == TokenKind::Eof,
            None => true,
        }
    }

    /// Consume optional semicolon
    fn consume_optional_semicolon(&mut self) {
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }
    }
}

/// Parse tokens into an AST
pub fn parse(tokens: Vec<Token>) -> DrafResult<Program> {
    let mut parser = Parser::new(&tokens);
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    #[test]
    fn test_parse_variable_declaration() {
        let source = "let x: number = 42;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parse(tokens).unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::VariableDeclaration {
                name,
                kind,
                type_annotation,
                initializer,
                ..
            } => {
                assert_eq!(name, "x");
                assert_eq!(kind, &VariableKind::Let);
                assert!(type_annotation.is_some());
                assert!(initializer.is_some());
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_expression() {
        let source = "x + y * 2;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parse(tokens).unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::ExpressionStatement { expression, .. } => match expression {
                Expression::Binary { operator, .. } => {
                    assert_eq!(operator, &BinaryOperator::Add);
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_literals() {
        let test_cases = vec![
            ("42;", LiteralValue::Number(42.0)),
            ("true;", LiteralValue::Boolean(true)),
            ("false;", LiteralValue::Boolean(false)),
            ("null;", LiteralValue::Null),
            ("undefined;", LiteralValue::Undefined),
            ("\"hello\";", LiteralValue::String("hello".to_string())),
        ];

        for (source, expected_value) in test_cases {
            let tokens = lexer::tokenize(source).unwrap();
            let program = parse(tokens).unwrap();

            match &program.statements[0] {
                Statement::ExpressionStatement { expression, .. } => match expression {
                    Expression::Literal { value, .. } => {
                        assert_eq!(value, &expected_value);
                    }
                    _ => panic!("Expected literal expression for: {}", source),
                },
                _ => panic!("Expected expression statement for: {}", source),
            }
        }
    }

    #[test]
    fn test_parse_error_handling() {
        let source = "let 123 = x;"; // Invalid identifier
        let tokens = lexer::tokenize(source).unwrap();
        let result = parse(tokens);

        assert!(result.is_err());
    }

    #[test]
    fn test_operator_precedence() {
        let source = "a + b * c;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parse(tokens).unwrap();

        match &program.statements[0] {
            Statement::ExpressionStatement { expression, .. } => {
                match expression {
                    Expression::Binary {
                        left,
                        operator,
                        right,
                        ..
                    } => {
                        assert_eq!(operator, &BinaryOperator::Add);
                        // Right side should be the multiplication
                        match right.as_ref() {
                            Expression::Binary { operator, .. } => {
                                assert_eq!(operator, &BinaryOperator::Multiply);
                            }
                            _ => panic!("Expected multiplication on right side"),
                        }
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }
}
