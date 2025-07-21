//! Error handling for the Draf compiler
//!
//! This module defines all error types that can occur during compilation,
//! from lexical analysis through code generation.

use std::fmt;
use thiserror::Error;

/// The main error type for the Draf compiler
#[derive(Error, Debug, Clone)]
pub enum DrafError {
    /// Lexical analysis errors
    #[error("Lexer error at line {line}, column {column}: {message}")]
    LexerError {
        line: usize,
        column: usize,
        message: String,
    },

    /// Parser errors
    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    /// Semantic analysis errors (type checking, etc.)
    #[error("Semantic error at line {line}, column {column}: {message}")]
    SemanticError {
        line: usize,
        column: usize,
        message: String,
    },

    /// Type errors
    #[error("Type error at line {line}, column {column}: {message}")]
    TypeError {
        line: usize,
        column: usize,
        message: String,
    },

    /// Code generation errors
    #[error("Code generation error: {message}")]
    CodegenError { message: String },

    /// LLVM-related errors
    #[error("LLVM error: {message}")]
    LlvmError { message: String },

    /// IO errors (file reading/writing)
    #[error("IO error: {0}")]
    IoError(String),

    /// Internal compiler errors (bugs in the compiler itself)
    #[error("Internal compiler error: {message}. This is a bug in the compiler.")]
    InternalError { message: String },

    /// Multiple errors collected during compilation
    #[error("Multiple compilation errors occurred")]
    MultipleErrors(Vec<DrafError>),
}

impl DrafError {
    /// Create a lexer error
    pub fn lexer_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::LexerError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a parse error
    pub fn parse_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::ParseError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a semantic error
    pub fn semantic_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::SemanticError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a type error
    pub fn type_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::TypeError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a codegen error
    pub fn codegen_error(message: impl Into<String>) -> Self {
        Self::CodegenError {
            message: message.into(),
        }
    }

    /// Create an LLVM error
    pub fn llvm_error(message: impl Into<String>) -> Self {
        Self::LlvmError {
            message: message.into(),
        }
    }

    /// Create an IO error
    pub fn io_error(message: impl Into<String>) -> Self {
        Self::IoError(message.into())
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }

    /// Get the line number if this error has location information
    pub fn line(&self) -> Option<usize> {
        match self {
            Self::LexerError { line, .. }
            | Self::ParseError { line, .. }
            | Self::SemanticError { line, .. }
            | Self::TypeError { line, .. } => Some(*line),
            _ => None,
        }
    }

    /// Get the column number if this error has location information
    pub fn column(&self) -> Option<usize> {
        match self {
            Self::LexerError { column, .. }
            | Self::ParseError { column, .. }
            | Self::SemanticError { column, .. }
            | Self::TypeError { column, .. } => Some(*column),
            _ => None,
        }
    }

    /// Check if this error is recoverable (compilation can continue)
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::LexerError { .. } => false, // Can't continue without valid tokens
            Self::ParseError { .. } => false, // Can't continue without valid AST
            Self::SemanticError { .. } => true, // Can collect multiple semantic errors
            Self::TypeError { .. } => true,   // Can collect multiple type errors
            Self::CodegenError { .. } => false,
            Self::LlvmError { .. } => false,
            Self::IoError(_) => false,
            Self::InternalError { .. } => false,
            Self::MultipleErrors(_) => false,
        }
    }
}

/// A specialized Result type for Draf compiler operations
pub type DrafResult<T> = Result<T, DrafError>;

/// Error collector for gathering multiple errors during compilation
#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Vec<DrafError>,
}

impl ErrorCollector {
    /// Create a new error collector
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Add an error to the collection
    pub fn add_error(&mut self, error: DrafError) {
        self.errors.push(error);
    }

    /// Add a lexer error
    pub fn add_lexer_error(&mut self, line: usize, column: usize, message: impl Into<String>) {
        self.add_error(DrafError::lexer_error(line, column, message));
    }

    /// Add a parse error
    pub fn add_parse_error(&mut self, line: usize, column: usize, message: impl Into<String>) {
        self.add_error(DrafError::parse_error(line, column, message));
    }

    /// Add a semantic error
    pub fn add_semantic_error(&mut self, line: usize, column: usize, message: impl Into<String>) {
        self.add_error(DrafError::semantic_error(line, column, message));
    }

    /// Add a type error
    pub fn add_type_error(&mut self, line: usize, column: usize, message: impl Into<String>) {
        self.add_error(DrafError::type_error(line, column, message));
    }

    /// Check if any errors have been collected
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get the number of errors collected
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Convert to a Result - Ok(()) if no errors, Err with all errors if any
    pub fn into_result(self) -> DrafResult<()> {
        if self.errors.is_empty() {
            Ok(())
        } else if self.errors.len() == 1 {
            Err(self.errors.into_iter().next().unwrap())
        } else {
            Err(DrafError::MultipleErrors(self.errors))
        }
    }

    /// Get all collected errors
    pub fn errors(&self) -> &[DrafError] {
        &self.errors
    }

    /// Clear all collected errors
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}

impl fmt::Display for ErrorCollector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, error) in self.errors.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", error)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = DrafError::type_error(10, 5, "Cannot assign string to number");

        assert_eq!(error.line(), Some(10));
        assert_eq!(error.column(), Some(5));
        assert!(error.is_recoverable());

        let error_str = error.to_string();
        assert!(error_str.contains("line 10"));
        assert!(error_str.contains("column 5"));
        assert!(error_str.contains("Cannot assign string to number"));
    }

    #[test]
    fn test_error_collector() {
        let mut collector = ErrorCollector::new();

        assert!(!collector.has_errors());
        assert_eq!(collector.error_count(), 0);

        collector.add_type_error(1, 1, "First error");
        collector.add_semantic_error(2, 2, "Second error");

        assert!(collector.has_errors());
        assert_eq!(collector.error_count(), 2);

        let result = collector.into_result();
        assert!(result.is_err());

        match result.unwrap_err() {
            DrafError::MultipleErrors(errors) => {
                assert_eq!(errors.len(), 2);
            }
            _ => panic!("Expected MultipleErrors"),
        }
    }

    #[test]
    fn test_single_error_collector() {
        let mut collector = ErrorCollector::new();
        collector.add_type_error(1, 1, "Single error");

        let result = collector.into_result();
        assert!(result.is_err());

        // Single error should not be wrapped in MultipleErrors
        match result.unwrap_err() {
            DrafError::TypeError { .. } => {} // Expected
            _ => panic!("Expected TypeError, not MultipleErrors"),
        }
    }

    #[test]
    fn test_error_recoverability() {
        assert!(!DrafError::lexer_error(1, 1, "test").is_recoverable());
        assert!(!DrafError::parse_error(1, 1, "test").is_recoverable());
        assert!(DrafError::semantic_error(1, 1, "test").is_recoverable());
        assert!(DrafError::type_error(1, 1, "test").is_recoverable());
        assert!(!DrafError::codegen_error("test").is_recoverable());
        assert!(!DrafError::llvm_error("test").is_recoverable());
        assert!(!DrafError::io_error("test").is_recoverable());
        assert!(!DrafError::internal_error("test").is_recoverable());
    }
}
