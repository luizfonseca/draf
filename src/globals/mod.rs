//! Global objects and built-in functions for the Draf TypeScript compiler
//!
//! This module provides implementations for JavaScript's global objects and functions,
//! optimized for compilation to native code via LLVM. Each global is implemented
//! in its own submodule for maintainability and extensibility.

pub mod date;
pub mod number;

use crate::arrays;

use crate::ast::{Expression, SourceLocation};

use crate::types::Type;
use std::collections::HashMap;

/// Represents a global object with its methods and properties
#[derive(Debug, Clone)]
pub struct GlobalObject {
    pub name: String,
    pub constructor_type: Type,
    pub static_methods: HashMap<String, GlobalMethod>,
    pub instance_methods: HashMap<String, GlobalMethod>,
    pub static_properties: HashMap<String, Type>,
    pub instance_properties: HashMap<String, Type>,
}

/// Represents a method on a global object
#[derive(Debug, Clone)]
pub struct GlobalMethod {
    pub name: String,
    pub parameters: Vec<GlobalParameter>,
    pub return_type: Type,
    pub is_static: bool,
    pub is_constructor: bool,
}

/// Represents a parameter for a global method
#[derive(Debug, Clone)]
pub struct GlobalParameter {
    pub name: String,
    pub param_type: Type,
    pub optional: bool,
    pub default_value: Option<Expression>,
}

/// Registry for all global objects and their methods
#[derive(Debug, Default)]
pub struct GlobalRegistry {
    objects: HashMap<String, GlobalObject>,
}

impl GlobalRegistry {
    /// Create a new global registry with all built-in globals
    pub fn new() -> Self {
        let mut registry = Self {
            objects: HashMap::new(),
        };

        // Register built-in global objects
        registry.register_number();
        registry.register_date();
        registry.register_array();

        registry
    }

    /// Register the Number global object
    fn register_number(&mut self) {
        let number_global = number::create_number_global();
        self.objects.insert("Number".to_string(), number_global);
    }

    /// Register the Date global object
    fn register_date(&mut self) {
        let date_global = date::create_date_global();
        self.objects.insert("Date".to_string(), date_global);
    }

    /// Register the Array global object
    fn register_array(&mut self) {
        let array_global = arrays::create_array_global();
        self.objects.insert("Array".to_string(), array_global);
    }

    /// Get a global object by name
    pub fn get_global(&self, name: &str) -> Option<&GlobalObject> {
        self.objects.get(name)
    }

    /// Check if a name refers to a global object
    pub fn is_global(&self, name: &str) -> bool {
        self.objects.contains_key(name)
    }

    /// Get all registered global names
    pub fn global_names(&self) -> Vec<&String> {
        self.objects.keys().collect()
    }

    /// Resolve a method call on a global object
    pub fn resolve_method_call(
        &self,
        object_name: &str,
        method_name: &str,
        is_static: bool,
    ) -> Option<&GlobalMethod> {
        let global = self.get_global(object_name)?;

        if is_static {
            global.static_methods.get(method_name)
        } else {
            global.instance_methods.get(method_name)
        }
    }

    /// Resolve a property access on a global object
    pub fn resolve_property_access(
        &self,
        object_name: &str,
        property_name: &str,
        is_static: bool,
    ) -> Option<&Type> {
        let global = self.get_global(object_name)?;

        if is_static {
            global.static_properties.get(property_name)
        } else {
            global.instance_properties.get(property_name)
        }
    }

    /// Get the constructor type for a global object
    pub fn get_constructor_type(&self, name: &str) -> Option<&Type> {
        self.get_global(name).map(|g| &g.constructor_type)
    }
}

/// Helper function to create a dummy source location for global objects
pub fn dummy_location() -> SourceLocation {
    SourceLocation::dummy()
}

/// Helper function to create a global method
pub fn create_global_method(
    name: &str,
    parameters: Vec<GlobalParameter>,
    return_type: Type,
    is_static: bool,
    is_constructor: bool,
) -> GlobalMethod {
    GlobalMethod {
        name: name.to_string(),
        parameters,
        return_type,
        is_static,
        is_constructor,
    }
}

/// Helper function to create a global parameter
pub fn create_global_parameter(
    name: &str,
    param_type: Type,
    optional: bool,
    default_value: Option<Expression>,
) -> GlobalParameter {
    GlobalParameter {
        name: name.to_string(),
        param_type,
        optional,
        default_value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_registry_creation() {
        let registry = GlobalRegistry::new();

        // Should have Number, Date, and Array registered
        assert!(registry.is_global("Number"));
        assert!(registry.is_global("Date"));
        assert!(registry.is_global("Array"));
        assert!(!registry.is_global("NonExistent"));
    }

    #[test]
    fn test_global_names() {
        let registry = GlobalRegistry::new();
        let names = registry.global_names();

        assert!(names.contains(&&"Number".to_string()));
        assert!(names.contains(&&"Date".to_string()));
        assert!(names.contains(&&"Array".to_string()));
    }
}
