//! Date global object implementation
//!
//! Provides the JavaScript Date global object with its static methods and constructor.
//! This includes methods like Date.now(), Date.parse(), and the Date constructor.

use crate::ast::{Expression, LiteralValue};
use crate::globals::{
    create_global_method, create_global_parameter, dummy_location, GlobalMethod, GlobalObject,
};
use crate::types::Type;
use std::collections::HashMap;

/// Create the Date global object with all its methods and properties
pub fn create_date_global() -> GlobalObject {
    let mut static_methods = HashMap::new();
    let mut instance_methods = HashMap::new();
    let static_properties = HashMap::new();
    let instance_properties = HashMap::new();

    // Static methods
    static_methods.insert("now".to_string(), create_date_now_method());
    static_methods.insert("parse".to_string(), create_date_parse_method());
    static_methods.insert("UTC".to_string(), create_date_utc_method());

    // Instance methods (for Date objects)
    instance_methods.insert("getTime".to_string(), create_get_time_method());
    instance_methods.insert("getFullYear".to_string(), create_get_full_year_method());
    instance_methods.insert("getMonth".to_string(), create_get_month_method());
    instance_methods.insert("getDate".to_string(), create_get_date_method());
    instance_methods.insert("getDay".to_string(), create_get_day_method());
    instance_methods.insert("getHours".to_string(), create_get_hours_method());
    instance_methods.insert("getMinutes".to_string(), create_get_minutes_method());
    instance_methods.insert("getSeconds".to_string(), create_get_seconds_method());
    instance_methods.insert(
        "getMilliseconds".to_string(),
        create_get_milliseconds_method(),
    );
    instance_methods.insert("toString".to_string(), create_to_string_method());
    instance_methods.insert("toISOString".to_string(), create_to_iso_string_method());
    instance_methods.insert("valueOf".to_string(), create_value_of_method());

    GlobalObject {
        name: "Date".to_string(),
        constructor_type: Type::Function {
            params: vec![Type::Any], // Date constructor can take various arguments
            return_type: Box::new(Type::Object(HashMap::new())), // Date object
        },
        static_methods,
        instance_methods,
        static_properties,
        instance_properties,
    }
}

/// Date.now()
fn create_date_now_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("now", parameters, Type::Number, true, false)
}

/// Date.parse(dateString)
fn create_date_parse_method() -> GlobalMethod {
    let parameters = vec![create_global_parameter(
        "dateString",
        Type::String,
        false,
        None,
    )];
    create_global_method("parse", parameters, Type::Number, true, false)
}

/// Date.UTC(year, month, day?, hour?, minute?, second?, millisecond?)
fn create_date_utc_method() -> GlobalMethod {
    let parameters = vec![
        create_global_parameter("year", Type::Number, false, None),
        create_global_parameter("month", Type::Number, false, None),
        create_global_parameter(
            "day",
            Type::Number,
            true,
            Some(Expression::Literal {
                value: LiteralValue::Number(1.0),
                location: dummy_location(),
            }),
        ),
        create_global_parameter(
            "hour",
            Type::Number,
            true,
            Some(Expression::Literal {
                value: LiteralValue::Number(0.0),
                location: dummy_location(),
            }),
        ),
        create_global_parameter(
            "minute",
            Type::Number,
            true,
            Some(Expression::Literal {
                value: LiteralValue::Number(0.0),
                location: dummy_location(),
            }),
        ),
        create_global_parameter(
            "second",
            Type::Number,
            true,
            Some(Expression::Literal {
                value: LiteralValue::Number(0.0),
                location: dummy_location(),
            }),
        ),
        create_global_parameter(
            "millisecond",
            Type::Number,
            true,
            Some(Expression::Literal {
                value: LiteralValue::Number(0.0),
                location: dummy_location(),
            }),
        ),
    ];
    create_global_method("UTC", parameters, Type::Number, true, false)
}

/// date.getTime()
fn create_get_time_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("getTime", parameters, Type::Number, false, false)
}

/// date.getFullYear()
fn create_get_full_year_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("getFullYear", parameters, Type::Number, false, false)
}

/// date.getMonth()
fn create_get_month_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("getMonth", parameters, Type::Number, false, false)
}

/// date.getDate()
fn create_get_date_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("getDate", parameters, Type::Number, false, false)
}

/// date.getDay()
fn create_get_day_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("getDay", parameters, Type::Number, false, false)
}

/// date.getHours()
fn create_get_hours_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("getHours", parameters, Type::Number, false, false)
}

/// date.getMinutes()
fn create_get_minutes_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("getMinutes", parameters, Type::Number, false, false)
}

/// date.getSeconds()
fn create_get_seconds_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("getSeconds", parameters, Type::Number, false, false)
}

/// date.getMilliseconds()
fn create_get_milliseconds_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("getMilliseconds", parameters, Type::Number, false, false)
}

/// date.toString()
fn create_to_string_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("toString", parameters, Type::String, false, false)
}

/// date.toISOString()
fn create_to_iso_string_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("toISOString", parameters, Type::String, false, false)
}

/// date.valueOf()
fn create_value_of_method() -> GlobalMethod {
    let parameters = vec![];
    create_global_method("valueOf", parameters, Type::Number, false, false)
}

/// Get current timestamp in milliseconds (simplified implementation)
pub fn get_current_timestamp() -> f64 {
    // For compilation purposes, we'll use a constant timestamp
    // In a real implementation, this would call system time functions
    1640995200000.0 // January 1, 2022 00:00:00 UTC
}

/// Execute a Date static method at compile time (for constant folding)
pub fn execute_date_static_method(method: &str, args: &[f64]) -> Option<f64> {
    match method {
        "now" => Some(get_current_timestamp()),
        "parse" => {
            // Simplified parse - in real implementation would parse date strings
            // For now, return a default timestamp
            Some(get_current_timestamp())
        }
        "UTC" => {
            // Simplified UTC - would normally calculate UTC timestamp from components
            if args.len() >= 2 {
                // Very basic calculation - year and month only
                let year = args[0] as i32;
                let month = args[1] as i32;

                // Simple approximation: days since epoch
                let years_since_1970 = year - 1970;
                let days = years_since_1970 * 365 + month * 30;
                Some(days as f64 * 24.0 * 60.0 * 60.0 * 1000.0)
            } else {
                Some(0.0)
            }
        }
        _ => None,
    }
}

/// Execute a Date instance method (for date objects)
pub fn execute_date_instance_method(method: &str, timestamp: f64) -> Option<f64> {
    match method {
        "getTime" | "valueOf" => Some(timestamp),
        "getFullYear" => {
            // Simplified year calculation
            let days_since_epoch = timestamp / (24.0 * 60.0 * 60.0 * 1000.0);
            Some(1970.0 + (days_since_epoch / 365.0).floor())
        }
        "getMonth" => {
            // Simplified month calculation (0-based)
            let days_since_epoch = timestamp / (24.0 * 60.0 * 60.0 * 1000.0);
            Some(((days_since_epoch % 365.0) / 30.0).floor())
        }
        "getDate" => {
            // Simplified date calculation
            let days_since_epoch = timestamp / (24.0 * 60.0 * 60.0 * 1000.0);
            Some(((days_since_epoch % 30.0) + 1.0).floor())
        }
        "getDay" => {
            // Simplified day of week calculation (0 = Sunday)
            let days_since_epoch = timestamp / (24.0 * 60.0 * 60.0 * 1000.0);
            Some((days_since_epoch % 7.0).floor())
        }
        "getHours" => {
            // Simplified hours calculation
            let hours_since_epoch = timestamp / (60.0 * 60.0 * 1000.0);
            Some((hours_since_epoch % 24.0).floor())
        }
        "getMinutes" => {
            // Simplified minutes calculation
            let minutes_since_epoch = timestamp / (60.0 * 1000.0);
            Some((minutes_since_epoch % 60.0).floor())
        }
        "getSeconds" => {
            // Simplified seconds calculation
            let seconds_since_epoch = timestamp / 1000.0;
            Some((seconds_since_epoch % 60.0).floor())
        }
        "getMilliseconds" => {
            // Simplified milliseconds calculation
            Some(timestamp % 1000.0)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_global_creation() {
        let date_global = create_date_global();

        assert_eq!(date_global.name, "Date");
        assert!(date_global.static_methods.contains_key("now"));
        assert!(date_global.static_methods.contains_key("parse"));
        assert!(date_global.static_methods.contains_key("UTC"));
        assert!(date_global.instance_methods.contains_key("getTime"));
        assert!(date_global.instance_methods.contains_key("toString"));
    }

    #[test]
    fn test_date_static_methods() {
        let now_result = execute_date_static_method("now", &[]);
        assert!(now_result.is_some());
        assert!(now_result.unwrap() > 0.0);

        let utc_result = execute_date_static_method("UTC", &[2022.0, 0.0]);
        assert!(utc_result.is_some());
    }

    #[test]
    fn test_date_instance_methods() {
        let timestamp = 1640995200000.0; // January 1, 2022

        assert_eq!(
            execute_date_instance_method("getTime", timestamp),
            Some(timestamp)
        );
        assert_eq!(
            execute_date_instance_method("valueOf", timestamp),
            Some(timestamp)
        );

        let year = execute_date_instance_method("getFullYear", timestamp);
        assert!(year.is_some());
        assert!(year.unwrap() >= 2022.0);
    }
}
