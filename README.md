# Draf - Strongly Typed TypeScript Compiler

A compiler for a strongly typed variant of TypeScript, targeting LLVM for high-performance native code generation.

## Requirements
- LLVM@18 (`brew install llvm@18`)
- Rust (latest stable version)
- TypeScript (for testing)

## Running TS tests
To run specific tests in the `tests` directory, use the following command:

```bash
make bin testfile=./tests/<fileName>.ts
```

This command compiles the specified TypeScript file and runs the tests defined within it. Replace `<fileName>` with the name of the test file you want to execute.


## Goals

- **Strong Typing**: No implicit type conversions, strict type checking at compile time
- **TypeScript Syntax**: Familiar syntax with stricter semantics
- **Performance**: LLVM backend for optimized native code generation
- **Simplicity**: Clean, understandable compiler architecture
- **Distinct Types**: `any`, `null`, and `undefined` as separate, well-defined types

## Project Status

🚧 **Early Development** - Basic compiler infrastructure in progress

### Current Features
- [ ] Lexical analysis (tokenization)
- [ ] Parser (AST generation)
- [ ] Semantic analysis (type checking)
- [ ] LLVM IR code generation
- [ ] Basic operations (variable assignment, arithmetic)

## Architecture

```
Source Code (.ts) → Lexer → Parser → Semantic Analyzer → Code Generator → LLVM IR → Native Binary
```

### Components

1. **Lexer**: Tokenizes TypeScript source code
2. **Parser**: Builds Abstract Syntax Tree (AST)
3. **Semantic Analyzer**: Type checking and symbol resolution
4. **Code Generator**: Converts AST to LLVM IR
5. **LLVM Backend**: Optimization and machine code generation

## Type System

Unlike standard TypeScript, Draf enforces strict typing:

- No implicit type conversions
- `any` is a distinct type that requires explicit handling
- `null` and `undefined` are separate types
- All operations must be type-safe at compile time

## Examples

```typescript
// Variable declarations with explicit types
let x: number = 42;
let name: string = "hello";
let flag: boolean = true;

// No implicit conversions - this would be a compile error
// let result = x + name; // Error: Cannot add number and string

// Explicit handling of nullable types
let maybeValue: number | null = getValue();
if (maybeValue !== null) {
    let doubled = maybeValue * 2; // Safe to use after null check
}
```

## Building

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the compiler
cargo build --release

# Run tests
cargo test

# Compile a TypeScript file
./target/release/draf input.ts -o output
```

## Dependencies

- **Rust**: Systems programming language
- **LLVM 17**: Code generation and optimization
- **inkwell**: Rust bindings for LLVM
- **logos**: Fast lexical analysis

## Development

```bash
# Run in development mode
cargo run -- input.ts

# Run with debug output
RUST_LOG=debug cargo run -- input.ts

# Benchmark parser performance
cargo bench
```

## Contributing

This is an experimental project exploring strongly typed compilation of TypeScript-like syntax. Contributions and ideas welcome!

## License

MIT License - see LICENSE file for details.
