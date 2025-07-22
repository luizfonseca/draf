//! Interface handling for TypeScript interface declarations
//!
//! This module provides functionality for parsing, validating, and working with
//! TypeScript interface declarations including inheritance and generic parameters.

use crate::ast::{SourceLocation, TypeAnnotation};
use crate::typing::{Interface, InterfaceField, InterfaceMethod, MethodParameter, TypeParameter};
use crate::DrafError;
use std::collections::HashMap;

/// Interface builder for constructing interface definitions
#[derive(Debug, Clone)]
pub struct InterfaceBuilder {
    name: String,
    type_parameters: Vec<TypeParameter>,
    extends: Vec<String>,
    fields: Vec<InterfaceField>,
    methods: Vec<InterfaceMethod>,
    location: SourceLocation,
}

impl InterfaceBuilder {
    /// Create a new interface builder
    pub fn new(name: String, location: SourceLocation) -> Self {
        Self {
            name,
            type_parameters: Vec::new(),
            extends: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            location,
        }
    }

    /// Add a type parameter to the interface
    pub fn add_type_parameter(mut self, param: TypeParameter) -> Self {
        self.type_parameters.push(param);
        self
    }

    /// Add an interface that this interface extends
    pub fn add_extends(mut self, interface_name: String) -> Self {
        if !self.extends.contains(&interface_name) {
            self.extends.push(interface_name);
        }
        self
    }

    /// Add a field to the interface
    pub fn add_field(mut self, field: InterfaceField) -> Self {
        self.fields.push(field);
        self
    }

    /// Add a method to the interface
    pub fn add_method(mut self, method: InterfaceMethod) -> Self {
        self.methods.push(method);
        self
    }

    /// Build the interface
    pub fn build(self) -> Interface {
        Interface {
            name: self.name,
            type_parameters: self.type_parameters,
            extends: self.extends,
            fields: self.fields,
            methods: self.methods,
            location: self.location,
        }
    }
}

/// Interface validator for checking interface definitions
pub struct InterfaceValidator;

impl InterfaceValidator {
    /// Validate an interface definition
    pub fn validate(interface: &Interface) -> Result<(), DrafError> {
        // Check for duplicate field names
        let mut field_names = HashMap::new();
        for field in &interface.fields {
            if let Some(existing_location) = field_names.get(&field.name) {
                return Err(DrafError::semantic_error(
                    field.location.line,
                    field.location.column,
                    format!(
                        "Duplicate field '{}' in interface '{}'. Previously declared at line {}",
                        field.name, interface.name, existing_location
                    ),
                ));
            }
            field_names.insert(field.name.clone(), field.location.line);
        }

        // Check for duplicate method names
        let mut method_names = HashMap::new();
        for method in &interface.methods {
            if let Some(existing_location) = method_names.get(&method.name) {
                return Err(DrafError::semantic_error(
                    method.location.line,
                    method.location.column,
                    format!(
                        "Duplicate method '{}' in interface '{}'. Previously declared at line {}",
                        method.name, interface.name, existing_location
                    ),
                ));
            }
            method_names.insert(method.name.clone(), method.location.line);
        }

        // Check for name conflicts between fields and methods
        for field in &interface.fields {
            if method_names.contains_key(&field.name) {
                return Err(DrafError::semantic_error(
                    field.location.line,
                    field.location.column,
                    format!(
                        "Field '{}' conflicts with method of the same name in interface '{}'",
                        field.name, interface.name
                    ),
                ));
            }
        }

        // Validate type parameters
        Self::validate_type_parameters(&interface.type_parameters)?;

        // Validate methods
        for method in &interface.methods {
            Self::validate_method(method)?;
        }

        Ok(())
    }

    /// Validate type parameters for duplicates and valid constraints
    fn validate_type_parameters(params: &[TypeParameter]) -> Result<(), DrafError> {
        let mut param_names = HashMap::new();
        for param in params {
            if let Some(existing_location) = param_names.get(&param.name) {
                return Err(DrafError::semantic_error(
                    param.location.line,
                    param.location.column,
                    format!(
                        "Duplicate type parameter '{}'. Previously declared at line {}",
                        param.name, existing_location
                    ),
                ));
            }
            param_names.insert(param.name.clone(), param.location.line);
        }
        Ok(())
    }

    /// Validate a method definition
    fn validate_method(method: &InterfaceMethod) -> Result<(), DrafError> {
        // Check for duplicate parameter names
        let mut param_names = HashMap::new();
        for param in &method.parameters {
            if let Some(existing_location) = param_names.get(&param.name) {
                return Err(DrafError::semantic_error(
                    param.location.line,
                    param.location.column,
                    format!(
                        "Duplicate parameter '{}' in method '{}'. Previously declared at line {}",
                        param.name, method.name, existing_location
                    ),
                ));
            }
            param_names.insert(param.name.clone(), param.location.line);
        }

        // Validate method type parameters
        Self::validate_type_parameters(&method.type_parameters)?;

        Ok(())
    }
}

/// Interface inheritance resolver
pub struct InterfaceInheritanceResolver;

impl InterfaceInheritanceResolver {
    /// Resolve the complete field set for an interface including inheritance
    pub fn resolve_fields(
        interface: &Interface,
        interfaces: &HashMap<String, Interface>,
    ) -> Result<Vec<InterfaceField>, DrafError> {
        let mut resolved_fields = Vec::new();
        let mut visited = Vec::new();

        Self::collect_fields(interface, interfaces, &mut resolved_fields, &mut visited)?;

        Ok(resolved_fields)
    }

    /// Resolve the complete method set for an interface including inheritance
    pub fn resolve_methods(
        interface: &Interface,
        interfaces: &HashMap<String, Interface>,
    ) -> Result<Vec<InterfaceMethod>, DrafError> {
        let mut resolved_methods = Vec::new();
        let mut visited = Vec::new();

        Self::collect_methods(interface, interfaces, &mut resolved_methods, &mut visited)?;

        Ok(resolved_methods)
    }

    /// Recursively collect fields from an interface and its parents
    fn collect_fields(
        interface: &Interface,
        interfaces: &HashMap<String, Interface>,
        fields: &mut Vec<InterfaceField>,
        visited: &mut Vec<String>,
    ) -> Result<(), DrafError> {
        if visited.contains(&interface.name) {
            return Err(DrafError::semantic_error(
                interface.location.line,
                interface.location.column,
                format!(
                    "Circular inheritance detected in interface '{}'",
                    interface.name
                ),
            ));
        }

        visited.push(interface.name.clone());

        // First, collect parent fields
        for parent_name in &interface.extends {
            if let Some(parent_interface) = interfaces.get(parent_name) {
                Self::collect_fields(parent_interface, interfaces, fields, visited)?;
            } else {
                return Err(DrafError::semantic_error(
                    interface.location.line,
                    interface.location.column,
                    format!(
                        "Interface '{}' extends unknown interface '{}'",
                        interface.name, parent_name
                    ),
                ));
            }
        }

        // Then add our own fields (they override parent fields with same name)
        for field in &interface.fields {
            // Remove any existing field with the same name
            fields.retain(|f| f.name != field.name);
            fields.push(field.clone());
        }

        visited.pop();
        Ok(())
    }

    /// Recursively collect methods from an interface and its parents
    fn collect_methods(
        interface: &Interface,
        interfaces: &HashMap<String, Interface>,
        methods: &mut Vec<InterfaceMethod>,
        visited: &mut Vec<String>,
    ) -> Result<(), DrafError> {
        if visited.contains(&interface.name) {
            return Err(DrafError::semantic_error(
                interface.location.line,
                interface.location.column,
                format!(
                    "Circular inheritance detected in interface '{}'",
                    interface.name
                ),
            ));
        }

        visited.push(interface.name.clone());

        // First, collect parent methods
        for parent_name in &interface.extends {
            if let Some(parent_interface) = interfaces.get(parent_name) {
                Self::collect_methods(parent_interface, interfaces, methods, visited)?;
            } else {
                return Err(DrafError::semantic_error(
                    interface.location.line,
                    interface.location.column,
                    format!(
                        "Interface '{}' extends unknown interface '{}'",
                        interface.name, parent_name
                    ),
                ));
            }
        }

        // Then add our own methods (they override parent methods with same signature)
        for method in &interface.methods {
            // For methods, we need to check signature compatibility
            methods.retain(|m| m.name != method.name);
            methods.push(method.clone());
        }

        visited.pop();
        Ok(())
    }

    /// Check if one interface is assignable to another (structural typing)
    pub fn is_assignable_to(
        source: &Interface,
        target: &Interface,
        interfaces: &HashMap<String, Interface>,
    ) -> Result<bool, DrafError> {
        // Get all fields and methods for both interfaces
        let source_fields = Self::resolve_fields(source, interfaces)?;
        let source_methods = Self::resolve_methods(source, interfaces)?;
        let target_fields = Self::resolve_fields(target, interfaces)?;
        let target_methods = Self::resolve_methods(target, interfaces)?;

        // Check that source has all required fields from target
        for target_field in &target_fields {
            if target_field.optional {
                continue; // Optional fields don't need to be present
            }

            let source_field = source_fields.iter().find(|f| f.name == target_field.name);

            match source_field {
                Some(field) => {
                    // TODO: Check type compatibility
                    // For now, we just check that the field exists
                    if field.readonly && !target_field.readonly {
                        return Ok(false); // Can't assign readonly to mutable
                    }
                }
                None => return Ok(false), // Required field missing
            }
        }

        // Check that source has all required methods from target
        for target_method in &target_methods {
            if target_method.optional {
                continue; // Optional methods don't need to be present
            }

            let source_method = source_methods.iter().find(|m| m.name == target_method.name);

            if source_method.is_none() {
                return Ok(false); // Required method missing
            }

            // TODO: Check method signature compatibility
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::TypeAnnotation;

    #[test]
    fn test_interface_builder() {
        let location = SourceLocation::new(1, 1);
        let interface = InterfaceBuilder::new("TestInterface".to_string(), location.clone())
            .add_field(InterfaceField {
                name: "field1".to_string(),
                field_type: TypeAnnotation::String {
                    location: location.clone(),
                },
                optional: false,
                readonly: false,
                location: location.clone(),
            })
            .build();

        assert_eq!(interface.name, "TestInterface");
        assert_eq!(interface.fields.len(), 1);
        assert_eq!(interface.fields[0].name, "field1");
    }

    #[test]
    fn test_interface_validation() {
        let location = SourceLocation::new(1, 1);
        let mut interface = InterfaceBuilder::new("TestInterface".to_string(), location.clone())
            .add_field(InterfaceField {
                name: "field1".to_string(),
                field_type: TypeAnnotation::String {
                    location: location.clone(),
                },
                optional: false,
                readonly: false,
                location: location.clone(),
            })
            .build();

        // Valid interface should pass
        assert!(InterfaceValidator::validate(&interface).is_ok());

        // Add duplicate field
        interface.fields.push(InterfaceField {
            name: "field1".to_string(),
            field_type: TypeAnnotation::Number {
                location: location.clone(),
            },
            optional: false,
            readonly: false,
            location: SourceLocation::new(2, 1),
        });

        // Should fail validation due to duplicate field
        assert!(InterfaceValidator::validate(&interface).is_err());
    }
}
