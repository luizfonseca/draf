//! Number global object implementation
//!
//! Provides the JavaScript Number global object with its static methods and properties.
//! This includes methods like Number.parseInt(), Number.parseFloat(), Number.isNaN(), etc.

use crate::ast::{Expression, LiteralValue};
use crate::globals::{
    create_global_method, create_global_parameter, dummy_location, GlobalMethod, GlobalObject,
};
use crate::types::Type;
use std::collections::HashMap;

/// Create the Number global object with all its methods and properties
pub fn create_number_global() -> GlobalObject {
    let mut static_methods = HashMap::new();
    let mut static_properties = HashMap::new();
    let instance_methods = HashMap::new(); // Number instances don't have methods in our impl
    let instance_properties = HashMap::new();

    // Static methods
    static_methods.insert("parseInt".to_string(), create_parse_int_method());
    static_methods.insert("parseFloat".to_string(), create_parse_float_method());
    static_methods.insert("isNaN".to_string(), create_is_nan_method());
    static_methods.insert("isFinite".to_string(), create_is_finite_method());
    static_methods.insert("isInteger".to_string(), create_is_integer_method());
    static_methods.insert("isSafeInteger".to_string(), create_is_safe_integer_method());

    // Static properties
    static_properties.insert("MAX_VALUE".to_string(), Type::Number);
    static_properties.insert("MIN_VALUE".to_string(), Type::Number);
    static_properties.insert("MAX_SAFE_INTEGER".to_string(), Type::Number);
    static_properties.insert("MIN_SAFE_INTEGER".to_string(), Type::Number);
    static_properties.insert("POSITIVE_INFINITY".to_string(), Type::Number);
    static_properties.insert("NEGATIVE_INFINITY".to_string(), Type::Number);
    static_properties.insert("NaN".to_string(), Type::Number);
    static_properties.insert("EPSILON".to_string(), Type::Number);

    GlobalObject {
        name: "Number".to_string(),
        constructor_type: Type::Function {
            params: vec![Type::Any], // Number constructor can take any value
            return_type: Box::new(Type::Number),
        },
        static_methods,
        instance_methods,
        static_properties,
        instance_properties,
    }
}

/// Number.parseInt(string, radix?)
fn create_parse_int_method() -> GlobalMethod {
    let parameters = vec![
        create_global_parameter("string", Type::String, false, None),
        create_global_parameter(
            "radix",
            Type::Number,
            true,
            Some(Expression::Literal {
                value: LiteralValue::Number(10.0),
                location: dummy_location(),
            }),
        ),
    ];

    create_global_method("parseInt", parameters, Type::Number, true, false)
}

/// Number.parseFloat(string)
fn create_parse_float_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("string", Type::String, false, None)];

    create_global_method("parseFloat", parameters, Type::Number, true, false)
}

/// Number.isNaN(value)
fn create_is_nan_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("value", Type::Any, false, None)];

    create_global_method("isNaN", parameters, Type::Boolean, true, false)
}

/// Number.isFinite(value)
fn create_is_finite_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("value", Type::Any, false, None)];

    create_global_method("isFinite", parameters, Type::Boolean, true, false)
}

/// Number.isInteger(value)
fn create_is_integer_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("value", Type::Any, false, None)];

    create_global_method("isInteger", parameters, Type::Boolean, true, false)
}

/// Number.isSafeInteger(value)
fn create_is_safe_integer_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter("value", Type::Any, false, None)];

    create_global_method("isSafeInteger", parameters, Type::Boolean, true, false)
}

/// Get the value of a Number static property
pub fn get_number_property_value(property: &str) -> Option<f64> {
    match property {
        "MAX_VALUE" => Some(f64::MAX),
        "MIN_VALUE" => Some(f64::MIN_POSITIVE),
        "MAX_SAFE_INTEGER" => Some(9007199254740991.0), // 2^53 - 1
        "MIN_SAFE_INTEGER" => Some(-9007199254740991.0), // -(2^53 - 1)
        "POSITIVE_INFINITY" => Some(f64::INFINITY),
        "NEGATIVE_INFINITY" => Some(f64::NEG_INFINITY),
        "NaN" => Some(f64::NAN),
        "EPSILON" => Some(f64::EPSILON),
        _ => None,
    }
}

/// Execute a Number static method at compile time (for constant folding)
pub fn execute_number_method(method: &str, args: &[f64]) -> Option<f64> {
    match method {
        "parseInt" => {
            if args.is_empty() {
                return Some(f64::NAN);
            }
            // Simplified parseInt - just truncate the number
            Some(args[0].trunc())
        }
        "parseFloat" => {
            if args.is_empty() {
                return Some(f64::NAN);
            }
            Some(args[0])
        }
        "isNaN" => {
            if args.is_empty() {
                return Some(0.0); // false
            }
            Some(if args[0].is_nan() { 1.0 } else { 0.0 })
        }
        "isFinite" => {
            if args.is_empty() {
                return Some(0.0); // false
            }
            Some(if args[0].is_finite() { 1.0 } else { 0.0 })
        }
        "isInteger" => {
            if args.is_empty() {
                return Some(0.0); // false
            }
            Some(if args[0].fract() == 0.0 && args[0].is_finite() {
                1.0
            } else {
                0.0
            })
        }
        "isSafeInteger" => {
            if args.is_empty() {
                return Some(0.0); // false
            }
            let safe_max = 9007199254740991.0;
            let safe_min = -9007199254740991.0;
            Some(
                if args[0].fract() == 0.0
                    && args[0].is_finite()
                    && args[0] >= safe_min
                    && args[0] <= safe_max
                {
                    1.0
                } else {
                    0.0
                },
            )
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_global_creation() {
        let number_global = create_number_global();

        assert_eq!(number_global.name, "Number");
        assert!(number_global.static_methods.contains_key("parseInt"));
        assert!(number_global.static_methods.contains_key("parseFloat"));
        assert!(number_global.static_methods.contains_key("isNaN"));
        assert!(number_global.static_properties.contains_key("MAX_VALUE"));
        assert!(number_global.static_properties.contains_key("NaN"));
    }

    #[test]
    fn test_number_property_values() {
        assert_eq!(
            get_number_property_value("MAX_SAFE_INTEGER"),
            Some(9007199254740991.0)
        );
        assert_eq!(
            get_number_property_value("MIN_SAFE_INTEGER"),
            Some(-9007199254740991.0)
        );
        assert!(get_number_property_value("NaN").unwrap().is_nan());
        assert_eq!(
            get_number_property_value("POSITIVE_INFINITY"),
            Some(f64::INFINITY)
        );
    }

    #[test]
    fn test_number_method_execution() {
        assert_eq!(execute_number_method("parseInt", &[3.14]), Some(3.0));
        assert_eq!(execute_number_method("parseFloat", &[3.14]), Some(3.14));
        assert_eq!(execute_number_method("isNaN", &[f64::NAN]), Some(1.0));
        assert_eq!(execute_number_method("isNaN", &[42.0]), Some(0.0));
        assert_eq!(execute_number_method("isFinite", &[42.0]), Some(1.0));
        assert_eq!(
            execute_number_method("isFinite", &[f64::INFINITY]),
            Some(0.0)
        );
    }
}
