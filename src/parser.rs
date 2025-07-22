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
        // Skip any leading newlines
        while self.check(&TokenKind::Newline) {
            self.advance();
        }

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
                TokenKind::Break => self.parse_break_statement(),
                TokenKind::Continue => self.parse_continue_statement(),
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

            // Skip any newlines after the assignment operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

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

    /// Parse interface declaration
    fn parse_interface_declaration(&mut self) -> DrafResult<Statement> {
        let location = self.current_location();
        self.advance(); // consume 'interface'

        // Parse interface name
        let name = if let Some(token) = self.current_token {
            if matches!(token.kind, TokenKind::Identifier) {
                let name = token.lexeme.clone();
                self.advance();
                name
            } else {
                return Err(DrafError::parse_error(
                    token.line,
                    token.column,
                    "Expected interface name",
                ));
            }
        } else {
            return Err(DrafError::parse_error(
                location.line,
                location.column,
                "Expected interface name",
            ));
        };

        // Parse type parameters if present
        let type_parameters = if self.check(&TokenKind::Less) {
            self.parse_type_parameters()?
        } else {
            Vec::new()
        };

        // Parse extends clause if present
        let extends = if self.check(&TokenKind::Identifier)
            && self.current_token.as_ref().unwrap().lexeme == "extends"
        {
            self.advance(); // consume 'extends'
            let mut extended_interfaces = Vec::new();

            loop {
                if let Some(token) = self.current_token {
                    if matches!(token.kind, TokenKind::Identifier) {
                        extended_interfaces.push(token.lexeme.clone());
                        self.advance();

                        if self.check(&TokenKind::Comma) {
                            self.advance(); // consume ','
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            extended_interfaces
        } else {
            Vec::new()
        };

        self.consume(
            TokenKind::LeftBrace,
            "Expected '{' after interface declaration",
        )?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }

            let member_location = self.current_location();

            // Check for readonly modifier
            let readonly = if self.check(&TokenKind::Identifier)
                && self.current_token.as_ref().unwrap().lexeme == "readonly"
            {
                self.advance(); // consume 'readonly'
                true
            } else {
                false
            };

            // Parse member name
            let member_name = if let Some(token) = self.current_token {
                if matches!(token.kind, TokenKind::Identifier) {
                    let name = token.lexeme.clone();
                    self.advance();
                    name
                } else {
                    return Err(DrafError::parse_error(
                        token.line,
                        token.column,
                        "Expected member name",
                    ));
                }
            } else {
                return Err(DrafError::parse_error(
                    member_location.line,
                    member_location.column,
                    "Expected member name",
                ));
            };

            let optional = if self.check(&TokenKind::Question) {
                self.advance(); // consume '?'
                true
            } else {
                false
            };

            // Check if this is a method (has parentheses after optional marker)
            let is_method = if optional {
                // Look ahead after the colon to see if there's a function type
                self.consume(TokenKind::Colon, "Expected ':' after member name")?;
                self.check(&TokenKind::LeftParen)
            } else {
                // Check if we have parentheses directly (method without optional marker)
                if self.check(&TokenKind::LeftParen) {
                    true
                } else {
                    self.consume(TokenKind::Colon, "Expected ':' after member name")?;
                    self.check(&TokenKind::LeftParen)
                }
            };

            if is_method {
                // Parse method
                let method_type = self.parse_function_type()?;
                if let TypeAnnotation::Function {
                    parameters,
                    return_type,
                    ..
                } = method_type
                {
                    methods.push(InterfaceMethod {
                        name: member_name,
                        type_parameters: Vec::new(), // TODO: Support method generics
                        parameters: parameters
                            .iter()
                            .enumerate()
                            .map(|(i, param_type)| MethodParameter {
                                name: format!("param{}", i),
                                param_type: param_type.clone(),
                                optional: false,
                                location: member_location.clone(),
                            })
                            .collect(),
                        return_type: *return_type,
                        optional,
                        location: member_location,
                    });
                }
            } else {
                // Parse field
                let field_type = self.parse_type_annotation()?;
                fields.push(InterfaceField {
                    name: member_name,
                    field_type,
                    optional,
                    readonly,
                    location: member_location,
                });
            }

            // Optional semicolon or comma
            if self.check(&TokenKind::Semicolon) || self.check(&TokenKind::Comma) {
                self.advance();
            }
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after interface body")?;

        Ok(Statement::InterfaceDeclaration {
            name,
            type_parameters,
            extends,
            fields,
            methods,
            location,
        })
    }

    /// Parse type alias
    fn parse_type_alias(&mut self) -> DrafResult<Statement> {
        let location = self.current_location();
        self.advance(); // consume 'type'

        // Parse type alias name
        let name = if let Some(token) = self.current_token {
            if matches!(token.kind, TokenKind::Identifier) {
                let name = token.lexeme.clone();
                self.advance();
                name
            } else {
                return Err(DrafError::parse_error(
                    token.line,
                    token.column,
                    "Expected type alias name",
                ));
            }
        } else {
            return Err(DrafError::parse_error(
                location.line,
                location.column,
                "Expected type alias name",
            ));
        };

        // Parse type parameters if present
        let type_parameters = if self.check(&TokenKind::Less) {
            self.parse_type_parameters()?
        } else {
            Vec::new()
        };

        self.consume(TokenKind::Equal, "Expected '=' after type alias name")?;
        let type_annotation = self.parse_type_annotation()?;

        // Optional semicolon
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }

        Ok(Statement::TypeAlias {
            name,
            type_parameters,
            type_annotation,
            location,
        })
    }

    /// Parse type parameters <T, U extends Something>
    fn parse_type_parameters(&mut self) -> DrafResult<Vec<TypeParameter>> {
        let mut parameters = Vec::new();

        self.consume(TokenKind::Less, "Expected '<' for type parameters")?;

        while !self.check(&TokenKind::Greater) && !self.is_at_end() {
            let param_location = self.current_location();

            // Parse parameter name
            let param_name = if let Some(token) = self.current_token {
                if matches!(token.kind, TokenKind::Identifier) {
                    let name = token.lexeme.clone();
                    self.advance();
                    name
                } else {
                    return Err(DrafError::parse_error(
                        token.line,
                        token.column,
                        "Expected type parameter name",
                    ));
                }
            } else {
                return Err(DrafError::parse_error(
                    param_location.line,
                    param_location.column,
                    "Expected type parameter name",
                ));
            };

            // Parse extends constraint if present
            let constraint = if self.check(&TokenKind::Identifier)
                && self.current_token.as_ref().unwrap().lexeme == "extends"
            {
                self.advance(); // consume 'extends'
                Some(self.parse_type_annotation()?)
            } else {
                None
            };

            // Parse default type if present
            let default = if self.check(&TokenKind::Equal) {
                self.advance(); // consume '='
                Some(self.parse_type_annotation()?)
            } else {
                None
            };

            parameters.push(TypeParameter {
                name: param_name,
                constraint,
                default,
                location: param_location,
            });

            if self.check(&TokenKind::Comma) {
                self.advance(); // consume ','
            } else {
                break;
            }
        }

        self.consume(TokenKind::Greater, "Expected '>' after type parameters")?;
        Ok(parameters)
    }

    /// Parse if statement with support for else and else if
    fn parse_if_statement(&mut self) -> DrafResult<Statement> {
        let start_location = self.current_location();
        self.advance(); // consume 'if'

        self.consume(TokenKind::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenKind::RightParen, "Expected ')' after if condition")?;

        // Skip any newlines before the then branch
        while self.check(&TokenKind::Newline) {
            self.advance();
        }

        let then_branch = Box::new(self.parse_statement()?);

        let else_branch = if self.check(&TokenKind::Else) {
            self.advance(); // consume 'else'

            // Skip any newlines after 'else'
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            // Check if this is an else if
            if self.check(&TokenKind::If) {
                Some(Box::new(self.parse_if_statement()?))
            } else {
                Some(Box::new(self.parse_statement()?))
            }
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
            location: start_location,
        })
    }

    /// Parse while statement: while (condition) { body }
    fn parse_while_statement(&mut self) -> DrafResult<Statement> {
        let location = self.current_location();
        self.consume(TokenKind::While, "Expected 'while'")?;
        self.consume(TokenKind::LeftParen, "Expected '(' after 'while'")?;

        let condition = self.parse_expression()?;

        self.consume(TokenKind::RightParen, "Expected ')' after while condition")?;

        let body = Box::new(self.parse_statement()?);

        Ok(Statement::While {
            condition,
            body,
            location,
        })
    }

    /// Parse for statement: for (init; condition; update) { body }
    fn parse_for_statement(&mut self) -> DrafResult<Statement> {
        let location = self.current_location();
        self.consume(TokenKind::For, "Expected 'for'")?;
        self.consume(TokenKind::LeftParen, "Expected '(' after 'for'")?;

        // Parse init (optional)
        let init = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            // Parse init statement without consuming semicolon
            let init_stmt = match &self.current_token {
                Some(token) => match &token.kind {
                    TokenKind::Let | TokenKind::Const | TokenKind::Var => {
                        self.parse_variable_declaration_no_semicolon()?
                    }
                    _ => self.parse_expression_statement_no_semicolon()?,
                },
                None => return Err(DrafError::parse_error(0, 0, "Unexpected end of input")),
            };
            Some(Box::new(init_stmt))
        };
        self.consume(TokenKind::Semicolon, "Expected ';' after for loop init")?;

        // Parse condition (optional)
        let condition = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.consume(
            TokenKind::Semicolon,
            "Expected ';' after for loop condition",
        )?;

        // Parse update (optional)
        let update = if self.check(&TokenKind::RightParen) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.consume(TokenKind::RightParen, "Expected ')' after for loop header")?;

        let body = Box::new(self.parse_statement()?);

        Ok(Statement::For {
            init,
            condition,
            update,
            body,
            location,
        })
    }

    /// Parse return statement (placeholder)
    fn parse_return_statement(&mut self) -> DrafResult<Statement> {
        Err(DrafError::parse_error(
            self.current_token.unwrap().line,
            self.current_token.unwrap().column,
            "Return statements not yet implemented",
        ))
    }

    /// Parse break statement
    fn parse_break_statement(&mut self) -> DrafResult<Statement> {
        let location = self.current_location();
        self.consume(TokenKind::Break, "Expected 'break'")?;

        // Optional semicolon
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }

        Ok(Statement::Break { location })
    }

    /// Parse continue statement
    fn parse_continue_statement(&mut self) -> DrafResult<Statement> {
        let location = self.current_location();
        self.consume(TokenKind::Continue, "Expected 'continue'")?;

        // Optional semicolon
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }

        Ok(Statement::Continue { location })
    }

    /// Parse variable declaration without consuming semicolon (for use in for loops)
    fn parse_variable_declaration_no_semicolon(&mut self) -> DrafResult<Statement> {
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

            // Skip any newlines after the assignment operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            Some(self.parse_expression()?)
        } else {
            None
        };

        // Don't consume semicolon here

        Ok(Statement::VariableDeclaration {
            name,
            type_annotation,
            initializer,
            kind,
            location: start_location,
        })
    }

    /// Parse expression statement without consuming semicolon (for use in for loops)
    fn parse_expression_statement_no_semicolon(&mut self) -> DrafResult<Statement> {
        let start_location = self.current_location();
        let expression = self.parse_expression()?;

        Ok(Statement::ExpressionStatement {
            expression,
            location: start_location,
        })
    }

    /// Parse block statement
    fn parse_block_statement(&mut self) -> DrafResult<Statement> {
        let start_location = self.current_location();
        self.consume(TokenKind::LeftBrace, "Expected '{'")?;

        let mut statements = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip any newlines between statements
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            // Check again for right brace after skipping newlines
            if self.check(&TokenKind::RightBrace) {
                break;
            }

            statements.push(self.parse_statement()?);
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after block")?;

        Ok(Statement::Block(Block {
            statements,
            location: start_location,
        }))
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
        let expr = self.parse_ternary()?;

        if self.check(&TokenKind::Equal) {
            let location = self.current_location();
            self.advance();

            // Skip any newlines after the assignment operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            let value = self.parse_assignment()?;
            return Ok(Expression::Assignment {
                target: Box::new(expr),
                value: Box::new(value),
                location,
            });
        }

        Ok(expr)
    }

    /// Parse ternary conditional expression
    fn parse_ternary(&mut self) -> DrafResult<Expression> {
        let expr = self.parse_logical_or()?;

        if self.check(&TokenKind::Question) {
            let location = self.current_location();
            self.advance(); // consume '?'

            // Skip any newlines after the '?' operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            let then_expr = self.parse_expression()?;
            self.consume(TokenKind::Colon, "Expected ':' in ternary expression")?;

            // Skip any newlines after the ':' operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            let else_expr = self.parse_ternary()?;
            return Ok(Expression::Conditional {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
                location,
            });
        }

        Ok(expr)
    }

    /// Parse logical OR expression
    fn parse_logical_or(&mut self) -> DrafResult<Expression> {
        let mut expr = self.parse_nullish_coalescing()?;

        while self.check(&TokenKind::OrOr) {
            let location = self.current_location();
            self.advance();

            // Skip any newlines after the operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            let right = self.parse_nullish_coalescing()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::LogicalOr,
                right: Box::new(right),
                location,
            };
        }

        Ok(expr)
    }

    /// Parse nullish coalescing expression
    fn parse_nullish_coalescing(&mut self) -> DrafResult<Expression> {
        let mut expr = self.parse_logical_and()?;

        while self.check(&TokenKind::NullishCoalescing) {
            let location = self.current_location();
            self.advance();

            // Skip any newlines after the operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            let right = self.parse_logical_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::NullishCoalescing,
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

            // Skip any newlines after the operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

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
                TokenKind::StrictEqual => BinaryOperator::StrictEqual,
                TokenKind::StrictNotEqual => BinaryOperator::StrictNotEqual,
                _ => break,
            };

            let location = self.current_location();
            self.advance();

            // Skip any newlines after the operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

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

            // Skip any newlines after the operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

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

            // Skip any newlines after the operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

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

            // Skip any newlines after the operator
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

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
                TokenKind::StringLiteralDouble => {
                    // Remove quotes from string literal
                    let content = token.lexeme[1..token.lexeme.len() - 1].to_string();
                    let string_literal = crate::strings::StringLiteral::new_regular(
                        content,
                        crate::strings::StringLiteralType::DoubleQuoted,
                    );
                    self.advance();
                    Ok(Expression::Literal {
                        value: LiteralValue::StringLiteral(string_literal),
                        location,
                    })
                }
                TokenKind::StringLiteralSingle => {
                    // Remove quotes from string literal
                    let content = token.lexeme[1..token.lexeme.len() - 1].to_string();
                    let string_literal = crate::strings::StringLiteral::new_regular(
                        content,
                        crate::strings::StringLiteralType::SingleQuoted,
                    );
                    self.advance();
                    Ok(Expression::Literal {
                        value: LiteralValue::StringLiteral(string_literal),
                        location,
                    })
                }
                TokenKind::TemplateLiteral => {
                    // Remove backticks from template literal
                    let content = token.lexeme[1..token.lexeme.len() - 1].to_string();
                    self.advance();
                    self.parse_template_literal(content, location)
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

                    // Skip any newlines and whitespace after opening parenthesis
                    while self.check(&TokenKind::Newline) {
                        self.advance();
                    }

                    let mut arguments = Vec::new();

                    // Parse arguments with comprehensive newline handling
                    while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
                        // Skip any leading newlines before argument
                        while self.check(&TokenKind::Newline) {
                            self.advance();
                        }

                        // Break if we hit the closing paren after skipping newlines
                        if self.check(&TokenKind::RightParen) {
                            break;
                        }

                        // Parse the argument
                        arguments.push(self.parse_expression()?);

                        // Skip newlines after expression
                        while self.check(&TokenKind::Newline) {
                            self.advance();
                        }

                        // Check for comma or end
                        if self.check(&TokenKind::Comma) {
                            self.advance(); // consume ','

                            // Skip newlines after comma
                            while self.check(&TokenKind::Newline) {
                                self.advance();
                            }
                        } else {
                            // No comma, so this should be the last argument
                            // Skip any remaining newlines before closing paren
                            while self.check(&TokenKind::Newline) {
                                self.advance();
                            }
                            break;
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
                TokenKind::LeftBrace => {
                    // Object literal { key: value, ... }
                    self.parse_object_literal()
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

    /// Parse object literal { key: value, ... }
    fn parse_object_literal(&mut self) -> DrafResult<Expression> {
        let location = self.current_location();
        self.advance(); // consume '{'

        // Skip newlines after opening brace
        while self.check(&TokenKind::Newline) {
            self.advance();
        }

        let mut fields = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines before field
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            // Break if we hit the closing brace after skipping newlines
            if self.check(&TokenKind::RightBrace) {
                break;
            }

            let field_location = self.current_location();

            // Parse field name (identifier or string literal)
            let key = if let Some(token) = self.current_token {
                match &token.kind {
                    TokenKind::Identifier => {
                        let name = token.lexeme.clone();
                        self.advance();
                        name
                    }
                    TokenKind::StringLiteralDouble | TokenKind::StringLiteralSingle => {
                        let name = token.lexeme.clone();
                        self.advance();
                        // Remove quotes from string literal
                        name.trim_matches('"').trim_matches('\'').to_string()
                    }
                    _ => {
                        return Err(DrafError::parse_error(
                            token.line,
                            token.column,
                            "Expected field name in object literal",
                        ));
                    }
                }
            } else {
                return Err(DrafError::parse_error(
                    field_location.line,
                    field_location.column,
                    "Expected field name in object literal",
                ));
            };

            self.consume(TokenKind::Colon, "Expected ':' after field name")?;

            // Skip newlines after colon
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            let value = self.parse_expression()?;

            fields.push(ObjectField::new(key, value, field_location));

            // Skip newlines after field value
            while self.check(&TokenKind::Newline) {
                self.advance();
            }

            // Check for comma or end
            if self.check(&TokenKind::Comma) {
                self.advance(); // consume ','

                // Skip newlines after comma
                while self.check(&TokenKind::Newline) {
                    self.advance();
                }
            } else {
                // No comma, skip any newlines before closing brace
                while self.check(&TokenKind::Newline) {
                    self.advance();
                }
                break;
            }
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after object literal")?;

        Ok(Expression::Object { fields, location })
    }

    /// Parse type annotation
    fn parse_type_annotation(&mut self) -> DrafResult<TypeAnnotation> {
        self.parse_union_type()
    }

    /// Parse union type (A | B | C)
    fn parse_union_type(&mut self) -> DrafResult<TypeAnnotation> {
        let mut types = vec![self.parse_intersection_type()?];

        while self.check(&TokenKind::Pipe) {
            self.advance(); // consume '|'
            types.push(self.parse_intersection_type()?);
        }

        if types.len() == 1 {
            Ok(types.into_iter().next().unwrap())
        } else {
            Ok(TypeAnnotation::Union {
                types,
                location: self.current_location(),
            })
        }
    }

    /// Parse intersection type (A & B & C)
    fn parse_intersection_type(&mut self) -> DrafResult<TypeAnnotation> {
        let mut types = vec![self.parse_primary_type()?];

        while self.check(&TokenKind::AndAnd) {
            self.advance(); // consume '&'
            types.push(self.parse_primary_type()?);
        }

        if types.len() == 1 {
            Ok(types.into_iter().next().unwrap())
        } else {
            Ok(TypeAnnotation::Intersection {
                types,
                location: self.current_location(),
            })
        }
    }

    /// Parse primary type (primitives, identifiers, arrays, etc.)
    fn parse_primary_type(&mut self) -> DrafResult<TypeAnnotation> {
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

                    // Check for generic type arguments
                    if self.check(&TokenKind::Less) {
                        self.advance(); // consume '<'

                        // Skip newlines after opening bracket
                        while self.check(&TokenKind::Newline) {
                            self.advance();
                        }

                        let mut type_arguments = Vec::new();

                        while !self.check(&TokenKind::Greater) && !self.is_at_end() {
                            type_arguments.push(self.parse_type_annotation()?);

                            // Skip newlines after type argument
                            while self.check(&TokenKind::Newline) {
                                self.advance();
                            }

                            if self.check(&TokenKind::Comma) {
                                self.advance(); // consume ','

                                // Skip newlines after comma
                                while self.check(&TokenKind::Newline) {
                                    self.advance();
                                }
                            } else {
                                break;
                            }
                        }

                        // Skip newlines before closing bracket
                        while self.check(&TokenKind::Newline) {
                            self.advance();
                        }

                        self.consume(TokenKind::Greater, "Expected '>' after type arguments")?;

                        Ok(TypeAnnotation::Generic {
                            name,
                            type_arguments,
                            location,
                        })
                    } else {
                        Ok(TypeAnnotation::Named { name, location })
                    }
                }
                TokenKind::LeftBrace => {
                    // Object type { field: Type }
                    self.parse_object_type()
                }
                TokenKind::LeftParen => {
                    // Function type (param: Type) => ReturnType
                    self.parse_function_type()
                }
                TokenKind::LeftBracket => {
                    // Tuple type [Type1, Type2]
                    self.parse_tuple_type()
                }
                TokenKind::StringLiteralDouble | TokenKind::StringLiteralSingle => {
                    // String literal type "value"
                    let value = token.lexeme.clone();
                    self.advance();
                    Ok(TypeAnnotation::Named {
                        name: value,
                        location,
                    })
                }
                TokenKind::NumberLiteral => {
                    // Number literal type 42
                    let value = token.lexeme.clone();
                    self.advance();
                    Ok(TypeAnnotation::Named {
                        name: value,
                        location,
                    })
                }
                TokenKind::True | TokenKind::False => {
                    // Boolean literal type true | false
                    let value = token.lexeme.clone();
                    self.advance();
                    Ok(TypeAnnotation::Named {
                        name: value,
                        location,
                    })
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

    /// Parse object type { field: Type, ... }
    fn parse_object_type(&mut self) -> DrafResult<TypeAnnotation> {
        let location = self.current_location();
        self.advance(); // consume '{'

        let mut fields = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }

            let field_location = self.current_location();
            let readonly = if self.check(&TokenKind::Identifier)
                && self.current_token.as_ref().unwrap().lexeme == "readonly"
            {
                self.advance(); // consume 'readonly'
                true
            } else {
                false
            };

            // Parse field name
            let field_name = if let Some(token) = self.current_token {
                if matches!(token.kind, TokenKind::Identifier) {
                    let name = token.lexeme.clone();
                    self.advance();
                    name
                } else {
                    return Err(DrafError::parse_error(
                        token.line,
                        token.column,
                        "Expected field name",
                    ));
                }
            } else {
                return Err(DrafError::parse_error(
                    field_location.line,
                    field_location.column,
                    "Expected field name",
                ));
            };

            let optional = if self.check(&TokenKind::Question) {
                self.advance(); // consume '?'
                true
            } else {
                false
            };

            self.consume(TokenKind::Colon, "Expected ':' after field name")?;
            let field_type = self.parse_type_annotation()?;

            fields.push(TypeAnnotationField {
                name: field_name,
                field_type,
                optional,
                readonly,
                location: field_location,
            });

            if self.check(&TokenKind::Comma) || self.check(&TokenKind::Semicolon) {
                self.advance(); // consume ',' or ';'
            } else {
                break;
            }
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after object type")?;

        Ok(TypeAnnotation::Object { fields, location })
    }

    /// Parse function type (param: Type) => ReturnType
    fn parse_function_type(&mut self) -> DrafResult<TypeAnnotation> {
        let location = self.current_location();
        self.advance(); // consume '('

        let mut parameters = Vec::new();

        while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
            parameters.push(self.parse_type_annotation()?);
            if self.check(&TokenKind::Comma) {
                self.advance(); // consume ','
            } else {
                break;
            }
        }

        self.consume(
            TokenKind::RightParen,
            "Expected ')' after function parameters",
        )?;
        self.consume(TokenKind::Arrow, "Expected '=>' after function parameters")?;
        let return_type = Box::new(self.parse_type_annotation()?);

        Ok(TypeAnnotation::Function {
            parameters,
            return_type,
            location,
        })
    }

    /// Parse tuple type [Type1, Type2, ...]
    fn parse_tuple_type(&mut self) -> DrafResult<TypeAnnotation> {
        let location = self.current_location();
        self.advance(); // consume '['

        let mut elements = Vec::new();

        while !self.check(&TokenKind::RightBracket) && !self.is_at_end() {
            elements.push(self.parse_type_annotation()?);
            if self.check(&TokenKind::Comma) {
                self.advance(); // consume ','
            } else {
                break;
            }
        }

        self.consume(TokenKind::RightBracket, "Expected ']' after tuple elements")?;

        Ok(TypeAnnotation::Tuple { elements, location })
    }

    /// Parse template literal with interpolation
    fn parse_template_literal(
        &mut self,
        content: String,
        location: SourceLocation,
    ) -> DrafResult<Expression> {
        use crate::ast::{Expression as AstExpression, TemplateElement};
        use crate::strings::{StringLiteral, StringLiteralType, StringValidator};

        // Validate template literal syntax
        if let Err(err) =
            StringValidator::validate_string_literal(&content, &StringLiteralType::TemplateLiteral)
        {
            return Err(DrafError::parse_error(
                location.line,
                location.column,
                format!("Invalid template literal: {}", err),
            ));
        }

        let string_literal = StringLiteral::new_template(content.clone());

        // Check if this is a simple template literal without interpolation
        if let Some(template_parts) = &string_literal.template_parts {
            if template_parts.expressions.is_empty() {
                // Simple template literal, treat as regular string
                return Ok(AstExpression::Literal {
                    value: LiteralValue::StringLiteral(string_literal),
                    location,
                });
            }

            // Template literal with interpolation
            let mut elements = Vec::new();

            for (i, static_part) in template_parts.static_parts.iter().enumerate() {
                if !static_part.is_empty() {
                    elements.push(TemplateElement::text(static_part.clone()));
                }

                if let Some(expr_str) = template_parts.expressions.get(i) {
                    // Parse the expression string into an AST node by tokenizing and parsing it
                    let expr = self.parse_template_expression(expr_str, &location)?;
                    elements.push(TemplateElement::expression(expr));
                }
            }

            Ok(AstExpression::TemplateLiteral {
                parts: elements,
                location,
            })
        } else {
            // Fallback to regular string literal
            Ok(AstExpression::Literal {
                value: LiteralValue::StringLiteral(string_literal),
                location,
            })
        }
    }

    /// Parse an expression inside a template literal interpolation
    fn parse_template_expression(
        &mut self,
        expr_str: &str,
        location: &SourceLocation,
    ) -> DrafResult<Expression> {
        use crate::lexer::tokenize;

        // Tokenize the expression string
        let tokens = tokenize(expr_str).map_err(|_| {
            DrafError::parse_error(
                location.line,
                location.column,
                format!("Failed to tokenize template expression: {}", expr_str),
            )
        })?;

        // Create a new parser for this expression
        let mut expr_parser = Parser::new(&tokens);

        // Parse the expression
        expr_parser.parse_expression().map_err(|_| {
            DrafError::parse_error(
                location.line,
                location.column,
                format!("Failed to parse template expression: {}", expr_str),
            )
        })
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

    /// Consume optional semicolon or newline for ASI (Automatic Semicolon Insertion)
    fn consume_optional_semicolon(&mut self) {
        if self.check(&TokenKind::Semicolon) || self.check(&TokenKind::Newline) {
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
