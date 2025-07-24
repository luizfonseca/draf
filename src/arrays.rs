//! Array support for the Draf TypeScript compiler
//!
//! This module provides comprehensive array functionality including:
//! - Array literal parsing and codegen
//! - Array constructor and global Array object
//! - Array instance methods (push, pop, slice, join, etc.)
//! - Dynamic array management in LLVM
//! - Type checking for array operations

use crate::ast::{Expression, LiteralValue, SourceLocation};
use crate::globals::{create_global_method, create_global_parameter, GlobalMethod, GlobalObject};
use crate::types::Type;
use std::collections::HashMap;

/// Array instance methods available on all arrays
#[derive(Debug, Clone)]
pub struct ArrayInstanceMethod {
    pub name: String,
    pub parameters: Vec<ArrayMethodParameter>,
    pub return_type: Type,
    pub mutates_array: bool, // Whether this method modifies the original array
}

/// Parameter for array instance methods
#[derive(Debug, Clone)]
pub struct ArrayMethodParameter {
    pub name: String,
    pub param_type: Type,
    pub optional: bool,
    pub rest: bool, // For rest parameters like ...items
}

/// Runtime array structure for LLVM codegen
#[derive(Debug, Clone)]
pub struct ArrayRuntime {
    pub element_type: Type,
    pub length: usize,
    pub capacity: usize,
}

/// Create the Array global object with constructor and static methods
pub fn create_array_global() -> GlobalObject {
    let mut static_methods = HashMap::new();
    let mut instance_methods = HashMap::new();
    let static_properties = HashMap::new();
    let instance_properties = HashMap::new();

    // Static methods
    static_methods.insert("isArray".to_string(), create_is_array_method());
    static_methods.insert("from".to_string(), create_from_method());
    static_methods.insert("of".to_string(), create_of_method());

    // Instance methods (these will be added to array instances)
    instance_methods.insert("push".to_string(), create_push_method());
    instance_methods.insert("pop".to_string(), create_pop_method());
    instance_methods.insert("slice".to_string(), create_slice_method());
    instance_methods.insert("join".to_string(), create_join_method());
    instance_methods.insert("concat".to_string(), create_concat_method());
    instance_methods.insert("indexOf".to_string(), create_index_of_method());
    instance_methods.insert("includes".to_string(), create_includes_method());
    instance_methods.insert("reverse".to_string(), create_reverse_method());
    instance_methods.insert("sort".to_string(), create_sort_method());
    instance_methods.insert("shift".to_string(), create_shift_method());
    instance_methods.insert("unshift".to_string(), create_unshift_method());
    instance_methods.insert("splice".to_string(), create_splice_method());

    GlobalObject {
        name: "Array".to_string(),
        constructor_type: Type::Function {
            params: vec![Type::Any], // Array constructor can take various arguments
            return_type: Box::new(Type::Array(Box::new(Type::Any))),
        },
        static_methods,
        instance_methods,
        static_properties,
        instance_properties,
    }
}

/// Array.isArray(value) - checks if value is an array
fn create_is_array_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("value", Type::Any, false, None)];
    create_global_method("isArray", parameters, Type::Boolean, true, false)
}

/// Array.from(arrayLike) - creates array from array-like object
fn create_from_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("arrayLike", Type::Any, false, None)];
    create_global_method(
        "from",
        parameters,
        Type::Array(Box::new(Type::Any)),
        true,
        false,
    )
}

/// Array.of(...items) - creates array from arguments
fn create_of_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("items", Type::Any, false, None)];
    create_global_method(
        "of",
        parameters,
        Type::Array(Box::new(Type::Any)),
        true,
        false,
    )
}

/// arr.push(item1, item2, ...) - adds elements to end of array
fn create_push_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("items", Type::Any, false, None)];
    create_global_method("push", parameters, Type::Number, false, false)
}

/// arr.pop() - removes and returns last element
fn create_pop_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("pop", parameters, Type::Any, false, false)
}

/// arr.slice(start?, end?) - returns shallow copy of portion of array
fn create_slice_method() -> GlobalMethod {
    let parameters = vec![
        create_global_parameter("start", Type::Number, true, None),
        create_global_parameter("end", Type::Number, true, None),
    ];
    create_global_method(
        "slice",
        parameters,
        Type::Array(Box::new(Type::Any)),
        false,
        false,
    )
}

/// arr.join(separator?) - joins array elements into string
fn create_join_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter(
        "separator",
        Type::String,
        true,
        Some(Expression::Literal {
            value: LiteralValue::String(",".to_string()),
            location: SourceLocation::dummy(),
        }),
    )];
    create_global_method("join", parameters, Type::String, false, false)
}

/// arr.concat(other) - returns new array with concatenated elements
fn create_concat_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter(
        "other",
        Type::Array(Box::new(Type::Any)),
        false,
        None,
    )];
    create_global_method(
        "concat",
        parameters,
        Type::Array(Box::new(Type::Any)),
        false,
        false,
    )
}

/// arr.indexOf(searchElement, fromIndex?) - returns first index of element
fn create_index_of_method() -> GlobalMethod {
    let parameters = vec![
        create_global_parameter("searchElement", Type::Any, false, None),
        create_global_parameter("fromIndex", Type::Number, true, None),
    ];
    create_global_method("indexOf", parameters, Type::Number, false, false)
}

/// arr.includes(searchElement, fromIndex?) - checks if array includes element
fn create_includes_method() -> GlobalMethod {
    let parameters = vec![
        create_global_parameter("searchElement", Type::Any, false, None),
        create_global_parameter("fromIndex", Type::Number, true, None),
    ];
    create_global_method("includes", parameters, Type::Boolean, false, false)
}

/// arr.reverse() - reverses array in place
fn create_reverse_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method(
        "reverse",
        parameters,
        Type::Array(Box::new(Type::Any)),
        false,
        false,
    )
}

/// arr.sort(compareFn?) - sorts array in place
fn create_sort_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter(
        "compareFn",
        Type::Function {
            params: vec![Type::Any, Type::Any],
            return_type: Box::new(Type::Number),
        },
        true,
        None,
    )];
    create_global_method(
        "sort",
        parameters,
        Type::Array(Box::new(Type::Any)),
        false,
        false,
    )
}

/// arr.shift() - removes and returns first element
fn create_shift_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("shift", parameters, Type::Any, false, false)
}

/// arr.unshift(item1, item2, ...) - adds elements to beginning of array
fn create_unshift_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("items", Type::Any, false, None)];
    create_global_method("unshift", parameters, Type::Number, false, false)
}

/// arr.splice(start, deleteCount?, item1?, item2?, ...) - changes array by removing/adding elements
fn create_splice_method() -> GlobalMethod {
    let parameters = vec![
        create_global_parameter("start", Type::Number, false, None),
        create_global_parameter("deleteCount", Type::Number, true, None),
        create_global_parameter("items", Type::Any, true, None),
    ];
    create_global_method(
        "splice",
        parameters,
        Type::Array(Box::new(Type::Any)),
        false,
        false,
    )
}

/// Helper functions for array literal parsing
pub mod parsing {
    use super::*;
    use crate::ast::Expression;
    use crate::error::DrafResult;

    /// Parse array literal elements from a vector of expressions
    pub fn parse_array_elements(elements: Vec<Expression>) -> DrafResult<ArrayLiteral> {
        // Infer element type from the first non-null element
        let element_type = if elements.is_empty() {
            Type::Any
        } else {
            infer_array_element_type(&elements)?
        };

        Ok(ArrayLiteral {
            elements,
            element_type,
            location: SourceLocation::dummy(),
        })
    }

    /// Infer the element type from array elements
    fn infer_array_element_type(elements: &[Expression]) -> DrafResult<Type> {
        if elements.is_empty() {
            return Ok(Type::Any);
        }

        // For now, use a simple heuristic: use the type of the first element
        // In a more sophisticated implementation, we would find the common type
        match &elements[0] {
            Expression::Literal { value, .. } => Ok(match value {
                LiteralValue::Number(_) => Type::Number,
                LiteralValue::String(_) | LiteralValue::StringLiteral(_) => Type::String,
                LiteralValue::Boolean(_) => Type::Boolean,
                LiteralValue::Null => Type::Null,
                LiteralValue::Undefined => Type::Undefined,
            }),
            Expression::Array { .. } => Ok(Type::Array(Box::new(Type::Any))),
            _ => Ok(Type::Any),
        }
    }
}

/// Runtime array management for LLVM codegen
pub mod runtime {
    use super::*;

    /// Default initial capacity for dynamic arrays
    pub const DEFAULT_ARRAY_CAPACITY: usize = 8;

    /// Create a new runtime array structure
    pub fn create_array_runtime(element_type: Type, initial_capacity: usize) -> ArrayRuntime {
        ArrayRuntime {
            element_type,
            length: 0,
            capacity: initial_capacity.max(DEFAULT_ARRAY_CAPACITY),
        }
    }

    /// Calculate new capacity when array needs to grow
    pub fn calculate_new_capacity(current_capacity: usize, required_size: usize) -> usize {
        let mut new_capacity = current_capacity;
        while new_capacity < required_size {
            new_capacity = (new_capacity * 3) / 2; // Grow by 1.5x
        }
        new_capacity
    }

    /// Get the LLVM struct layout for array headers
    /// Arrays in memory will have: [length: i64, capacity: i64, data: ptr]
    pub fn get_array_header_size() -> usize {
        16 // 8 bytes for length + 8 bytes for capacity
    }
}

/// Array literal representation
#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
    pub element_type: Type,
    pub location: SourceLocation,
}

impl ArrayLiteral {
    /// Create a new array literal
    pub fn new(elements: Vec<Expression>, element_type: Type, location: SourceLocation) -> Self {
        Self {
            elements,
            element_type,
            location,
        }
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// Array method execution for compile-time evaluation
pub mod execution {
    use super::*;

    /// Execute array static methods at compile time when possible
    pub fn execute_array_static_method(method: &str, args: &[Expression]) -> Option<Expression> {
        match method {
            "isArray" => {
                // Return true for array expressions, false otherwise
                if let Some(arg) = args.first() {
                    match arg {
                        Expression::Array { .. } => Some(Expression::Literal {
                            value: LiteralValue::Boolean(true),
                            location: SourceLocation::dummy(),
                        }),
                        _ => Some(Expression::Literal {
                            value: LiteralValue::Boolean(false),
                            location: SourceLocation::dummy(),
                        }),
                    }
                } else {
                    Some(Expression::Literal {
                        value: LiteralValue::Boolean(false),
                        location: SourceLocation::dummy(),
                    })
                }
            }
            "of" => {
                // Array.of(...args) creates array from arguments
                Some(Expression::Array {
                    elements: args.to_vec(),
                    location: SourceLocation::dummy(),
                })
            }
            _ => None,
        }
    }

    /// Execute array instance methods when possible
    pub fn execute_array_instance_method(
        array: &ArrayLiteral,
        method: &str,
        _args: &[Expression],
    ) -> Option<Expression> {
        match method {
            "join" => {
                // For now, return a placeholder string
                Some(Expression::Literal {
                    value: LiteralValue::String("joined_array".to_string()),
                    location: SourceLocation::dummy(),
                })
            }
            "slice" => {
                // Return a copy of the array for now
                Some(Expression::Array {
                    elements: array.elements.clone(),
                    location: SourceLocation::dummy(),
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_global_creation() {
        let array_global = create_array_global();

        assert_eq!(array_global.name, "Array");
        assert!(array_global.static_methods.contains_key("isArray"));
        assert!(array_global.static_methods.contains_key("from"));
        assert!(array_global.static_methods.contains_key("of"));
        assert!(array_global.instance_methods.contains_key("push"));
        assert!(array_global.instance_methods.contains_key("pop"));
        assert!(array_global.instance_methods.contains_key("slice"));
        assert!(array_global.instance_methods.contains_key("join"));
    }

    #[test]
    fn test_array_runtime_creation() {
        let runtime = runtime::create_array_runtime(Type::Number, 10);

        assert_eq!(runtime.element_type, Type::Number);
        assert_eq!(runtime.length, 0);
        assert_eq!(runtime.capacity, 10);
    }

    #[test]
    fn test_capacity_calculation() {
        assert_eq!(runtime::calculate_new_capacity(8, 9), 12);
        assert_eq!(runtime::calculate_new_capacity(8, 15), 18);
        assert_eq!(runtime::calculate_new_capacity(4, 20), 24);
    }

    #[test]
    fn test_array_literal_creation() {
        let elements = vec![
            Expression::Literal {
                value: LiteralValue::Number(1.0),
                location: SourceLocation::dummy(),
            },
            Expression::Literal {
                value: LiteralValue::Number(2.0),
                location: SourceLocation::dummy(),
            },
        ];

        let array_literal = ArrayLiteral::new(elements, Type::Number, SourceLocation::dummy());

        assert_eq!(array_literal.len(), 2);
        assert!(!array_literal.is_empty());
        assert_eq!(array_literal.element_type, Type::Number);
    }

    #[test]
    fn test_array_static_method_execution() {
        let array_expr = Expression::Array {
            elements: vec![],
            location: SourceLocation::dummy(),
        };

        let result = execution::execute_array_static_method("isArray", &[array_expr]);
        assert!(result.is_some());

        if let Some(Expression::Literal {
            value: LiteralValue::Boolean(true),
            ..
        }) = result
        {
            // Test passed
        } else {
            panic!("Expected boolean true result");
        }
    }
}
