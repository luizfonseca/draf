// Test to verify that const variables must be initialized
// This file should produce a compilation error

// Valid const declaration
const VALID_CONST: number = 42;
console.log(VALID_CONST);

// Invalid const declaration - should cause error
const INVALID_CONST: number;
console.log(INVALID_CONST);
