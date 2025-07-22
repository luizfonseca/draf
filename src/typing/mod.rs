//! Type system module for advanced TypeScript typing features
//!
//! This module provides comprehensive support for TypeScript's type system including:
//! - Type aliases (`type` keyword)
//! - Interfaces with inheritance
//! - Generic types
//! - Union and intersection types
//! - Function type signatures
//! - Type resolution and validation

pub mod interface;
pub mod resolver;
pub mod type_alias;

use crate::ast::{SourceLocation, TypeAnnotation};
use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a type alias declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub target_type: TypeAnnotation,
    pub location: SourceLocation,
}

/// Represents an interface declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interface {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub extends: Vec<String>, // Names of interfaces this extends
    pub fields: Vec<InterfaceField>,
    pub methods: Vec<InterfaceMethod>,
    pub location: SourceLocation,
}

/// Represents a generic type parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeParameter {
    pub name: String,
    pub constraint: Option<TypeAnnotation>, // T extends SomeType
    pub default: Option<TypeAnnotation>,    // T = DefaultType
    pub location: SourceLocation,
}

/// Represents a field in an interface
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceField {
    pub name: String,
    pub field_type: TypeAnnotation,
    pub optional: bool,
    pub readonly: bool,
    pub location: SourceLocation,
}

/// Represents a method in an interface
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceMethod {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<MethodParameter>,
    pub return_type: TypeAnnotation,
    pub optional: bool,
    pub location: SourceLocation,
}

/// Represents a method parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodParameter {
    pub name: String,
    pub param_type: TypeAnnotation,
    pub optional: bool,
    pub location: SourceLocation,
}

/// Type context for tracking declared types and interfaces
#[derive(Debug, Clone, Default)]
pub struct TypeContext {
    /// Declared type aliases
    pub type_aliases: HashMap<String, TypeAlias>,
    /// Declared interfaces
    pub interfaces: HashMap<String, Interface>,
    /// Generic type instantiations cache
    pub generic_cache: HashMap<String, Type>,
}

impl TypeContext {
    /// Create a new empty type context
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a type alias to the context
    pub fn add_type_alias(&mut self, alias: TypeAlias) -> Result<(), String> {
        if self.type_aliases.contains_key(&alias.name) {
            return Err(format!("Type alias '{}' already declared", alias.name));
        }
        if self.interfaces.contains_key(&alias.name) {
            return Err(format!(
                "Name '{}' conflicts with existing interface",
                alias.name
            ));
        }
        self.type_aliases.insert(alias.name.clone(), alias);
        Ok(())
    }

    /// Add an interface to the context
    pub fn add_interface(&mut self, interface: Interface) -> Result<(), String> {
        if self.interfaces.contains_key(&interface.name) {
            return Err(format!("Interface '{}' already declared", interface.name));
        }
        if self.type_aliases.contains_key(&interface.name) {
            return Err(format!(
                "Name '{}' conflicts with existing type alias",
                interface.name
            ));
        }
        self.interfaces.insert(interface.name.clone(), interface);
        Ok(())
    }

    /// Get a type alias by name
    pub fn get_type_alias(&self, name: &str) -> Option<&TypeAlias> {
        self.type_aliases.get(name)
    }

    /// Get an interface by name
    pub fn get_interface(&self, name: &str) -> Option<&Interface> {
        self.interfaces.get(name)
    }

    /// Check if a type name exists (either alias or interface)
    pub fn has_type(&self, name: &str) -> bool {
        self.type_aliases.contains_key(name) || self.interfaces.contains_key(name)
    }

    /// Resolve a type annotation to a concrete type
    pub fn resolve_type(&self, type_annotation: &TypeAnnotation) -> Result<Type, String> {
        resolver::resolve_type(self, type_annotation)
    }

    /// Validate interface inheritance chains
    pub fn validate_interface_inheritance(&self) -> Result<(), String> {
        for interface in self.interfaces.values() {
            self.check_inheritance_cycles(&interface.name, &mut Vec::new())?;
        }
        Ok(())
    }

    /// Check for inheritance cycles
    fn check_inheritance_cycles(
        &self,
        interface_name: &str,
        visited: &mut Vec<String>,
    ) -> Result<(), String> {
        if visited.contains(&interface_name.to_string()) {
            return Err(format!(
                "Circular inheritance detected involving interface '{}'",
                interface_name
            ));
        }

        if let Some(interface) = self.get_interface(interface_name) {
            visited.push(interface_name.to_string());
            for extended in &interface.extends {
                self.check_inheritance_cycles(extended, visited)?;
            }
            visited.pop();
        }

        Ok(())
    }
}

/// Advanced type information for TypeScript features
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdvancedType {
    /// Generic type with parameters
    Generic { base: String, arguments: Vec<Type> },
    /// Conditional type (T extends U ? X : Y)
    Conditional {
        check_type: Box<Type>,
        extends_type: Box<Type>,
        true_type: Box<Type>,
        false_type: Box<Type>,
    },
    /// Mapped type ({ [K in keyof T]: U })
    Mapped {
        key_type: Box<Type>,
        value_type: Box<Type>,
        optional: bool,
        readonly: bool,
    },
    /// Template literal type
    TemplateLiteral {
        parts: Vec<String>,
        types: Vec<Type>,
    },
}

/// Type constraint for generic parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeConstraint {
    pub parameter: String,
    pub constraint: Type,
    pub location: SourceLocation,
}

/// Utility functions for type operations
impl TypeContext {
    /// Get all fields for an interface, including inherited ones
    pub fn get_all_interface_fields(
        &self,
        interface_name: &str,
    ) -> Result<Vec<InterfaceField>, String> {
        let mut fields = Vec::new();
        self.collect_interface_fields(interface_name, &mut fields, &mut Vec::new())?;
        Ok(fields)
    }

    /// Recursively collect interface fields including inheritance
    fn collect_interface_fields(
        &self,
        interface_name: &str,
        fields: &mut Vec<InterfaceField>,
        visited: &mut Vec<String>,
    ) -> Result<(), String> {
        if visited.contains(&interface_name.to_string()) {
            return Ok(()); // Already processed or circular reference
        }

        if let Some(interface) = self.get_interface(interface_name) {
            visited.push(interface_name.to_string());

            // First collect parent fields
            for parent in &interface.extends {
                self.collect_interface_fields(parent, fields, visited)?;
            }

            // Then add our own fields (they override parent fields with same name)
            for field in &interface.fields {
                // Remove any existing field with the same name
                fields.retain(|f| f.name != field.name);
                fields.push(field.clone());
            }

            visited.pop();
        }

        Ok(())
    }

    /// Check if one interface extends another (directly or indirectly)
    pub fn interface_extends(&self, child: &str, parent: &str) -> Result<bool, String> {
        if child == parent {
            return Ok(true);
        }

        if let Some(interface) = self.get_interface(child) {
            for extended in &interface.extends {
                if extended == parent || self.interface_extends(extended, parent)? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}
