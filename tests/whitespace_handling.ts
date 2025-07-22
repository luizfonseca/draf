// Test for whitespace and indentation handling in multiline assignments
// This test validates that various whitespace patterns are handled correctly

let base: number = 5;

// Test 1: Spaces before newline
let test1: number = base + 10;
console.log(test1); // Should be 15

// Test 2: Tabs after assignment
let test2: number = base * 2;
console.log(test2); // Should be 10

// Test 3: Mixed tabs and spaces
let test3: number = base + base;
console.log(test3); // Should be 10

// Test 4: Multiple newlines
let test4: number = base + 5;
console.log(test4); // Should be 10

// Test 5: No indentation
let test5: number = base + 1;
console.log(test5); // Should be 6

// Test 6: Heavy indentation
let test6: number = base + 3;
console.log(test6); // Should be 8

// Test 7: Boolean expression with various whitespace
let test7: boolean = (base > 0 && true) || (false && base == 1);
console.log(test7); // Should be true

// Test 8: Ternary with whitespace
let test8: number = base > 0 ? 100 : 200;
console.log(test8); // Should be 100

// Test 9: Complex expression with mixed whitespace
let test9: number = base + 5 * 2 - 1;
console.log(test9); // Should be 14

// Test 10: Strict equality with whitespace
let test10: boolean = base === 5 && base !== 0;
console.log(test10); // Should be true

// Test 11: Nullish coalescing with whitespace
let test11: number = base ?? 999;
console.log(test11); // Should be 999 (right operand)

// Test 12: Logical OR with newlines
let test12: boolean = false || true;
console.log(test12); // Should be true

// Test 13: Parenthesized expression with whitespace
let test13: number = (base + 5) * 2;
console.log(test13); // Should be 20

// Test 14: Assignment to variable with whitespace
let intermediate: number = 42;
let test14: number = (intermediate = intermediate + 8);
console.log(test14); // Should be 50

// Test 15: Complex nested expression
let test15: boolean =
	((base > 0 && true) || (false && base == 1)) && base === 5;
console.log(test15); // Should be true

console.log(9999); // Test completion marker
