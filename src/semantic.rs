//! Semantic analysis for the strongly typed TypeScript compiler
//!
//! This module performs type checking, symbol resolution, and other semantic
//! analysis on the AST to ensure type safety and correctness.

use crate::ast::*;
use crate::error::{DrafError, DrafResult, ErrorCollector};
use crate::types::{BinaryOp, Type, TypeContext};

/// Semantic analyzer that performs type checking and symbol resolution
pub struct SemanticAnalyzer {
    /// Type context for symbol resolution
    context: TypeContext,
    /// Error collector for gathering multiple errors
    errors: ErrorCollector,
    /// Whether to continue analysis after errors
    continue_on_error: bool,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        Self {
            context: TypeContext::new(),
            errors: ErrorCollector::new(),
            continue_on_error: true,
        }
    }

    /// Create analyzer that stops on first error
    pub fn strict() -> Self {
        Self {
            context: TypeContext::new(),
            errors: ErrorCollector::new(),
            continue_on_error: false,
        }
    }

    /// Analyze a program and return a type-checked AST
    pub fn analyze(&mut self, program: Program) -> DrafResult<TypedProgram> {
        let mut typed_statements = Vec::new();

        for statement in program.statements {
            match self.analyze_statement(statement) {
                Ok(typed_stmt) => typed_statements.push(typed_stmt),
                Err(e) => {
                    self.errors.add_error(e);
                    if !self.continue_on_error {
                        break;
                    }
                }
            }
        }

        // Check if any errors occurred
        if self.errors.has_errors() {
            let errors = std::mem::take(&mut self.errors);
            return Err(errors.into_result().unwrap_err());
        }

        Ok(TypedProgram {
            statements: typed_statements,
            location: program.location,
        })
    }

    /// Analyze a statement
    fn analyze_statement(&mut self, statement: Statement) -> DrafResult<TypedStatement> {
        match statement {
            Statement::VariableDeclaration {
                name,
                type_annotation,
                initializer,
                kind,
                location,
            } => {
                let declared_type = if let Some(type_ann) = &type_annotation {
                    Some(type_ann.to_type())
                } else {
                    None
                };

                let (init_expr, inferred_type) = if let Some(init) = initializer {
                    let typed_init = self.analyze_expression(init)?;
                    let init_type = typed_init.type_info.clone();
                    (Some(typed_init), Some(init_type))
                } else {
                    (None, None)
                };

                // Determine final type
                let final_type = match (declared_type, inferred_type) {
                    (Some(declared), Some(inferred)) => {
                        // Check if initializer type is assignable to declared type
                        if !inferred.is_assignable_to(&declared) {
                            return Err(DrafError::type_error(
                                location.line,
                                location.column,
                                format!(
                                    "Cannot assign {} to variable of type {}",
                                    inferred, declared
                                ),
                            ));
                        }
                        declared
                    }
                    (Some(declared), None) => declared,
                    (None, Some(inferred)) => inferred,
                    (None, None) => {
                        return Err(DrafError::semantic_error(
                            location.line,
                            location.column,
                            "Variable declaration must have either type annotation or initializer",
                        ));
                    }
                };

                // Check const variables have initializers
                if kind == VariableKind::Const && init_expr.is_none() {
                    return Err(DrafError::semantic_error(
                        location.line,
                        location.column,
                        "Const variables must be initialized",
                    ));
                }

                // Add to symbol table
                if kind == VariableKind::Const {
                    self.context
                        .add_const_variable(name.clone(), final_type.clone());
                } else {
                    self.context.add_variable(name.clone(), final_type.clone());
                }

                Ok(TypedStatement::VariableDeclaration {
                    name,
                    type_annotation,
                    initializer: init_expr,
                    kind,
                    inferred_type: final_type,
                    location,
                })
            }

            Statement::ExpressionStatement {
                expression,
                location,
            } => {
                let typed_expr = self.analyze_expression(expression)?;
                Ok(TypedStatement::ExpressionStatement {
                    expression: typed_expr,
                    location,
                })
            }

            Statement::If {
                condition,
                then_branch,
                else_branch,
                location,
            } => {
                let typed_condition = self.analyze_expression(condition)?;

                // Condition should be boolean
                if !matches!(typed_condition.type_info, Type::Boolean) {
                    return Err(DrafError::semantic_error(
                        location.line,
                        location.column,
                        "If condition must be boolean",
                    ));
                }

                let typed_then = Box::new(self.analyze_statement(*then_branch)?);
                let typed_else = if let Some(else_stmt) = else_branch {
                    Some(Box::new(self.analyze_statement(*else_stmt)?))
                } else {
                    None
                };

                Ok(TypedStatement::If {
                    condition: typed_condition,
                    then_branch: typed_then,
                    else_branch: typed_else,
                    location,
                })
            }

            Statement::Block(block) => {
                let mut typed_statements = Vec::new();
                for stmt in block.statements {
                    typed_statements.push(self.analyze_statement(stmt)?);
                }

                Ok(TypedStatement::Block {
                    statements: typed_statements,
                    location: block.location,
                })
            }

            // Placeholder implementations for other statement types
            _ => Err(DrafError::semantic_error(
                statement.location().line,
                statement.location().column,
                "Statement type not yet implemented in semantic analysis",
            )),
        }
    }

    /// Analyze an expression and return typed expression
    fn analyze_expression(&mut self, expression: Expression) -> DrafResult<TypedExpression> {
        match expression {
            Expression::Literal { value, location } => {
                let type_info = match &value {
                    LiteralValue::Number(_) => Type::Number,
                    LiteralValue::String(_) => Type::String,
                    LiteralValue::Boolean(_) => Type::Boolean,
                    LiteralValue::Null => Type::Null,
                    LiteralValue::Undefined => Type::Undefined,
                };

                Ok(TypedExpression {
                    expression: Expression::Literal { value, location },
                    type_info,
                    operand_types: None,
                })
            }

            Expression::Identifier { name, location } => {
                if let Some(var_type) = self.context.get_variable(&name) {
                    Ok(TypedExpression {
                        expression: Expression::Identifier { name, location },
                        type_info: var_type.clone(),
                        operand_types: None,
                    })
                } else {
                    Err(DrafError::semantic_error(
                        location.line,
                        location.column,
                        format!("Undefined variable: {}", name),
                    ))
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
                location,
            } => {
                let typed_left = self.analyze_expression(*left)?;
                let typed_right = self.analyze_expression(*right)?;

                let binary_op = match operator {
                    BinaryOperator::Add => BinaryOp::Add,
                    BinaryOperator::Subtract => BinaryOp::Sub,
                    BinaryOperator::Multiply => BinaryOp::Mul,
                    BinaryOperator::Divide => BinaryOp::Div,
                    BinaryOperator::Modulo => BinaryOp::Mod,
                    BinaryOperator::Equal => BinaryOp::Eq,
                    BinaryOperator::NotEqual => BinaryOp::Ne,
                    BinaryOperator::StrictEqual => BinaryOp::StrictEq,
                    BinaryOperator::StrictNotEqual => BinaryOp::StrictNe,
                    BinaryOperator::LessThan => BinaryOp::Lt,
                    BinaryOperator::LessEqual => BinaryOp::Le,
                    BinaryOperator::GreaterThan => BinaryOp::Gt,
                    BinaryOperator::GreaterEqual => BinaryOp::Ge,
                    BinaryOperator::LogicalAnd => BinaryOp::And,
                    BinaryOperator::LogicalOr => BinaryOp::Or,
                    BinaryOperator::NullishCoalescing => BinaryOp::NullishCoalescing,
                };

                if let Some(result_type) = typed_left
                    .type_info
                    .can_binary_op(&typed_right.type_info, binary_op)
                {
                    Ok(TypedExpression {
                        expression: Expression::Binary {
                            left: Box::new(typed_left.expression),
                            operator,
                            right: Box::new(typed_right.expression),
                            location,
                        },
                        type_info: result_type,
                        operand_types: Some((typed_left.type_info, typed_right.type_info)),
                    })
                } else {
                    Err(DrafError::type_error(
                        location.line,
                        location.column,
                        format!(
                            "Cannot apply operator {} to {} and {}",
                            operator, typed_left.type_info, typed_right.type_info
                        ),
                    ))
                }
            }

            Expression::Unary {
                operator,
                operand,
                location,
            } => {
                let typed_operand = self.analyze_expression(*operand)?;

                let result_type = match operator {
                    UnaryOperator::Minus | UnaryOperator::Plus => {
                        if typed_operand.type_info == Type::Number {
                            Type::Number
                        } else {
                            return Err(DrafError::type_error(
                                location.line,
                                location.column,
                                format!("Cannot apply {} to {}", operator, typed_operand.type_info),
                            ));
                        }
                    }
                    UnaryOperator::LogicalNot => {
                        if typed_operand.type_info == Type::Boolean {
                            Type::Boolean
                        } else {
                            return Err(DrafError::type_error(
                                location.line,
                                location.column,
                                format!("Cannot apply ! to {}", typed_operand.type_info),
                            ));
                        }
                    }
                };

                Ok(TypedExpression {
                    expression: Expression::Unary {
                        operator,
                        operand: Box::new(typed_operand.expression),
                        location,
                    },
                    type_info: result_type,
                    operand_types: None,
                })
            }

            Expression::Assignment {
                target,
                value,
                location,
            } => {
                // Check if trying to assign to a const variable before moving target
                if let Expression::Identifier { name, .. } = target.as_ref() {
                    if self.context.is_const_variable(name) {
                        return Err(DrafError::semantic_error(
                            location.line,
                            location.column,
                            format!("Cannot assign to const variable '{}'", name),
                        ));
                    }
                }

                let typed_target = self.analyze_expression(*target)?;
                let typed_value = self.analyze_expression(*value)?;

                // Check if value type is assignable to target type
                if !typed_value
                    .type_info
                    .is_assignable_to(&typed_target.type_info)
                {
                    return Err(DrafError::type_error(
                        location.line,
                        location.column,
                        format!(
                            "Cannot assign {} to {}",
                            typed_value.type_info, typed_target.type_info
                        ),
                    ));
                }

                Ok(TypedExpression {
                    expression: Expression::Assignment {
                        target: Box::new(typed_target.expression),
                        value: Box::new(typed_value.expression),
                        location,
                    },
                    type_info: typed_target.type_info,
                    operand_types: None,
                })
            }

            Expression::ConsoleCall {
                method,
                arguments,
                location,
            } => {
                // Type check all arguments
                let mut typed_arguments = Vec::new();
                for arg in arguments {
                    let typed_arg = self.analyze_expression(arg)?;
                    typed_arguments.push(typed_arg);
                }

                // Console methods always return void
                Ok(TypedExpression {
                    expression: Expression::ConsoleCall {
                        method: method.clone(),
                        arguments: typed_arguments
                            .into_iter()
                            .map(|ta| ta.expression)
                            .collect(),
                        location: location.clone(),
                    },
                    type_info: Type::Void,
                    operand_types: None,
                })
            }

            Expression::Conditional {
                condition,
                then_expr,
                else_expr,
                location,
            } => {
                let typed_condition = self.analyze_expression(*condition)?;
                let typed_then = self.analyze_expression(*then_expr)?;
                let typed_else = self.analyze_expression(*else_expr)?;

                // Condition should be boolean
                if !matches!(typed_condition.type_info, Type::Boolean) {
                    return Err(DrafError::semantic_error(
                        location.line,
                        location.column,
                        "Ternary condition must be boolean",
                    ));
                }

                // Both branches should have compatible types
                let result_type = if typed_then.type_info == typed_else.type_info {
                    typed_then.type_info.clone()
                } else {
                    // For now, we'll use the then branch type
                    // In a more sophisticated implementation, we'd find a common type
                    typed_then.type_info.clone()
                };

                Ok(TypedExpression {
                    expression: Expression::Conditional {
                        condition: Box::new(typed_condition.expression),
                        then_expr: Box::new(typed_then.expression),
                        else_expr: Box::new(typed_else.expression),
                        location,
                    },
                    type_info: result_type,
                    operand_types: None,
                })
            }

            // Placeholder for other expression types
            _ => Err(DrafError::semantic_error(
                expression.location().line,
                expression.location().column,
                "Expression type not yet implemented in semantic analysis",
            )),
        }
    }
}

/// A typed version of the AST after semantic analysis
#[derive(Debug, Clone)]
pub struct TypedProgram {
    pub statements: Vec<TypedStatement>,
    pub location: SourceLocation,
}

/// A typed statement with additional type information
#[derive(Debug, Clone)]
pub enum TypedStatement {
    VariableDeclaration {
        name: String,
        type_annotation: Option<TypeAnnotation>,
        initializer: Option<TypedExpression>,
        kind: VariableKind,
        inferred_type: Type,
        location: SourceLocation,
    },
    ExpressionStatement {
        expression: TypedExpression,
        location: SourceLocation,
    },
    If {
        condition: TypedExpression,
        then_branch: Box<TypedStatement>,
        else_branch: Option<Box<TypedStatement>>,
        location: SourceLocation,
    },
    Block {
        statements: Vec<TypedStatement>,
        location: SourceLocation,
    },
    // Other statement types will be added as needed
}

/// A typed expression with type information
#[derive(Debug, Clone)]
pub struct TypedExpression {
    pub expression: Expression,
    pub type_info: Type,
    pub operand_types: Option<(Type, Type)>, // For binary operations, store operand types
}

impl TypedExpression {
    pub fn new(expression: Expression, type_info: Type) -> Self {
        Self {
            expression,
            type_info,
            operand_types: None,
        }
    }

    pub fn new_with_operands(
        expression: Expression,
        type_info: Type,
        operand_types: (Type, Type),
    ) -> Self {
        Self {
            expression,
            type_info,
            operand_types: Some(operand_types),
        }
    }
}

/// Analyze an AST and return a typed AST
pub fn analyze(program: Program) -> DrafResult<TypedProgram> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(program)
}

/// Analyze an AST strictly (stop on first error)
pub fn analyze_strict(program: Program) -> DrafResult<TypedProgram> {
    let mut analyzer = SemanticAnalyzer::strict();
    analyzer.analyze(program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;
    use crate::parser;

    #[test]
    fn test_variable_declaration_analysis() {
        let source = "let x: number = 42;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let typed_program = analyze(program).unwrap();

        assert_eq!(typed_program.statements.len(), 1);
        match &typed_program.statements[0] {
            TypedStatement::VariableDeclaration { inferred_type, .. } => {
                assert_eq!(inferred_type, &Type::Number);
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_type_inference() {
        let source = "let x = 42;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let typed_program = analyze(program).unwrap();

        match &typed_program.statements[0] {
            TypedStatement::VariableDeclaration { inferred_type, .. } => {
                assert_eq!(inferred_type, &Type::Number);
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_type_mismatch_error() {
        let source = "let x: string = 42;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let result = analyze(program);

        assert!(result.is_err());
        match result.unwrap_err() {
            DrafError::TypeError { .. } => {} // Expected
            _ => panic!("Expected type error"),
        }
    }

    #[test]
    fn test_binary_expression_analysis() {
        let source = "let x = 1 + 2;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let typed_program = analyze(program).unwrap();

        match &typed_program.statements[0] {
            TypedStatement::VariableDeclaration { inferred_type, .. } => {
                assert_eq!(inferred_type, &Type::Number);
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_invalid_binary_operation() {
        let source = "let x = 1 + \"hello\";";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let result = analyze(program);

        assert!(result.is_err());
    }

    #[test]
    fn test_undefined_variable() {
        let source = "let x = y;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let result = analyze(program);

        assert!(result.is_err());
        match result.unwrap_err() {
            DrafError::SemanticError { .. } => {} // Expected
            _ => panic!("Expected semantic error"),
        }
    }

    #[test]
    fn test_const_without_initializer() {
        let source = "const x: number;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let result = analyze(program);

        assert!(result.is_err());
    }

    #[test]
    fn test_strong_typing_no_implicit_conversion() {
        // In strongly typed mode, this should fail
        let source = "let x: number = true;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let result = analyze(program);

        assert!(result.is_err());
    }
}
