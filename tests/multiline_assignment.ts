// Test for multiline assignment support
// This test validates that assignments can span multiple lines with newlines after the = operator

let final_edge: number = 5;

// Basic multiline assignment
let final_bool: boolean =
	(final_edge > 0 && true) || (false && final_edge == 1);

console.log(final_bool); // Should be true

// Test with numbers
let result: number = final_edge + 10 - 3;

console.log(result); // Should be 12

// Test with complex expressions
let complex: boolean = final_edge > 0 && final_edge < 10 && true;

console.log(complex); // Should be true

// Test with ternary
let ternary_result: number = final_edge > 0 ? 100 : 200;

console.log(ternary_result); // Should be 100

// Test with strict equality
let strict_check: boolean = final_edge === 5;

console.log(strict_check); // Should be true

// Test with nullish coalescing
let default_val: number = final_edge ?? 999;

console.log(default_val); // Should be 999 (right operand in our implementation)

// Multiple variables with multiline assignments
let x: number = 10;

let y: number = 20;

let sum: number = x + y;

console.log(sum); // Should be 30

// Nested expressions across lines
let nested: boolean = ((final_edge > 0 && true) || false) && final_edge !== 0;

console.log(nested); // Should be true

// Final validation
console.log(9999); // Test completion marker
