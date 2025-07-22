// Comprehensive edge case tests for multiline assignments
// This test validates various edge cases and complex scenarios for multiline assignment support

let base_value: number = 10;

// Test 1: Multiple newlines after assignment
let test1: number = base_value + 5;
console.log(test1); // Should be 15

// Test 2: Tabs and spaces mixed with newlines
let test2: boolean = base_value > 5 && true;
console.log(test2); // Should be true

// Test 3: Complex nested expressions across lines
let test3: number = (base_value > 5 ? base_value * 2 : base_value / 2) + 10;
console.log(test3); // Should be 30

// Test 4: Boolean expressions with all operators
let test4: boolean =
	(base_value > 0 && true) || (false && base_value == 1) || base_value === 10;
console.log(test4); // Should be true

// Test 5: Nullish coalescing across lines
let test5: number = base_value ?? 999;
console.log(test5); // Should be 999 (right operand)

// Test 6: Strict equality chains
let test6: boolean =
	base_value === 10 && base_value !== 5 && base_value === base_value;
console.log(test6); // Should be true

// Test 7: Ternary operator spread across multiple lines
let test7: number = base_value > 5 ? 100 : 200;
console.log(test7); // Should be 100

// Test 8: Assignment to assignment (chained)
let intermediate: number = 42;
let test8: number = (intermediate = intermediate + 8);
console.log(test8); // Should be 50
console.log(intermediate); // Should be 50

// Test 9: Complex boolean logic with parentheses
let test9: boolean =
	(base_value > 0 && base_value < 100) || (base_value === 0 && false);
console.log(test9); // Should be true

// Test 10: Mathematical expressions
let test10: number = base_value * 2 + base_value / 2 - (base_value % 3);
console.log(test10); // Should be 24 (20 + 5 - 1)

// Test 11: Mixed operators with different precedence
let test11: number = base_value + 5 * 2 - base_value / 2;
console.log(test11); // Should be 15 (10 + 10 - 5)

// Test 12: Boolean variables in expressions
let flag1: boolean = true;
let flag2: boolean = false;

let test12: boolean = (flag1 && !flag2) || flag2;
console.log(test12); // Should be true

// Test 13: Comparison operators across lines
let test13: boolean = base_value >= 10 && base_value <= 10 && base_value != 9;
console.log(test13); // Should be true

// Test 14: Nested ternary with newlines
let test14: number = base_value > 5 ? (base_value > 8 ? 1000 : 2000) : 3000;
console.log(test14); // Should be 1000

// Test 15: Assignment within conditional expressions
let test15: boolean = true;
let test15_result: number = test15 ? 1 : 0;
console.log(test15_result); // Should be 1

// Test 16: Very long expression across many lines
let test16: number =
	base_value + base_value * 2 + base_value / 2 + base_value - 3 + 5;
console.log(test16); // Should be 47 (10 + 20 + 5 + 10 - 3 + 5)

// Test 17: Boolean operators with short-circuit evaluation
let test17: boolean = true || (false && true);
console.log(test17); // Should be true

// Test 18: Multiple assignments in sequence
let seq1: number = 10;

let seq2: number = seq1 + 5;

let seq3: number = seq2 * 2;

console.log(seq3); // Should be 30

// Test 19: Complex condition for if statement
if ((base_value > 0 && base_value === 10) || false) {
	console.log(9001); // Should print
}

// Test 20: Edge case with only newlines and spaces
let test20: number = 42;

console.log(test20); // Should be 42

console.log(9999); // Test completion marker
