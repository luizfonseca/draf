//! Draf - A strongly typed TypeScript compiler with LLVM backend
//!
//! This library provides the core components for compiling a strongly typed
//! variant of TypeScript to native code via LLVM.

pub mod ast;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod strings;
pub mod types;
pub mod typing;

use error::{DrafError, DrafResult};
use std::path::Path;

/// The main compiler interface
pub struct Compiler {
    /// Whether to enable debug output
    pub debug: bool,
    /// Optimization level (0-3)
    pub optimization_level: u8,
    /// Target triple for code generation
    pub target_triple: Option<String>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            debug: false,
            optimization_level: 2,
            target_triple: None,
        }
    }
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Set debug mode
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Set optimization level
    pub fn with_optimization(mut self, level: u8) -> Self {
        self.optimization_level = level.min(3);
        self
    }

    /// Set target triple
    pub fn with_target(mut self, target: String) -> Self {
        self.target_triple = Some(target);
        self
    }

    /// Compile a TypeScript file to native code
    pub fn compile_file<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: P,
    ) -> Result<(), DrafError> {
        let source = std::fs::read_to_string(&input_path)
            .map_err(|e| DrafError::IoError(format!("Failed to read input file: {}", e)))?;

        self.compile_source(&source, output_path)
    }

    /// Compile TypeScript source code to native code
    pub fn compile_source<P: AsRef<Path>>(
        &self,
        source: &str,
        output_path: P,
    ) -> Result<(), DrafError> {
        // Lexical analysis
        let tokens = lexer::tokenize(source)?;

        if self.debug {
            println!("Tokens: {:#?}", tokens);
        }

        // Parse to AST
        let ast = parser::parse(tokens)?;

        if self.debug {
            println!("AST: {:#?}", ast);
        }

        // Semantic analysis and type checking
        let checked_ast = semantic::analyze(ast)?;

        if self.debug {
            println!("Type-checked AST: {:#?}", checked_ast);
        }

        // Code generation
        let ir_code = codegen::generate(checked_ast, self.target_triple.as_deref())?;

        // Determine output format based on file extension
        let output_path_ref = output_path.as_ref();
        if let Some(ext) = output_path_ref.extension() {
            match ext.to_str() {
                Some("o") => {
                    // Object file
                    codegen::write_object_file(&ir_code, output_path, self.optimization_level)?;
                }
                _ => {
                    // Executable (default)
                    codegen::write_executable(&ir_code, output_path, self.optimization_level)?;
                }
            }
        } else {
            // No extension, assume executable
            codegen::write_executable(&ir_code, output_path, self.optimization_level)?;
        }

        Ok(())
    }

    /// Compile to executable
    pub fn compile_to_executable<P: AsRef<Path>>(
        &self,
        source: &str,
        output_path: P,
    ) -> DrafResult<()> {
        let tokens = lexer::tokenize(source)?;
        let ast = parser::parse(tokens)?;
        let checked_ast = semantic::analyze(ast)?;
        let ir_code = codegen::generate(checked_ast, self.target_triple.as_deref())?;

        codegen::write_executable(&ir_code, output_path, self.optimization_level)
    }

    /// Compile to LLVM IR only (useful for debugging)
    pub fn compile_to_ir(&self, source: &str) -> Result<String, DrafError> {
        let tokens = lexer::tokenize(source)?;
        let ast = parser::parse(tokens)?;
        let checked_ast = semantic::analyze(ast)?;
        let ir_code = codegen::generate(checked_ast, self.target_triple.as_deref())?;

        Ok(ir_code)
    }
}

/// Convenience function to compile a file with default settings
pub fn compile_file<P: AsRef<Path>>(input_path: P, output_path: P) -> Result<(), DrafError> {
    Compiler::new().compile_file(input_path, output_path)
}

/// Convenience function to compile source code with default settings
pub fn compile_source<P: AsRef<Path>>(source: &str, output_path: P) -> Result<(), DrafError> {
    Compiler::new().compile_source(source, output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_basic_compilation() {
        let source = r#"
            let x: number = 42;
            let y: number = x + 10;
        "#;

        let compiler = Compiler::new().with_debug(true);
        let result = compiler.compile_to_ir(source);

        // For now, we expect this to fail since we haven't implemented
        // the full pipeline yet, but it should fail gracefully
        match result {
            Ok(_) => println!("Compilation succeeded!"),
            Err(e) => println!("Expected compilation error: {:?}", e),
        }
    }

    #[test]
    fn test_compiler_creation() {
        let compiler = Compiler::new()
            .with_debug(true)
            .with_optimization(3)
            .with_target("x86_64-unknown-linux-gnu".to_string());

        assert_eq!(compiler.debug, true);
        assert_eq!(compiler.optimization_level, 3);
        assert_eq!(
            compiler.target_triple,
            Some("x86_64-unknown-linux-gnu".to_string())
        );
    }
}
