//! String management module for the Draf TypeScript compiler
//!
//! This module provides efficient string handling, including:
//! - String literal parsing and validation
//! - String interpolation and template literals
//! - Memory-efficient string storage with interning
//! - String concatenation and type coercion
//! - LLVM string code generation support

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    /// String interning system for memory efficiency
    static ref STRING_INTERNER: Mutex<StringInterner> = Mutex::new(StringInterner::new());
}

/// String interner for memory-efficient string storage
#[derive(Debug)]
pub struct StringInterner {
    strings: Vec<String>,
    indices: HashMap<String, usize>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            indices: HashMap::new(),
        }
    }

    /// Intern a string and return its index
    pub fn intern(&mut self, s: String) -> usize {
        if let Some(&index) = self.indices.get(&s) {
            return index;
        }

        let index = self.strings.len();
        self.indices.insert(s.clone(), index);
        self.strings.push(s);
        index
    }

    /// Get a string by its index
    pub fn get(&self, index: usize) -> Option<&str> {
        self.strings.get(index).map(|s| s.as_str())
    }

    /// Get the total number of interned strings
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
}

/// Global function to intern a string
pub fn intern_string(s: String) -> usize {
    STRING_INTERNER.lock().unwrap().intern(s)
}

/// Global function to get an interned string
pub fn get_interned_string(index: usize) -> Option<String> {
    STRING_INTERNER
        .lock()
        .unwrap()
        .get(index)
        .map(|s| s.to_string())
}

/// Represents different types of string literals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StringLiteralType {
    /// Regular string with double quotes: "hello"
    DoubleQuoted,
    /// Regular string with single quotes: 'hello'
    SingleQuoted,
    /// Template literal with backticks: `hello ${name}`
    TemplateLiteral,
}

/// Represents a parsed string literal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringLiteral {
    /// The type of string literal
    pub literal_type: StringLiteralType,
    /// The processed content (escape sequences resolved)
    pub content: String,
    /// Template parts for template literals (None for regular strings)
    pub template_parts: Option<TemplateParts>,
    /// Whether this string spans multiple lines
    pub is_multiline: bool,
    /// Interned string index for memory efficiency
    pub interned_index: Option<usize>,
}

/// Template literal parts for interpolation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateParts {
    /// Static string parts between interpolations
    pub static_parts: Vec<String>,
    /// Expression parts for interpolation (as string representations for now)
    pub expressions: Vec<String>,
}

impl StringLiteral {
    /// Create a new regular string literal
    pub fn new_regular(content: String, literal_type: StringLiteralType) -> Self {
        let processed_content = Self::process_escape_sequences(&content);
        let interned_index = Some(intern_string(processed_content.clone()));

        Self {
            literal_type,
            content: processed_content,
            template_parts: None,
            is_multiline: content.contains('\n'),
            interned_index,
        }
    }

    /// Create a new template literal
    pub fn new_template(content: String) -> Self {
        let is_multiline = content.contains('\n');
        let (static_parts, expressions) = Self::parse_template_literal(&content);
        let template_parts = TemplateParts {
            static_parts,
            expressions,
        };

        Self {
            literal_type: StringLiteralType::TemplateLiteral,
            content,
            template_parts: Some(template_parts),
            is_multiline,
            interned_index: None, // Template literals are not interned due to dynamic nature
        }
    }

    /// Process escape sequences in a string
    fn process_escape_sequences(input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(&next_ch) = chars.peek() {
                    chars.next(); // consume the next character
                    match next_ch {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '\'' => result.push('\''),
                        '"' => result.push('"'),
                        '0' => result.push('\0'),
                        _ => {
                            // Unknown escape sequence, keep as is
                            result.push('\\');
                            result.push(next_ch);
                        }
                    }
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Parse template literal content into static parts and expressions
    fn parse_template_literal(content: &str) -> (Vec<String>, Vec<String>) {
        let mut static_parts = Vec::new();
        let mut expressions = Vec::new();
        let mut current_static = String::new();
        let mut chars = content.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' && chars.peek() == Some(&'{') {
                chars.next(); // consume '{'

                // Save current static part
                static_parts.push(current_static);
                current_static = String::new();

                // Parse expression until '}'
                let mut expression = String::new();
                let mut brace_count = 1;

                while let Some(expr_ch) = chars.next() {
                    if expr_ch == '{' {
                        brace_count += 1;
                    } else if expr_ch == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            break;
                        }
                    }
                    expression.push(expr_ch);
                }

                expressions.push(expression.trim().to_string());
            } else {
                current_static.push(ch);
            }
        }

        // Add the final static part
        static_parts.push(current_static);

        (static_parts, expressions)
    }

    /// Get the display length of the string (for console output)
    pub fn display_length(&self) -> usize {
        match &self.template_parts {
            Some(_) => {
                // For template literals, we'll estimate based on static parts
                // In practice, this would be calculated during interpolation
                self.content.len()
            }
            None => self.content.len(),
        }
    }

    /// Check if this is a template literal
    pub fn is_template(&self) -> bool {
        matches!(self.literal_type, StringLiteralType::TemplateLiteral)
    }

    /// Get the raw content without processing
    pub fn raw_content(&self) -> &str {
        &self.content
    }

    /// Get the processed content (with escape sequences resolved)
    pub fn processed_content(&self) -> &str {
        &self.content
    }
}

/// String operation types for code generation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StringOperation {
    /// String concatenation
    Concatenation,
    /// Template literal interpolation
    Interpolation,
    /// Type coercion to string
    Coercion,
}

/// String concatenation builder for efficient string operations
#[derive(Debug)]
pub struct StringConcatenationBuilder {
    parts: Vec<StringPart>,
}

/// A part of a string concatenation operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StringPart {
    /// Static string literal
    Literal(String),
    /// Variable reference (name)
    Variable(String),
    /// Expression result
    Expression(String),
    /// Interned string reference
    Interned(usize),
}

impl StringConcatenationBuilder {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    /// Add a literal string part
    pub fn add_literal(&mut self, content: String) {
        let interned_index = intern_string(content.clone());
        self.parts.push(StringPart::Interned(interned_index));
    }

    /// Add a variable reference
    pub fn add_variable(&mut self, var_name: String) {
        self.parts.push(StringPart::Variable(var_name));
    }

    /// Add an expression result
    pub fn add_expression(&mut self, expression: String) {
        self.parts.push(StringPart::Expression(expression));
    }

    /// Get the parts for code generation
    pub fn parts(&self) -> &[StringPart] {
        &self.parts
    }

    /// Estimate the final string length (for optimization)
    pub fn estimated_length(&self) -> usize {
        self.parts
            .iter()
            .map(|part| match part {
                StringPart::Literal(s) => s.len(),
                StringPart::Variable(_) => 10,   // Estimate
                StringPart::Expression(_) => 20, // Estimate
                StringPart::Interned(index) => {
                    get_interned_string(*index).map(|s| s.len()).unwrap_or(10)
                }
            })
            .sum()
    }
}

/// String validation utilities
pub struct StringValidator;

impl StringValidator {
    /// Validate a string literal for syntax correctness
    pub fn validate_string_literal(
        content: &str,
        literal_type: &StringLiteralType,
    ) -> Result<(), String> {
        match literal_type {
            StringLiteralType::DoubleQuoted => Self::validate_regular_string(content, '"'),
            StringLiteralType::SingleQuoted => Self::validate_regular_string(content, '\''),
            StringLiteralType::TemplateLiteral => Self::validate_template_literal(content),
        }
    }

    /// Validate regular string literals
    fn validate_regular_string(content: &str, quote_char: char) -> Result<(), String> {
        let mut chars = content.chars().peekable();
        let mut escape_next = false;

        while let Some(ch) = chars.next() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
            } else if ch == quote_char {
                return Err(format!(
                    "Unescaped quote character '{}' in string",
                    quote_char
                ));
            }
        }

        if escape_next {
            return Err("String ends with incomplete escape sequence".to_string());
        }

        Ok(())
    }

    /// Validate template literal syntax
    fn validate_template_literal(content: &str) -> Result<(), String> {
        let mut chars = content.chars().peekable();
        let mut in_expression = false;
        let mut brace_count = 0;

        while let Some(ch) = chars.next() {
            if ch == '$' && chars.peek() == Some(&'{') && !in_expression {
                chars.next(); // consume '{'
                in_expression = true;
                brace_count = 1;
            } else if in_expression {
                if ch == '{' {
                    brace_count += 1;
                } else if ch == '}' {
                    brace_count -= 1;
                    if brace_count == 0 {
                        in_expression = false;
                    }
                }
            }
        }

        if in_expression {
            return Err("Unclosed template literal expression".to_string());
        }

        Ok(())
    }
}

/// Type coercion utilities for string operations
pub struct TypeCoercion;

impl TypeCoercion {
    /// Check if a type can be coerced to string in + operations
    pub fn can_coerce_to_string(type_name: &str) -> bool {
        matches!(
            type_name,
            "number" | "boolean" | "string" | "null" | "undefined"
        )
    }

    /// Get the string representation format for a type
    pub fn get_string_format(type_name: &str) -> &'static str {
        match type_name {
            "number" => "%.2f",
            "boolean" => "%s",
            "string" => "%s",
            "null" => "null",
            "undefined" => "undefined",
            _ => "%s",
        }
    }

    /// Check if an operation should result in string concatenation
    pub fn is_string_concatenation(left_type: &str, right_type: &str) -> bool {
        left_type == "string" || right_type == "string"
    }
}

/// String formatting utilities
pub struct StringFormatter;

impl StringFormatter {
    /// Format a template literal with given values
    pub fn format_template(template: &TemplateParts, values: &[String]) -> String {
        let mut result = String::new();

        for (i, static_part) in template.static_parts.iter().enumerate() {
            result.push_str(static_part);

            if let Some(value) = values.get(i) {
                result.push_str(value);
            }
        }

        result
    }

    /// Escape a string for safe inclusion in generated code
    pub fn escape_for_codegen(input: &str) -> String {
        let mut result = String::new();

        for ch in input.chars() {
            match ch {
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                '\n' => result.push_str("\\n"),
                '\t' => result.push_str("\\t"),
                '\r' => result.push_str("\\r"),
                '\0' => result.push_str("\\0"),
                _ => result.push(ch),
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_literal_creation() {
        let literal = StringLiteral::new_regular(
            "Hello, World!".to_string(),
            StringLiteralType::DoubleQuoted,
        );

        assert_eq!(literal.content, "Hello, World!");
        assert_eq!(literal.literal_type, StringLiteralType::DoubleQuoted);
        assert!(!literal.is_multiline);
        assert!(literal.interned_index.is_some());
    }

    #[test]
    fn test_escape_sequences() {
        let literal = StringLiteral::new_regular(
            "Hello\\nWorld\\t!".to_string(),
            StringLiteralType::DoubleQuoted,
        );

        assert_eq!(literal.content, "Hello\nWorld\t!");
    }

    #[test]
    fn test_template_literal_parsing() {
        let literal =
            StringLiteral::new_template("Hello ${name}, you are ${age} years old!".to_string());

        assert!(literal.is_template());

        if let Some(parts) = &literal.template_parts {
            assert_eq!(
                parts.static_parts,
                vec!["Hello ", ", you are ", " years old!"]
            );
            assert_eq!(parts.expressions, vec!["name", "age"]);
        }
    }

    #[test]
    fn test_string_validation() {
        assert!(StringValidator::validate_string_literal(
            "Hello, World!",
            &StringLiteralType::DoubleQuoted
        )
        .is_ok());

        assert!(StringValidator::validate_string_literal(
            "Hello ${name}!",
            &StringLiteralType::TemplateLiteral
        )
        .is_ok());

        assert!(StringValidator::validate_string_literal(
            "Unclosed ${expression",
            &StringLiteralType::TemplateLiteral
        )
        .is_err());
    }

    #[test]
    fn test_string_interner() {
        let mut interner = StringInterner::new();

        let index1 = interner.intern("hello".to_string());
        let index2 = interner.intern("world".to_string());
        let index3 = interner.intern("hello".to_string()); // Should reuse index1

        assert_eq!(index1, index3);
        assert_ne!(index1, index2);
        assert_eq!(interner.get(index1), Some("hello"));
        assert_eq!(interner.get(index2), Some("world"));
    }

    #[test]
    fn test_type_coercion() {
        assert!(TypeCoercion::can_coerce_to_string("number"));
        assert!(TypeCoercion::can_coerce_to_string("boolean"));
        assert!(TypeCoercion::can_coerce_to_string("string"));
        assert!(!TypeCoercion::can_coerce_to_string("object"));

        assert!(TypeCoercion::is_string_concatenation("string", "number"));
        assert!(TypeCoercion::is_string_concatenation("number", "string"));
        assert!(!TypeCoercion::is_string_concatenation("number", "number"));
    }

    #[test]
    fn test_string_concatenation_builder() {
        let mut builder = StringConcatenationBuilder::new();

        builder.add_literal("Hello, ".to_string());
        builder.add_variable("name".to_string());
        builder.add_literal("!".to_string());

        assert_eq!(builder.parts().len(), 3);
        assert!(builder.estimated_length() > 0);
    }

    #[test]
    fn test_string_formatter() {
        let escaped = StringFormatter::escape_for_codegen("Hello\n\"World\"");
        assert_eq!(escaped, "Hello\\n\\\"World\\\"");
    }
}
