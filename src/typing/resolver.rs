//! Type resolver for advanced type resolution and validation
//!
//! This module provides functionality for resolving type annotations to concrete types,
//! handling generic type instantiation, and validating type compatibility.

use crate::ast::{SourceLocation, TypeAnnotation};
use crate::types::Type;
use crate::typing::{Interface, TypeAlias, TypeContext, TypeParameter};
use crate::DrafError;
use std::collections::HashMap;

/// Type resolver for converting type annotations to concrete types
pub struct TypeResolver<'a> {
    context: &'a TypeContext,
    generic_substitutions: HashMap<String, Type>,
}

impl<'a> TypeResolver<'a> {
    /// Create a new type resolver with the given context
    pub fn new(context: &'a TypeContext) -> Self {
        Self {
            context,
            generic_substitutions: HashMap::new(),
        }
    }

    /// Create a type resolver with generic substitutions
    pub fn with_substitutions(
        context: &'a TypeContext,
        substitutions: HashMap<String, Type>,
    ) -> Self {
        Self {
            context,
            generic_substitutions: substitutions,
        }
    }

    /// Resolve a type annotation to a concrete type
    pub fn resolve(&self, type_annotation: &TypeAnnotation) -> Result<Type, DrafError> {
        match type_annotation {
            TypeAnnotation::Number { .. } => Ok(Type::Number),
            TypeAnnotation::String { .. } => Ok(Type::String),
            TypeAnnotation::Boolean { .. } => Ok(Type::Boolean),
            TypeAnnotation::Any { .. } => Ok(Type::Any),
            TypeAnnotation::Null { .. } => Ok(Type::Null),
            TypeAnnotation::Undefined { .. } => Ok(Type::Undefined),
            TypeAnnotation::Void { .. } => Ok(Type::Void),
            TypeAnnotation::Never { .. } => Ok(Type::Never),

            TypeAnnotation::Named { name, location } => self.resolve_named_type(name, location),

            TypeAnnotation::Array { element_type, .. } => {
                let resolved_element = self.resolve(element_type)?;
                Ok(Type::Array(Box::new(resolved_element)))
            }

            TypeAnnotation::Tuple { elements, .. } => {
                let mut resolved_elements = Vec::new();
                for element in elements {
                    resolved_elements.push(self.resolve(element)?);
                }
                Ok(Type::Tuple(resolved_elements))
            }

            TypeAnnotation::Union { types, .. } => {
                let mut resolved_types = Vec::new();
                for type_ann in types {
                    resolved_types.push(self.resolve(type_ann)?);
                }
                Ok(Type::Union(resolved_types))
            }

            TypeAnnotation::Object { fields, .. } => {
                let mut resolved_fields = HashMap::new();
                for field in fields {
                    let resolved_type = self.resolve(&field.field_type)?;
                    resolved_fields.insert(field.name.clone(), resolved_type);
                }
                Ok(Type::Object(resolved_fields))
            }

            TypeAnnotation::Function {
                parameters,
                return_type,
                ..
            } => {
                let mut resolved_params = Vec::new();
                for param in parameters {
                    resolved_params.push(self.resolve(param)?);
                }
                let resolved_return = self.resolve(return_type)?;
                Ok(Type::Function {
                    params: resolved_params,
                    return_type: Box::new(resolved_return),
                })
            }

            TypeAnnotation::Generic {
                name,
                type_arguments,
                location,
            } => self.resolve_generic_type(name, type_arguments, location),

            TypeAnnotation::Intersection { types, .. } => {
                let mut resolved_types = Vec::new();
                for type_ann in types {
                    resolved_types.push(self.resolve(type_ann)?);
                }
                // For now, treat intersection as the first type
                // TODO: Implement proper intersection type support
                Ok(resolved_types.into_iter().next().unwrap_or(Type::Any))
            }
        }
    }

    /// Resolve a named type (could be type alias or interface)
    fn resolve_named_type(&self, name: &str, location: &SourceLocation) -> Result<Type, DrafError> {
        // Check if it's a generic type parameter
        if let Some(substituted_type) = self.generic_substitutions.get(name) {
            return Ok(substituted_type.clone());
        }

        // Check if it's a type alias
        if let Some(type_alias) = self.context.get_type_alias(name) {
            return self.resolve_type_alias(type_alias, &[], location);
        }

        // Check if it's an interface
        if let Some(interface) = self.context.get_interface(name) {
            return self.resolve_interface_type(interface, &[], location);
        }

        // Unknown type
        Err(DrafError::semantic_error(
            location.line,
            location.column,
            format!("Unknown type '{}'", name),
        ))
    }

    /// Resolve a generic type with type arguments
    fn resolve_generic_type(
        &self,
        name: &str,
        type_arguments: &[TypeAnnotation],
        location: &SourceLocation,
    ) -> Result<Type, DrafError> {
        // Resolve type arguments first
        let mut resolved_args = Vec::new();
        for arg in type_arguments {
            resolved_args.push(self.resolve(arg)?);
        }

        // Check if it's a generic type alias
        if let Some(type_alias) = self.context.get_type_alias(name) {
            return self.resolve_type_alias(type_alias, &resolved_args, location);
        }

        // Check if it's a generic interface
        if let Some(interface) = self.context.get_interface(name) {
            return self.resolve_interface_type(interface, &resolved_args, location);
        }

        // Built-in generic types (like Array<T>, which should be handled above)
        // For now, treat as unknown generic
        Err(DrafError::semantic_error(
            location.line,
            location.column,
            format!("Unknown generic type '{}'", name),
        ))
    }

    /// Resolve a type alias with optional type arguments
    fn resolve_type_alias(
        &self,
        type_alias: &TypeAlias,
        type_arguments: &[Type],
        location: &SourceLocation,
    ) -> Result<Type, DrafError> {
        // Check argument count
        if type_arguments.len() != type_alias.type_parameters.len() {
            return Err(DrafError::semantic_error(
                location.line,
                location.column,
                format!(
                    "Type alias '{}' expects {} type arguments, but {} were provided",
                    type_alias.name,
                    type_alias.type_parameters.len(),
                    type_arguments.len()
                ),
            ));
        }

        // Create substitutions for type parameters
        let mut substitutions = self.generic_substitutions.clone();
        for (param, arg) in type_alias.type_parameters.iter().zip(type_arguments) {
            substitutions.insert(param.name.clone(), arg.clone());
        }

        // Create new resolver with substitutions
        let resolver = TypeResolver::with_substitutions(self.context, substitutions);
        resolver.resolve(&type_alias.target_type)
    }

    /// Resolve an interface type with optional type arguments
    fn resolve_interface_type(
        &self,
        interface: &Interface,
        type_arguments: &[Type],
        location: &SourceLocation,
    ) -> Result<Type, DrafError> {
        // Check argument count
        if type_arguments.len() != interface.type_parameters.len() {
            return Err(DrafError::semantic_error(
                location.line,
                location.column,
                format!(
                    "Interface '{}' expects {} type arguments, but {} were provided",
                    interface.name,
                    interface.type_parameters.len(),
                    type_arguments.len()
                ),
            ));
        }

        // For now, create an interface type representation
        // This could be expanded to handle structural typing
        let mut field_map = HashMap::new();

        // Create substitutions for type parameters
        let mut substitutions = self.generic_substitutions.clone();
        for (param, arg) in interface.type_parameters.iter().zip(type_arguments) {
            substitutions.insert(param.name.clone(), arg.clone());
        }

        // Resolve field types with substitutions
        let resolver = TypeResolver::with_substitutions(self.context, substitutions);
        for field in &interface.fields {
            let resolved_type = resolver.resolve(&field.field_type)?;
            field_map.insert(field.name.clone(), resolved_type);
        }

        Ok(Type::Interface {
            name: interface.name.clone(),
            fields: field_map,
        })
    }
}

/// Global type resolution function
pub fn resolve_type(
    context: &TypeContext,
    type_annotation: &TypeAnnotation,
) -> Result<Type, String> {
    let resolver = TypeResolver::new(context);
    resolver.resolve(type_annotation).map_err(|e| e.to_string())
}

/// Type compatibility checker
pub struct TypeCompatibilityChecker<'a> {
    context: &'a TypeContext,
}

impl<'a> TypeCompatibilityChecker<'a> {
    /// Create a new type compatibility checker
    pub fn new(context: &'a TypeContext) -> Self {
        Self { context }
    }

    /// Check if source type is assignable to target type
    pub fn is_assignable(&self, source: &Type, target: &Type) -> Result<bool, DrafError> {
        // Exact match
        if source == target {
            return Ok(true);
        }

        match (source, target) {
            // Any is assignable to/from anything
            (Type::Any, _) | (_, Type::Any) => Ok(true),

            // Never is assignable to everything, nothing is assignable to Never
            (Type::Never, _) => Ok(true),
            (_, Type::Never) => Ok(false),

            // Null and undefined assignability
            (Type::Null, Type::Union(types)) => Ok(types.contains(&Type::Null)),
            (Type::Undefined, Type::Union(types)) => Ok(types.contains(&Type::Undefined)),

            // Union type assignability
            (Type::Union(source_types), target_type) => {
                // All source types must be assignable to target
                for source_type in source_types {
                    if !self.is_assignable(source_type, target_type)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            (source_type, Type::Union(target_types)) => {
                // Source must be assignable to at least one target type
                for target_type in target_types {
                    if self.is_assignable(source_type, target_type)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }

            // Array assignability (covariant)
            (Type::Array(source_elem), Type::Array(target_elem)) => {
                self.is_assignable(source_elem, target_elem)
            }

            // Tuple assignability
            (Type::Tuple(source_elems), Type::Tuple(target_elems)) => {
                if source_elems.len() != target_elems.len() {
                    return Ok(false);
                }
                for (source_elem, target_elem) in source_elems.iter().zip(target_elems) {
                    if !self.is_assignable(source_elem, target_elem)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }

            // Object assignability (structural)
            (Type::Object(source_fields), Type::Object(target_fields)) => {
                // Target fields must all exist in source with compatible types
                for (field_name, target_type) in target_fields {
                    if let Some(source_type) = source_fields.get(field_name) {
                        if !self.is_assignable(source_type, target_type)? {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false); // Missing field
                    }
                }
                Ok(true)
            }

            // Interface assignability
            (
                Type::Interface {
                    fields: source_fields,
                    ..
                },
                Type::Interface {
                    fields: target_fields,
                    ..
                },
            ) => {
                // Similar to object assignability
                for (field_name, target_type) in target_fields {
                    if let Some(source_type) = source_fields.get(field_name) {
                        if !self.is_assignable(source_type, target_type)? {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                }
                Ok(true)
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
                // Parameter count must match
                if source_params.len() != target_params.len() {
                    return Ok(false);
                }

                // Parameters are contravariant
                for (source_param, target_param) in source_params.iter().zip(target_params) {
                    if !self.is_assignable(target_param, source_param)? {
                        return Ok(false);
                    }
                }

                // Return type is covariant
                self.is_assignable(source_return, target_return)
            }

            // Default case: types are not assignable
            _ => Ok(false),
        }
    }

    /// Check if a type satisfies a constraint
    pub fn satisfies_constraint(&self, type_: &Type, constraint: &Type) -> Result<bool, DrafError> {
        self.is_assignable(type_, constraint)
    }
}

/// Generic type instantiation utilities
pub struct GenericInstantiator<'a> {
    context: &'a TypeContext,
}

impl<'a> GenericInstantiator<'a> {
    /// Create a new generic instantiator
    pub fn new(context: &'a TypeContext) -> Self {
        Self { context }
    }

    /// Instantiate a generic type with concrete type arguments
    pub fn instantiate(
        &self,
        base_type: &str,
        type_arguments: &[Type],
        location: &SourceLocation,
    ) -> Result<Type, DrafError> {
        // Check if it's a generic type alias
        if let Some(type_alias) = self.context.get_type_alias(base_type) {
            let resolver = TypeResolver::new(self.context);
            return resolver.resolve_type_alias(type_alias, type_arguments, location);
        }

        // Check if it's a generic interface
        if let Some(interface) = self.context.get_interface(base_type) {
            let resolver = TypeResolver::new(self.context);
            return resolver.resolve_interface_type(interface, type_arguments, location);
        }

        Err(DrafError::semantic_error(
            location.line,
            location.column,
            format!("Unknown generic type '{}'", base_type),
        ))
    }

    /// Validate that type arguments satisfy parameter constraints
    pub fn validate_constraints(
        &self,
        type_parameters: &[TypeParameter],
        type_arguments: &[Type],
        location: &SourceLocation,
    ) -> Result<(), DrafError> {
        if type_parameters.len() != type_arguments.len() {
            return Err(DrafError::semantic_error(
                location.line,
                location.column,
                format!(
                    "Expected {} type arguments, but {} were provided",
                    type_parameters.len(),
                    type_arguments.len()
                ),
            ));
        }

        let checker = TypeCompatibilityChecker::new(self.context);
        for (param, arg) in type_parameters.iter().zip(type_arguments) {
            if let Some(constraint) = &param.constraint {
                let resolver = TypeResolver::new(self.context);
                let constraint_type = resolver.resolve(constraint)?;
                if !checker.satisfies_constraint(arg, &constraint_type)? {
                    return Err(DrafError::semantic_error(
                        location.line,
                        location.column,
                        format!(
                            "Type argument does not satisfy constraint for parameter '{}'",
                            param.name
                        ),
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::TypeAnnotation;

    #[test]
    fn test_primitive_type_resolution() {
        let context = TypeContext::new();
        let resolver = TypeResolver::new(&context);

        let number_type = TypeAnnotation::Number {
            location: SourceLocation::new(1, 1),
        };
        assert_eq!(resolver.resolve(&number_type).unwrap(), Type::Number);

        let string_type = TypeAnnotation::String {
            location: SourceLocation::new(1, 1),
        };
        assert_eq!(resolver.resolve(&string_type).unwrap(), Type::String);
    }

    #[test]
    fn test_array_type_resolution() {
        let context = TypeContext::new();
        let resolver = TypeResolver::new(&context);

        let array_type = TypeAnnotation::Array {
            element_type: Box::new(TypeAnnotation::Number {
                location: SourceLocation::new(1, 1),
            }),
            location: SourceLocation::new(1, 1),
        };

        let resolved = resolver.resolve(&array_type).unwrap();
        assert!(matches!(resolved, Type::Array(_)));
        if let Type::Array(element) = resolved {
            assert_eq!(*element, Type::Number);
        }
    }

    #[test]
    fn test_type_compatibility() {
        let context = TypeContext::new();
        let checker = TypeCompatibilityChecker::new(&context);

        // Same types should be assignable
        assert!(checker.is_assignable(&Type::Number, &Type::Number).unwrap());

        // Any should be assignable to/from anything
        assert!(checker.is_assignable(&Type::Any, &Type::Number).unwrap());
        assert!(checker.is_assignable(&Type::Number, &Type::Any).unwrap());

        // Different primitive types should not be assignable
        assert!(!checker.is_assignable(&Type::Number, &Type::String).unwrap());
    }
}
