// Test to verify const reassignment prevention
// This file should produce compilation errors for const reassignment attempts

// Valid const declarations
const PI: number = 3.14159;
const MAX_SIZE: number = 1000;
const IS_ENABLED: boolean = true;

// Valid usage of const variables
console.log(PI);
console.log(MAX_SIZE);
console.log(IS_ENABLED);

// Valid operations with const variables
console.log(PI * 2);
console.log(MAX_SIZE + 100);
console.log(IS_ENABLED && true);

// These should cause compilation errors:

// Error: Cannot assign to const variable 'PI'
PI = 2.71828;

// Error: Cannot assign to const variable 'MAX_SIZE'
MAX_SIZE = 2000;

// Error: Cannot assign to const variable 'IS_ENABLED'
IS_ENABLED = false;

// Additional test cases
const ANOTHER_CONST = 42;
console.log(ANOTHER_CONST);

// Error: Cannot assign to const variable 'ANOTHER_CONST'
ANOTHER_CONST = 99;

// End of test
