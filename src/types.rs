//! Type system for the strongly typed TypeScript compiler
//!
//! This module defines the type system that enforces strict typing rules,
//! including the distinct handling of any, null, and undefined types.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// The core type representation in Draf's type system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// Primitive types
    Number,
    String,
    Boolean,

    /// Special types with distinct semantics
    Any, // Represents unknown type - requires explicit handling
    Null,      // Explicit null value
    Undefined, // Explicit undefined value
    Void,      // Function return type for no return value

    /// Composite types
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Union(Vec<Type>),
    Object(HashMap<String, Type>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },

    /// Generic/parametric types
    Generic {
        name: String,
        constraints: Vec<Type>,
    },

    /// User-defined types
    Interface {
        name: String,
        fields: HashMap<String, Type>,
    },
    Class {
        name: String,
        fields: HashMap<String, Type>,
        methods: HashMap<String, Type>,
    },

    /// Type aliases
    Alias {
        name: String,
        target: Box<Type>,
    },

    /// Never type (for functions that never return)
    Never,
}

impl Type {
    /// Check if this type is primitive
    pub fn is_primitive(&self) -> bool {
        matches!(self, Type::Number | Type::String | Type::Boolean)
    }

    /// Check if this type is nullable (can be null or undefined)
    pub fn is_nullable(&self) -> bool {
        match self {
            Type::Null | Type::Undefined => true,
            Type::Union(types) => types.iter().any(|t| t.is_nullable()),
            _ => false,
        }
    }

    /// Check if this type can be assigned to another type
    pub fn is_assignable_to(&self, other: &Type) -> bool {
        // Exact match
        if self == other {
            return true;
        }

        match (self, other) {
            // Any can be assigned to anything (but requires explicit handling)
            (Type::Any, _) => true,

            // Nothing can be assigned to Never
            (_, Type::Never) => false,

            // Void can only be assigned to void
            (Type::Void, Type::Void) => true,
            (_, Type::Void) => false,

            // Union type assignability
            (_, Type::Union(target_types)) => target_types.iter().any(|t| self.is_assignable_to(t)),
            (Type::Union(source_types), _) => {
                source_types.iter().all(|t| t.is_assignable_to(other))
            }

            // Array assignability (covariant)
            (Type::Array(source_elem), Type::Array(target_elem)) => {
                source_elem.is_assignable_to(target_elem)
            }

            // Function assignability (contravariant in parameters, covariant in return)
            (
                Type::Function {
                    params: source_params,
                    return_type: source_return,
                },
                Type::Function {
                    params: target_params,
                    return_type: target_return,
                },
            ) => {
                // Same number of parameters
                if source_params.len() != target_params.len() {
                    return false;
                }

                // Parameters are contravariant
                let params_compatible = target_params.iter().zip(source_params.iter()).all(
                    |(target_param, source_param)| target_param.is_assignable_to(source_param),
                );

                // Return type is covariant
                let return_compatible = source_return.is_assignable_to(target_return);

                params_compatible && return_compatible
            }

            // Object structural compatibility
            (Type::Object(source_fields), Type::Object(target_fields)) => {
                // Target must have all fields that source has, with compatible types
                target_fields.iter().all(|(field_name, target_type)| {
                    source_fields
                        .get(field_name)
                        .map(|source_type| source_type.is_assignable_to(target_type))
                        .unwrap_or(false)
                })
            }

            // Interface compatibility
            (
                Type::Object(fields),
                Type::Interface {
                    fields: interface_fields,
                    ..
                },
            ) => interface_fields.iter().all(|(field_name, target_type)| {
                fields
                    .get(field_name)
                    .map(|source_type| source_type.is_assignable_to(target_type))
                    .unwrap_or(false)
            }),

            // Type alias resolution
            (_, Type::Alias { target, .. }) => self.is_assignable_to(target),
            (Type::Alias { target, .. }, _) => target.is_assignable_to(other),

            // Default: no assignment possible
            _ => false,
        }
    }

    /// Check if two types can be used in a binary operation
    pub fn can_binary_op(&self, other: &Type, op: BinaryOp) -> Option<Type> {
        match op {
            BinaryOp::Add => match (self, other) {
                (Type::Number, Type::Number) => Some(Type::Number),
                (Type::String, Type::String) => Some(Type::String),
                _ => None,
            },
            BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => match (self, other) {
                (Type::Number, Type::Number) => Some(Type::Number),
                _ => None,
            },
            BinaryOp::Eq | BinaryOp::Ne => {
                // Equality comparison is allowed between same types
                if self == other {
                    Some(Type::Boolean)
                } else {
                    None
                }
            }
            BinaryOp::StrictEq | BinaryOp::StrictNe => {
                // Strict equality comparison is allowed between same types
                if self == other {
                    Some(Type::Boolean)
                } else {
                    None
                }
            }
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => match (self, other) {
                (Type::Number, Type::Number) => Some(Type::Boolean),
                (Type::String, Type::String) => Some(Type::Boolean),
                _ => None,
            },
            BinaryOp::And | BinaryOp::Or => match (self, other) {
                (Type::Boolean, Type::Boolean) => Some(Type::Boolean),
                _ => None,
            },
            BinaryOp::NullishCoalescing => {
                // Nullish coalescing returns the right operand if left is null/undefined
                // For now, we'll return the type of the right operand
                Some(other.clone())
            }
        }
    }

    /// Get the size of this type in bytes (for LLVM code generation)
    pub fn size_bytes(&self) -> usize {
        match self {
            Type::Number => 8,    // f64
            Type::String => 8,    // pointer to string data
            Type::Boolean => 1,   // i8
            Type::Null => 8,      // pointer-sized
            Type::Undefined => 8, // pointer-sized
            Type::Any => 16,      // tagged union (type tag + value)
            Type::Void => 0,
            Type::Array(_) => 16,       // pointer + length
            Type::Object(_) => 8,       // pointer to object data
            Type::Function { .. } => 8, // function pointer
            Type::Never => 0,
            _ => 8, // Default pointer size for complex types
        }
    }

    /// Create a union type, automatically flattening nested unions
    pub fn union(types: Vec<Type>) -> Type {
        let mut flattened = Vec::new();

        for ty in types {
            match ty {
                Type::Union(nested) => flattened.extend(nested),
                _ => flattened.push(ty),
            }
        }

        // Remove duplicates and sort for canonical representation
        flattened.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
        flattened.dedup();

        match flattened.len() {
            0 => Type::Never,
            1 => flattened.into_iter().next().unwrap(),
            _ => Type::Union(flattened),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Number => write!(f, "number"),
            Type::String => write!(f, "string"),
            Type::Boolean => write!(f, "boolean"),
            Type::Any => write!(f, "any"),
            Type::Null => write!(f, "null"),
            Type::Undefined => write!(f, "undefined"),
            Type::Void => write!(f, "void"),
            Type::Never => write!(f, "never"),
            Type::Array(elem_type) => write!(f, "{}[]", elem_type),
            Type::Tuple(types) => {
                write!(f, "[")?;
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", ty)?;
                }
                write!(f, "]")
            }
            Type::Union(types) => {
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", ty)?;
                }
                Ok(())
            }
            Type::Object(fields) => {
                write!(f, "{{ ")?;
                for (i, (name, ty)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", name, ty)?;
                }
                write!(f, " }}")
            }
            Type::Function {
                params,
                return_type,
            } => {
                write!(f, "(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") => {}", return_type)
            }
            Type::Generic { name, constraints } => {
                write!(f, "{}", name)?;
                if !constraints.is_empty() {
                    write!(f, " extends ")?;
                    for (i, constraint) in constraints.iter().enumerate() {
                        if i > 0 {
                            write!(f, " & ")?;
                        }
                        write!(f, "{}", constraint)?;
                    }
                }
                Ok(())
            }
            Type::Interface { name, .. } => write!(f, "{}", name),
            Type::Class { name, .. } => write!(f, "{}", name),
            Type::Alias { name, .. } => write!(f, "{}", name),
        }
    }
}

/// Binary operators supported in the type system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Comparison
    Eq,
    Ne,
    StrictEq,
    StrictNe,
    Lt,
    Le,
    Gt,
    Ge,

    // Logical
    And,
    Or,
    NullishCoalescing,
}

/// Unary operators supported in the type system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,   // !expr
    Minus, // -expr
    Plus,  // +expr
}

/// Type context for type checking and inference
#[derive(Debug, Clone)]
pub struct TypeContext {
    /// Variable bindings
    variables: HashMap<String, Type>,
    /// Function bindings
    functions: HashMap<String, Type>,
    /// Type aliases
    aliases: HashMap<String, Type>,
    /// Generic type parameters
    generics: HashMap<String, Type>,
    /// Const variable tracking
    const_variables: HashSet<String>,
}

impl TypeContext {
    /// Create a new empty type context
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            aliases: HashMap::new(),
            generics: HashMap::new(),
            const_variables: HashSet::new(),
        }
    }

    /// Add a variable binding
    pub fn add_variable(&mut self, name: String, ty: Type) {
        self.variables.insert(name, ty);
    }

    /// Add a variable binding and mark it as const
    pub fn add_const_variable(&mut self, name: String, ty: Type) {
        self.variables.insert(name.clone(), ty);
        self.const_variables.insert(name);
    }

    /// Check if a variable is const
    pub fn is_const_variable(&self, name: &str) -> bool {
        self.const_variables.contains(name)
    }

    /// Get a variable's type
    pub fn get_variable(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }

    /// Add a function binding
    pub fn add_function(&mut self, name: String, ty: Type) {
        self.functions.insert(name, ty);
    }

    /// Get a function's type
    pub fn get_function(&self, name: &str) -> Option<&Type> {
        self.functions.get(name)
    }

    /// Add a type alias
    pub fn add_alias(&mut self, name: String, ty: Type) {
        self.aliases.insert(name, ty);
    }

    /// Get a type alias
    pub fn get_alias(&self, name: &str) -> Option<&Type> {
        self.aliases.get(name)
    }

    /// Create a child context (for scopes)
    pub fn child(&self) -> Self {
        self.clone()
    }
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_types() {
        assert!(Type::Number.is_primitive());
        assert!(Type::String.is_primitive());
        assert!(Type::Boolean.is_primitive());
        assert!(!Type::Any.is_primitive());
        assert!(!Type::Null.is_primitive());
    }

    #[test]
    fn test_nullable_types() {
        assert!(Type::Null.is_nullable());
        assert!(Type::Undefined.is_nullable());
        assert!(!Type::Number.is_nullable());

        let union = Type::union(vec![Type::Number, Type::Null]);
        assert!(union.is_nullable());
    }

    #[test]
    fn test_type_assignability() {
        // Same types are assignable
        assert!(Type::Number.is_assignable_to(&Type::Number));

        // Any is assignable to anything
        assert!(Type::Any.is_assignable_to(&Type::Number));
        assert!(Type::Any.is_assignable_to(&Type::String));

        // Nothing is assignable to Never
        assert!(!Type::Number.is_assignable_to(&Type::Never));

        // Union assignability
        let union = Type::union(vec![Type::Number, Type::String]);
        assert!(Type::Number.is_assignable_to(&union));
        assert!(Type::String.is_assignable_to(&union));
        assert!(!Type::Boolean.is_assignable_to(&union));
    }

    #[test]
    fn test_binary_operations() {
        // Number arithmetic
        assert_eq!(
            Type::Number.can_binary_op(&Type::Number, BinaryOp::Add),
            Some(Type::Number)
        );

        // String concatenation
        assert_eq!(
            Type::String.can_binary_op(&Type::String, BinaryOp::Add),
            Some(Type::String)
        );

        // Invalid operations
        assert_eq!(
            Type::Number.can_binary_op(&Type::String, BinaryOp::Add),
            None
        );

        // Comparisons
        assert_eq!(
            Type::Number.can_binary_op(&Type::Number, BinaryOp::Lt),
            Some(Type::Boolean)
        );
    }

    #[test]
    fn test_union_creation() {
        let union = Type::union(vec![Type::Number, Type::String]);
        match union {
            Type::Union(types) => {
                assert_eq!(types.len(), 2);
                assert!(types.contains(&Type::Number));
                assert!(types.contains(&Type::String));
            }
            _ => panic!("Expected union type"),
        }

        // Single type should not create union
        let single = Type::union(vec![Type::Number]);
        assert_eq!(single, Type::Number);

        // Empty union should be Never
        let empty = Type::union(vec![]);
        assert_eq!(empty, Type::Never);
    }

    #[test]
    fn test_type_context() {
        let mut ctx = TypeContext::new();

        ctx.add_variable("x".to_string(), Type::Number);
        ctx.add_variable("y".to_string(), Type::String);

        assert_eq!(ctx.get_variable("x"), Some(&Type::Number));
        assert_eq!(ctx.get_variable("y"), Some(&Type::String));
        assert_eq!(ctx.get_variable("z"), None);
    }
}
