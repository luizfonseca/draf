//! Abstract Syntax Tree (AST) definitions for the strongly typed TypeScript compiler
//!
//! This module defines the AST nodes that represent the parsed structure of
//! TypeScript source code with strong typing semantics.

use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Source location information for AST nodes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub end: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, start: usize, end: usize) -> Self {
        Self {
            line,
            column,
            start,
            end,
        }
    }

    /// Create a dummy location for testing
    pub fn dummy() -> Self {
        Self {
            line: 0,
            column: 0,
            start: 0,
            end: 0,
        }
    }
}

/// The root of the AST - represents a complete program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub location: SourceLocation,
}

impl Program {
    pub fn new(statements: Vec<Statement>, location: SourceLocation) -> Self {
        Self {
            statements,
            location,
        }
    }
}

/// Top-level statements in a program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// Variable declaration: let x: number = 42;
    VariableDeclaration {
        name: String,
        type_annotation: Option<TypeAnnotation>,
        initializer: Option<Expression>,
        kind: VariableKind,
        location: SourceLocation,
    },

    /// Function declaration: function add(a: number, b: number): number { ... }
    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<TypeAnnotation>,
        body: Block,
        location: SourceLocation,
    },

    /// Interface declaration: interface Person { name: string; age: number; }
    InterfaceDeclaration {
        name: String,
        fields: Vec<InterfaceField>,
        location: SourceLocation,
    },

    /// Type alias: type UserId = number;
    TypeAlias {
        name: String,
        type_annotation: TypeAnnotation,
        location: SourceLocation,
    },

    /// Expression statement: x + y;
    ExpressionStatement {
        expression: Expression,
        location: SourceLocation,
    },

    /// Block statement: { ... }
    Block(Block),

    /// If statement: if (condition) { ... } else { ... }
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
        location: SourceLocation,
    },

    /// While loop: while (condition) { ... }
    While {
        condition: Expression,
        body: Box<Statement>,
        location: SourceLocation,
    },

    /// For loop: for (init; condition; update) { ... }
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Box<Statement>,
        location: SourceLocation,
    },

    /// Return statement: return expression;
    Return {
        value: Option<Expression>,
        location: SourceLocation,
    },

    /// Empty statement: ;
    Empty { location: SourceLocation },
}

impl Statement {
    pub fn location(&self) -> &SourceLocation {
        match self {
            Statement::VariableDeclaration { location, .. } => location,
            Statement::FunctionDeclaration { location, .. } => location,
            Statement::InterfaceDeclaration { location, .. } => location,
            Statement::TypeAlias { location, .. } => location,
            Statement::ExpressionStatement { location, .. } => location,
            Statement::Block(block) => &block.location,
            Statement::If { location, .. } => location,
            Statement::While { location, .. } => location,
            Statement::For { location, .. } => location,
            Statement::Return { location, .. } => location,
            Statement::Empty { location } => location,
        }
    }
}

/// Block of statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub location: SourceLocation,
}

impl Block {
    pub fn new(statements: Vec<Statement>, location: SourceLocation) -> Self {
        Self {
            statements,
            location,
        }
    }
}

/// Variable declaration kinds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariableKind {
    Let,
    Const,
    Var,
}

impl fmt::Display for VariableKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableKind::Let => write!(f, "let"),
            VariableKind::Const => write!(f, "const"),
            VariableKind::Var => write!(f, "var"),
        }
    }
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
    pub location: SourceLocation,
}

impl Parameter {
    pub fn new(
        name: String,
        type_annotation: Option<TypeAnnotation>,
        optional: bool,
        location: SourceLocation,
    ) -> Self {
        Self {
            name,
            type_annotation,
            optional,
            location,
        }
    }
}

/// Interface field
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceField {
    pub name: String,
    pub type_annotation: TypeAnnotation,
    pub optional: bool,
    pub location: SourceLocation,
}

impl InterfaceField {
    pub fn new(
        name: String,
        type_annotation: TypeAnnotation,
        optional: bool,
        location: SourceLocation,
    ) -> Self {
        Self {
            name,
            type_annotation,
            optional,
            location,
        }
    }
}

/// Expressions in the language
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Literal values
    Literal {
        value: LiteralValue,
        location: SourceLocation,
    },

    /// Variable reference: x
    Identifier {
        name: String,
        location: SourceLocation,
    },

    /// Binary operations: a + b, a == b, etc.
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
        location: SourceLocation,
    },

    /// Unary operations: !x, -x, +x
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
        location: SourceLocation,
    },

    /// Assignment: x = value
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
        location: SourceLocation,
    },

    /// Function call: func(arg1, arg2)
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        location: SourceLocation,
    },

    /// Console method call: console.log(arg1, arg2)
    ConsoleCall {
        method: ConsoleMethod,
        arguments: Vec<Expression>,
        location: SourceLocation,
    },

    /// Member access: obj.property
    MemberAccess {
        object: Box<Expression>,
        property: String,
        location: SourceLocation,
    },

    /// Array access: arr[index]
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
        location: SourceLocation,
    },

    /// Array literal: [1, 2, 3]
    Array {
        elements: Vec<Expression>,
        location: SourceLocation,
    },

    /// Object literal: { a: 1, b: 2 }
    Object {
        fields: Vec<ObjectField>,
        location: SourceLocation,
    },

    /// Type cast: value as Type
    TypeCast {
        expression: Box<Expression>,
        target_type: TypeAnnotation,
        location: SourceLocation,
    },

    /// Conditional expression: condition ? then_expr : else_expr
    Conditional {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
        location: SourceLocation,
    },
}

impl Expression {
    pub fn location(&self) -> &SourceLocation {
        match self {
            Expression::Literal { location, .. } => location,
            Expression::Identifier { location, .. } => location,
            Expression::Binary { location, .. } => location,
            Expression::Unary { location, .. } => location,
            Expression::Assignment { location, .. } => location,
            Expression::Call { location, .. } => location,
            Expression::MemberAccess { location, .. } => location,
            Expression::ArrayAccess { location, .. } => location,
            Expression::Array { location, .. } => location,
            Expression::Object { location, .. } => location,
            Expression::TypeCast { location, .. } => location,
            Expression::Conditional { location, .. } => location,
            Expression::ConsoleCall { location, .. } => location,
        }
    }
}

/// Console methods supported by the compiler
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsoleMethod {
    Log,   // console.log() -> stdout
    Info,  // console.info() -> stdout
    Warn,  // console.warn() -> stderr
    Error, // console.error() -> stderr
    Debug, // console.debug() -> stdout (when debug enabled)
}

impl fmt::Display for ConsoleMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_name = match self {
            ConsoleMethod::Log => "log",
            ConsoleMethod::Info => "info",
            ConsoleMethod::Warn => "warn",
            ConsoleMethod::Error => "error",
            ConsoleMethod::Debug => "debug",
        };
        write!(f, "{}", method_name)
    }
}

/// Object field in object literals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectField {
    pub key: String,
    pub value: Expression,
    pub location: SourceLocation,
}

impl ObjectField {
    pub fn new(key: String, value: Expression, location: SourceLocation) -> Self {
        Self {
            key,
            value,
            location,
        }
    }
}

/// Literal values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Null => write!(f, "null"),
            LiteralValue::Undefined => write!(f, "undefined"),
        }
    }
}

/// Binary operators
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,

    // Logical
    LogicalAnd,
    LogicalOr,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterEqual => ">=",
            BinaryOperator::LogicalAnd => "&&",
            BinaryOperator::LogicalOr => "||",
        };
        write!(f, "{}", symbol)
    }
}

/// Unary operators
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Minus,      // -x
    Plus,       // +x
    LogicalNot, // !x
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            UnaryOperator::Minus => "-",
            UnaryOperator::Plus => "+",
            UnaryOperator::LogicalNot => "!",
        };
        write!(f, "{}", symbol)
    }
}

/// Type annotations in the AST
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeAnnotation {
    /// Primitive types
    Number {
        location: SourceLocation,
    },
    String {
        location: SourceLocation,
    },
    Boolean {
        location: SourceLocation,
    },
    Any {
        location: SourceLocation,
    },
    Null {
        location: SourceLocation,
    },
    Undefined {
        location: SourceLocation,
    },
    Void {
        location: SourceLocation,
    },
    Never {
        location: SourceLocation,
    },

    /// Array type: number[]
    Array {
        element_type: Box<TypeAnnotation>,
        location: SourceLocation,
    },

    /// Tuple type: [number, string]
    Tuple {
        elements: Vec<TypeAnnotation>,
        location: SourceLocation,
    },

    /// Union type: number | string
    Union {
        types: Vec<TypeAnnotation>,
        location: SourceLocation,
    },

    /// Object type: { a: number, b: string }
    Object {
        fields: Vec<TypeAnnotationField>,
        location: SourceLocation,
    },

    /// Function type: (a: number, b: string) => number
    Function {
        parameters: Vec<TypeAnnotation>,
        return_type: Box<TypeAnnotation>,
        location: SourceLocation,
    },

    /// Named type reference: SomeInterface
    Named {
        name: String,
        location: SourceLocation,
    },

    /// Generic type: Array<T>
    Generic {
        name: String,
        type_arguments: Vec<TypeAnnotation>,
        location: SourceLocation,
    },
}

impl TypeAnnotation {
    pub fn location(&self) -> &SourceLocation {
        match self {
            TypeAnnotation::Number { location } => location,
            TypeAnnotation::String { location } => location,
            TypeAnnotation::Boolean { location } => location,
            TypeAnnotation::Any { location } => location,
            TypeAnnotation::Null { location } => location,
            TypeAnnotation::Undefined { location } => location,
            TypeAnnotation::Void { location } => location,
            TypeAnnotation::Never { location } => location,
            TypeAnnotation::Array { location, .. } => location,
            TypeAnnotation::Tuple { location, .. } => location,
            TypeAnnotation::Union { location, .. } => location,
            TypeAnnotation::Object { location, .. } => location,
            TypeAnnotation::Function { location, .. } => location,
            TypeAnnotation::Named { location, .. } => location,
            TypeAnnotation::Generic { location, .. } => location,
        }
    }

    /// Convert type annotation to a Type (used in semantic analysis)
    pub fn to_type(&self) -> Type {
        match self {
            TypeAnnotation::Number { .. } => Type::Number,
            TypeAnnotation::String { .. } => Type::String,
            TypeAnnotation::Boolean { .. } => Type::Boolean,
            TypeAnnotation::Any { .. } => Type::Any,
            TypeAnnotation::Null { .. } => Type::Null,
            TypeAnnotation::Undefined { .. } => Type::Undefined,
            TypeAnnotation::Void { .. } => Type::Void,
            TypeAnnotation::Never { .. } => Type::Never,
            TypeAnnotation::Array { element_type, .. } => {
                Type::Array(Box::new(element_type.to_type()))
            }
            TypeAnnotation::Tuple { elements, .. } => {
                Type::Tuple(elements.iter().map(|e| e.to_type()).collect())
            }
            TypeAnnotation::Union { types, .. } => {
                Type::Union(types.iter().map(|t| t.to_type()).collect())
            }
            TypeAnnotation::Object { fields, .. } => {
                let mut field_map = HashMap::new();
                for field in fields {
                    field_map.insert(field.name.clone(), field.type_annotation.to_type());
                }
                Type::Object(field_map)
            }
            TypeAnnotation::Function {
                parameters,
                return_type,
                ..
            } => Type::Function {
                params: parameters.iter().map(|p| p.to_type()).collect(),
                return_type: Box::new(return_type.to_type()),
            },
            TypeAnnotation::Named { name, .. } => {
                // This will need to be resolved during semantic analysis
                Type::Alias {
                    name: name.clone(),
                    target: Box::new(Type::Any), // Placeholder
                }
            }
            TypeAnnotation::Generic {
                name,
                type_arguments,
                ..
            } => {
                // Handle built-in generics
                if name == "Array" && type_arguments.len() == 1 {
                    Type::Array(Box::new(type_arguments[0].to_type()))
                } else {
                    Type::Generic {
                        name: name.clone(),
                        constraints: type_arguments.iter().map(|t| t.to_type()).collect(),
                    }
                }
            }
        }
    }
}

/// Field in an object type annotation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAnnotationField {
    pub name: String,
    pub type_annotation: TypeAnnotation,
    pub optional: bool,
    pub location: SourceLocation,
}

impl TypeAnnotationField {
    pub fn new(
        name: String,
        type_annotation: TypeAnnotation,
        optional: bool,
        location: SourceLocation,
    ) -> Self {
        Self {
            name,
            type_annotation,
            optional,
            location,
        }
    }
}

/// Visitor trait for traversing the AST
pub trait Visitor<T> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_statement(&mut self, statement: &Statement) -> T;
    fn visit_expression(&mut self, expression: &Expression) -> T;
    fn visit_type_annotation(&mut self, type_annotation: &TypeAnnotation) -> T;
}

/// Mutable visitor trait for transforming the AST
pub trait VisitorMut<T> {
    fn visit_program(&mut self, program: &mut Program) -> T;
    fn visit_statement(&mut self, statement: &mut Statement) -> T;
    fn visit_expression(&mut self, expression: &mut Expression) -> T;
    fn visit_type_annotation(&mut self, type_annotation: &mut TypeAnnotation) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_values() {
        let num = LiteralValue::Number(42.0);
        assert_eq!(num.to_string(), "42");

        let str_val = LiteralValue::String("hello".to_string());
        assert_eq!(str_val.to_string(), "\"hello\"");

        let bool_val = LiteralValue::Boolean(true);
        assert_eq!(bool_val.to_string(), "true");

        let null_val = LiteralValue::Null;
        assert_eq!(null_val.to_string(), "null");

        let undefined_val = LiteralValue::Undefined;
        assert_eq!(undefined_val.to_string(), "undefined");
    }

    #[test]
    fn test_binary_operators() {
        assert_eq!(BinaryOperator::Add.to_string(), "+");
        assert_eq!(BinaryOperator::Equal.to_string(), "==");
        assert_eq!(BinaryOperator::LogicalAnd.to_string(), "&&");
    }

    #[test]
    fn test_unary_operators() {
        assert_eq!(UnaryOperator::Minus.to_string(), "-");
        assert_eq!(UnaryOperator::LogicalNot.to_string(), "!");
    }

    #[test]
    fn test_variable_kinds() {
        assert_eq!(VariableKind::Let.to_string(), "let");
        assert_eq!(VariableKind::Const.to_string(), "const");
        assert_eq!(VariableKind::Var.to_string(), "var");
    }

    #[test]
    fn test_type_annotation_conversion() {
        let type_ann = TypeAnnotation::Number {
            location: SourceLocation::dummy(),
        };
        assert_eq!(type_ann.to_type(), Type::Number);

        let array_type = TypeAnnotation::Array {
            element_type: Box::new(TypeAnnotation::String {
                location: SourceLocation::dummy(),
            }),
            location: SourceLocation::dummy(),
        };
        assert_eq!(array_type.to_type(), Type::Array(Box::new(Type::String)));

        let union_type = TypeAnnotation::Union {
            types: vec![
                TypeAnnotation::Number {
                    location: SourceLocation::dummy(),
                },
                TypeAnnotation::String {
                    location: SourceLocation::dummy(),
                },
            ],
            location: SourceLocation::dummy(),
        };
        assert_eq!(
            union_type.to_type(),
            Type::Union(vec![Type::Number, Type::String])
        );
    }

    #[test]
    fn test_ast_node_locations() {
        let location = SourceLocation::new(1, 1, 0, 10);

        let expr = Expression::Literal {
            value: LiteralValue::Number(42.0),
            location: location.clone(),
        };

        assert_eq!(expr.location(), &location);

        let stmt = Statement::ExpressionStatement {
            expression: expr,
            location: location.clone(),
        };

        assert_eq!(stmt.location(), &location);
    }

    #[test]
    fn test_console_method_display() {
        assert_eq!(ConsoleMethod::Log.to_string(), "log");
        assert_eq!(ConsoleMethod::Info.to_string(), "info");
        assert_eq!(ConsoleMethod::Warn.to_string(), "warn");
        assert_eq!(ConsoleMethod::Error.to_string(), "error");
        assert_eq!(ConsoleMethod::Debug.to_string(), "debug");
    }

    #[test]
    fn test_program_creation() {
        let location = SourceLocation::dummy();
        let statements = vec![Statement::Empty {
            location: location.clone(),
        }];

        let program = Program::new(statements.clone(), location.clone());

        assert_eq!(program.statements, statements);
        assert_eq!(program.location, location);
    }
}
