// Test to verify const reassignment behavior
// This file tests whether const variables can be reassigned (they shouldn't be)

// Valid const declarations
const PI: number = 3.14159;
const MAX_COUNT: number = 100;
const IS_ENABLED: boolean = true;

console.log(PI);
console.log(MAX_COUNT);
console.log(IS_ENABLED);

// Test reassignment - this should cause an error in a fully implemented system
// Note: Current implementation may not catch this at compile time
// but this demonstrates the expected behavior

// Try to reassign const values (should be prevented)
PI = 2.71828;
console.log(PI);

MAX_COUNT = 200;
console.log(MAX_COUNT);

IS_ENABLED = false;
console.log(IS_ENABLED);
