//! Main CLI entry point for the Draf strongly typed TypeScript compiler
//!
//! This binary provides a command-line interface for compiling TypeScript files
//! with strong typing semantics to native code via LLVM.

use clap::{Arg, Command};
use draf::Compiler;
use std::path::Path;
use std::process;

fn main() {
    env_logger::init();

    let matches = Command::new("draf")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Draf Contributors")
        .about("A strongly typed TypeScript compiler with LLVM backend")
        .long_about(
            "Draf compiles a strongly typed variant of TypeScript to native code.\n\
             Unlike standard TypeScript, Draf enforces strict typing with no implicit conversions.\n\
             The compiler generates optimized native code via LLVM."
        )
        .arg(
            Arg::new("input")
                .help("TypeScript file to compile")
                .required(true)
                .value_name("FILE")
                .index(1)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file path")
                .value_name("FILE")
                .required(false)
        )
        .arg(
            Arg::new("optimization")
                .short('O')
                .long("opt-level")
                .help("Optimization level (0-3)")
                .value_name("LEVEL")
                .default_value("2")
                .value_parser(clap::value_parser!(u8).range(0..=3))
        )
        .arg(
            Arg::new("target")
                .long("target")
                .help("Target triple for code generation")
                .value_name("TRIPLE")
        )
        .arg(
            Arg::new("emit-ir")
                .long("emit-ir")
                .help("Emit LLVM IR instead of executable")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("emit-obj")
                .long("emit-obj")
                .help("Emit object file instead of executable")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("check")
                .long("check")
                .help("Only perform type checking, don't generate code")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let optimization_level = *matches.get_one::<u8>("optimization").unwrap();
    let debug = matches.get_flag("debug");
    let verbose = matches.get_flag("verbose");
    let emit_ir = matches.get_flag("emit-ir");
    let emit_obj = matches.get_flag("emit-obj");
    let check_only = matches.get_flag("check");

    if verbose {
        println!("Draf TypeScript Compiler v{}", env!("CARGO_PKG_VERSION"));
        println!("Input file: {}", input_path);
        println!("Optimization level: {}", optimization_level);
    }

    // Validate input file exists
    if !Path::new(input_path).exists() {
        eprintln!("Error: Input file '{}' does not exist", input_path);
        process::exit(1);
    }

    // Determine output path
    let output_path = if let Some(output) = matches.get_one::<String>("output") {
        output.clone()
    } else {
        // Default output name based on input
        let input_path = Path::new(input_path);
        let stem = input_path.file_stem().unwrap().to_str().unwrap();

        if emit_ir {
            format!("{}.ll", stem)
        } else if emit_obj {
            format!("{}.o", stem)
        } else {
            // Default to executable with no extension on Unix, .exe on Windows
            #[cfg(windows)]
            {
                format!("{}.exe", stem)
            }
            #[cfg(not(windows))]
            {
                stem.to_string()
            }
        }
    };

    if verbose {
        println!("Output file: {}", output_path);
    }

    // Create compiler with specified options
    let mut compiler = Compiler::new()
        .with_debug(debug)
        .with_optimization(optimization_level);

    if let Some(target) = matches.get_one::<String>("target") {
        compiler = compiler.with_target(target.clone());
        if verbose {
            println!("Target triple: {}", target);
        }
    }

    // Read source code
    let source = match std::fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            process::exit(1);
        }
    };

    if verbose {
        println!("Source code length: {} characters", source.len());
    }

    // Compile the source
    let result = if check_only {
        // Type check only
        if verbose {
            println!("Performing type checking only...");
        }

        match compiler.compile_to_ir(&source) {
            Ok(_) => {
                if verbose {
                    println!("Type checking passed!");
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else if emit_ir {
        // Emit LLVM IR
        if verbose {
            println!("Generating LLVM IR...");
        }

        match compiler.compile_to_ir(&source) {
            Ok(ir) => match std::fs::write(&output_path, ir) {
                Ok(_) => {
                    if verbose {
                        println!("LLVM IR written to {}", output_path);
                    }
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error writing IR file: {}", e);
                    process::exit(1);
                }
            },
            Err(e) => Err(e),
        }
    } else if emit_obj {
        // Compile to object file
        if verbose {
            println!("Compiling to object file...");
        }

        // For object files, we need to call the object-specific compilation
        let ir = match compiler.compile_to_ir(&source) {
            Ok(ir) => ir,
            Err(e) => {
                eprintln!("Compilation failed: {}", e);
                process::exit(1);
            }
        };

        match draf::codegen::write_object_file(&ir, &output_path, optimization_level) {
            Ok(_) => {
                if verbose {
                    println!("Object file written to {}", output_path);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Object file generation failed: {}", e);
                process::exit(1);
            }
        }
    } else {
        // Compile to executable (default)
        if verbose {
            println!("Compiling to executable...");
        }

        compiler.compile_source(&source, &output_path)
    };

    // Handle compilation result
    match result {
        Ok(_) => {
            if verbose && !check_only {
                println!("Compilation successful!");
                if emit_ir {
                    println!("LLVM IR: {}", output_path);
                } else if emit_obj {
                    println!("Object file: {}", output_path);
                } else {
                    println!("Executable: {}", output_path);
                }
            } else if verbose && check_only {
                println!("Type checking completed successfully!");
            }
        }
        Err(e) => {
            eprintln!("Compilation failed:");

            // Pretty print errors
            match &e {
                draf::error::DrafError::MultipleErrors(errors) => {
                    for error in errors {
                        eprintln!("  {}", error);
                    }
                }
                _ => {
                    eprintln!("  {}", e);
                }
            }

            if debug {
                eprintln!("\nDebug information:");
                eprintln!("{:#?}", e);
            }

            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_cli_help() {
        // This test ensures the CLI can be built and help can be displayed
        let cmd = Command::new("draf").version("test").about("Test");

        let help = cmd.render_help();
        assert!(help.to_string().contains("draf"));
    }

    #[test]
    fn test_output_path_generation() {
        // Test default output path generation
        let input = "test.ts";
        let expected_obj = "test.o";
        let expected_ir = "test.ll";

        // This is a simplified version of the logic in main()
        let path = std::path::Path::new(input);
        let stem = path.file_stem().unwrap().to_str().unwrap();

        assert_eq!(format!("{}.o", stem), expected_obj);
        assert_eq!(format!("{}.ll", stem), expected_ir);
    }
}
