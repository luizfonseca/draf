//! Type alias handling for TypeScript type declarations
//!
//! This module provides functionality for parsing, validating, and working with
//! TypeScript type alias declarations including generic type parameters.

use crate::ast::{SourceLocation, TypeAnnotation};
use crate::types::Type;
use crate::typing::{TypeAlias, TypeParameter};
use crate::DrafError;
use std::collections::HashMap;

/// Type alias builder for constructing type alias definitions
#[derive(Debug, Clone)]
pub struct TypeAliasBuilder {
    name: String,
    type_parameters: Vec<TypeParameter>,
    target_type: TypeAnnotation,
    location: SourceLocation,
}

impl TypeAliasBuilder {
    /// Create a new type alias builder
    pub fn new(name: String, target_type: TypeAnnotation, location: SourceLocation) -> Self {
        Self {
            name,
            type_parameters: Vec::new(),
            target_type,
            location,
        }
    }

    /// Add a type parameter to the type alias
    pub fn add_type_parameter(mut self, param: TypeParameter) -> Self {
        self.type_parameters.push(param);
        self
    }

    /// Build the type alias
    pub fn build(self) -> TypeAlias {
        TypeAlias {
            name: self.name,
            type_parameters: self.type_parameters,
            target_type: self.target_type,
            location: self.location,
        }
    }
}

/// Type alias validator for checking type alias definitions
pub struct TypeAliasValidator;

impl TypeAliasValidator {
    /// Validate a type alias definition
    pub fn validate(type_alias: &TypeAlias) -> Result<(), DrafError> {
        // Check for duplicate type parameter names
        let mut param_names = HashMap::new();
        for param in &type_alias.type_parameters {
            if let Some(existing_location) = param_names.get(&param.name) {
                return Err(DrafError::semantic_error(
                    param.location.line,
                    param.location.column,
                    format!(
                        "Duplicate type parameter '{}' in type alias '{}'. Previously declared at line {}",
                        param.name, type_alias.name, existing_location
                    ),
                ));
            }
            param_names.insert(param.name.clone(), param.location.line);
        }

        // Validate that the type alias doesn't reference itself directly
        Self::check_direct_recursion(type_alias)?;

        Ok(())
    }

    /// Check for direct recursion in type alias (e.g., type A = A)
    fn check_direct_recursion(type_alias: &TypeAlias) -> Result<(), DrafError> {
        if Self::references_self(&type_alias.target_type, &type_alias.name) {
            return Err(DrafError::semantic_error(
                type_alias.location.line,
                type_alias.location.column,
                format!(
                    "Type alias '{}' has a direct circular reference to itself",
                    type_alias.name
                ),
            ));
        }
        Ok(())
    }

    /// Check if a type annotation directly references a given type name
    fn references_self(type_annotation: &TypeAnnotation, type_name: &str) -> bool {
        match type_annotation {
            TypeAnnotation::Named { name, .. } => name == type_name,
            TypeAnnotation::Union { types, .. } => {
                types.iter().any(|t| Self::references_self(t, type_name))
            }
            TypeAnnotation::Array { element_type, .. } => {
                Self::references_self(element_type, type_name)
            }
            TypeAnnotation::Tuple { elements, .. } => {
                elements.iter().any(|t| Self::references_self(t, type_name))
            }
            TypeAnnotation::Function {
                parameters,
                return_type,
                ..
            } => {
                parameters
                    .iter()
                    .any(|p| Self::references_self(p, type_name))
                    || Self::references_self(return_type, type_name)
            }
            TypeAnnotation::Generic {
                name,
                type_arguments,
                ..
            } => {
                name == type_name
                    || type_arguments
                        .iter()
                        .any(|arg| Self::references_self(arg, type_name))
            }
            // Primitive types don't reference other types
            TypeAnnotation::Number { .. }
            | TypeAnnotation::String { .. }
            | TypeAnnotation::Boolean { .. }
            | TypeAnnotation::Any { .. }
            | TypeAnnotation::Null { .. }
            | TypeAnnotation::Undefined { .. }
            | TypeAnnotation::Void { .. }
            | TypeAnnotation::Never { .. } => false,
            // Object types need field checking
            TypeAnnotation::Object { fields, .. } => fields
                .iter()
                .any(|field| Self::references_self(&field.field_type, type_name)),
            TypeAnnotation::Intersection { types, .. } => {
                types.iter().any(|t| Self::references_self(t, type_name))
            }
        }
    }
}

/// Type alias resolver for expanding type aliases
pub struct TypeAliasResolver;

impl TypeAliasResolver {
    /// Resolve a type alias to its underlying type
    pub fn resolve(
        type_alias: &TypeAlias,
        type_arguments: &[Type],
        type_aliases: &HashMap<String, TypeAlias>,
    ) -> Result<Type, DrafError> {
        // Check that the number of type arguments matches the number of parameters
        if type_arguments.len() != type_alias.type_parameters.len() {
            return Err(DrafError::semantic_error(
                type_alias.location.line,
                type_alias.location.column,
                format!(
                    "Type alias '{}' expects {} type arguments, but {} were provided",
                    type_alias.name,
                    type_alias.type_parameters.len(),
                    type_arguments.len()
                ),
            ));
        }

        // Create substitution map for type parameters
        let mut substitutions = HashMap::new();
        for (param, arg) in type_alias.type_parameters.iter().zip(type_arguments) {
            substitutions.insert(param.name.clone(), arg.clone());
        }

        // Resolve the target type with substitutions
        Self::resolve_type_annotation(&type_alias.target_type, &substitutions, type_aliases)
    }

    /// Resolve a type annotation with type parameter substitutions
    fn resolve_type_annotation(
        type_annotation: &TypeAnnotation,
        substitutions: &HashMap<String, Type>,
        type_aliases: &HashMap<String, TypeAlias>,
    ) -> Result<Type, DrafError> {
        match type_annotation {
            TypeAnnotation::Number { .. } => Ok(Type::Number),
            TypeAnnotation::String { .. } => Ok(Type::String),
            TypeAnnotation::Boolean { .. } => Ok(Type::Boolean),
            TypeAnnotation::Any { .. } => Ok(Type::Any),
            TypeAnnotation::Null { .. } => Ok(Type::Null),
            TypeAnnotation::Undefined { .. } => Ok(Type::Undefined),
            TypeAnnotation::Void { .. } => Ok(Type::Void),
            TypeAnnotation::Never { .. } => Ok(Type::Never),

            TypeAnnotation::Named { name, .. } => {
                // Check if this is a type parameter
                if let Some(substituted_type) = substitutions.get(name) {
                    return Ok(substituted_type.clone());
                }

                // Check if this is a type alias
                if let Some(alias) = type_aliases.get(name) {
                    if alias.type_parameters.is_empty() {
                        // Simple type alias without parameters
                        return Self::resolve_type_annotation(
                            &alias.target_type,
                            substitutions,
                            type_aliases,
                        );
                    } else {
                        // Generic type alias requires explicit type arguments
                        return Err(DrafError::semantic_error(
                            0,
                            0,
                            format!("Generic type alias '{}' requires type arguments", name),
                        ));
                    }
                }

                // Unknown type name
                Err(DrafError::semantic_error(
                    0,
                    0,
                    format!("Unknown type '{}'", name),
                ))
            }

            TypeAnnotation::Array { element_type, .. } => {
                let resolved_element =
                    Self::resolve_type_annotation(element_type, substitutions, type_aliases)?;
                Ok(Type::Array(Box::new(resolved_element)))
            }

            TypeAnnotation::Tuple { elements, .. } => {
                let mut resolved_elements = Vec::new();
                for element in elements {
                    resolved_elements.push(Self::resolve_type_annotation(
                        element,
                        substitutions,
                        type_aliases,
                    )?);
                }
                Ok(Type::Tuple(resolved_elements))
            }

            TypeAnnotation::Union { types, .. } => {
                let mut resolved_types = Vec::new();
                for type_ann in types {
                    resolved_types.push(Self::resolve_type_annotation(
                        type_ann,
                        substitutions,
                        type_aliases,
                    )?);
                }
                Ok(Type::Union(resolved_types))
            }

            TypeAnnotation::Function {
                parameters,
                return_type,
                ..
            } => {
                let mut resolved_params = Vec::new();
                for param in parameters {
                    resolved_params.push(Self::resolve_type_annotation(
                        param,
                        substitutions,
                        type_aliases,
                    )?);
                }
                let resolved_return =
                    Self::resolve_type_annotation(return_type, substitutions, type_aliases)?;
                Ok(Type::Function {
                    params: resolved_params,
                    return_type: Box::new(resolved_return),
                })
            }

            TypeAnnotation::Generic {
                name,
                type_arguments,
                ..
            } => {
                // Resolve type arguments
                let mut resolved_args = Vec::new();
                for arg in type_arguments {
                    resolved_args.push(Self::resolve_type_annotation(
                        arg,
                        substitutions,
                        type_aliases,
                    )?);
                }

                // Check if this is a generic type alias
                if let Some(alias) = type_aliases.get(name) {
                    return Self::resolve(alias, &resolved_args, type_aliases);
                }

                // For now, treat as generic type (will be expanded later with proper generic support)
                Ok(Type::Generic {
                    name: name.clone(),
                    constraints: Vec::new(), // TODO: Handle constraints
                })
            }

            TypeAnnotation::Object { fields, .. } => {
                let mut resolved_fields = HashMap::new();
                for field in fields {
                    let resolved_type = Self::resolve_type_annotation(
                        &field.field_type,
                        substitutions,
                        type_aliases,
                    )?;
                    resolved_fields.insert(field.name.clone(), resolved_type);
                }
                Ok(Type::Object(resolved_fields))
            }

            TypeAnnotation::Intersection { types, .. } => {
                let mut resolved_types = Vec::new();
                for type_ann in types {
                    resolved_types.push(Self::resolve_type_annotation(
                        type_ann,
                        substitutions,
                        type_aliases,
                    )?);
                }
                // For now, treat intersection as the first type
                // TODO: Implement proper intersection type support
                Ok(resolved_types.into_iter().next().unwrap_or(Type::Any))
            }
        }
    }

    /// Check for circular dependencies in type aliases
    pub fn check_circular_dependencies(
        type_aliases: &HashMap<String, TypeAlias>,
    ) -> Result<(), DrafError> {
        for (name, alias) in type_aliases {
            let mut visited = Vec::new();
            Self::check_circular_dependency_recursive(name, alias, type_aliases, &mut visited)?;
        }
        Ok(())
    }

    /// Recursively check for circular dependencies
    fn check_circular_dependency_recursive(
        current_name: &str,
        current_alias: &TypeAlias,
        type_aliases: &HashMap<String, TypeAlias>,
        visited: &mut Vec<String>,
    ) -> Result<(), DrafError> {
        if visited.contains(&current_name.to_string()) {
            return Err(DrafError::semantic_error(
                current_alias.location.line,
                current_alias.location.column,
                format!(
                    "Circular dependency detected in type alias '{}': {}",
                    current_name,
                    visited.join(" -> ")
                ),
            ));
        }

        visited.push(current_name.to_string());

        // Check all referenced type names in the target type
        let referenced_types = Self::extract_referenced_types(&current_alias.target_type);
        for referenced_type in referenced_types {
            if let Some(referenced_alias) = type_aliases.get(&referenced_type) {
                Self::check_circular_dependency_recursive(
                    &referenced_type,
                    referenced_alias,
                    type_aliases,
                    visited,
                )?;
            }
        }

        visited.pop();
        Ok(())
    }

    /// Extract all type names referenced in a type annotation
    fn extract_referenced_types(type_annotation: &TypeAnnotation) -> Vec<String> {
        let mut types = Vec::new();
        Self::extract_referenced_types_recursive(type_annotation, &mut types);
        types
    }

    /// Recursively extract referenced type names
    fn extract_referenced_types_recursive(
        type_annotation: &TypeAnnotation,
        types: &mut Vec<String>,
    ) {
        match type_annotation {
            TypeAnnotation::Named { name, .. } => {
                if !types.contains(name) {
                    types.push(name.clone());
                }
            }
            TypeAnnotation::Array { element_type, .. } => {
                Self::extract_referenced_types_recursive(element_type, types);
            }
            TypeAnnotation::Tuple { elements, .. } => {
                for element in elements {
                    Self::extract_referenced_types_recursive(element, types);
                }
            }
            TypeAnnotation::Union {
                types: union_types, ..
            } => {
                for union_type in union_types {
                    Self::extract_referenced_types_recursive(union_type, types);
                }
            }
            TypeAnnotation::Function {
                parameters,
                return_type,
                ..
            } => {
                for param in parameters {
                    Self::extract_referenced_types_recursive(param, types);
                }
                Self::extract_referenced_types_recursive(return_type, types);
            }
            TypeAnnotation::Generic {
                name,
                type_arguments,
                ..
            } => {
                if !types.contains(name) {
                    types.push(name.clone());
                }
                for arg in type_arguments {
                    Self::extract_referenced_types_recursive(arg, types);
                }
            }
            TypeAnnotation::Object { fields, .. } => {
                for field in fields {
                    Self::extract_referenced_types_recursive(&field.field_type, types);
                }
            }
            TypeAnnotation::Intersection {
                types: intersection_types,
                ..
            } => {
                for intersection_type in intersection_types {
                    Self::extract_referenced_types_recursive(intersection_type, types);
                }
            }
            // Primitive types don't reference other types
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::TypeAnnotation;

    #[test]
    fn test_type_alias_builder() {
        let location = SourceLocation::new(1, 1);
        let target_type = TypeAnnotation::String {
            location: location.clone(),
        };
        let type_alias =
            TypeAliasBuilder::new("StringAlias".to_string(), target_type, location).build();

        assert_eq!(type_alias.name, "StringAlias");
        assert!(matches!(
            type_alias.target_type,
            TypeAnnotation::String { .. }
        ));
    }

    #[test]
    fn test_direct_recursion_detection() {
        let location = SourceLocation::new(1, 1);
        let target_type = TypeAnnotation::Named {
            name: "SelfReference".to_string(),
            location: location.clone(),
        };
        let type_alias =
            TypeAliasBuilder::new("SelfReference".to_string(), target_type, location).build();

        assert!(TypeAliasValidator::validate(&type_alias).is_err());
    }

    #[test]
    fn test_valid_type_alias() {
        let location = SourceLocation::new(1, 1);
        let target_type = TypeAnnotation::String {
            location: location.clone(),
        };
        let type_alias =
            TypeAliasBuilder::new("StringAlias".to_string(), target_type, location).build();

        assert!(TypeAliasValidator::validate(&type_alias).is_ok());
    }
}
