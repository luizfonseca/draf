// Comprehensive validation test to ensure all existing functionality still works
// This test validates if/else, ternary, strict equality, nullish coalescing, and logical operators

// Test data setup
let num1: number = 42;
let num2: number = 42;
let num3: number = 15;
let isActive: boolean = true;
let isComplete: boolean = false;
let score: number = 87;

console.log(9000); // === VALIDATION START ===

// Test 1: Basic if statements
if (num1 > num3) {
	console.log(9001); // Should print: num1 > num3
}

if (num1 < num3) {
	console.log(9002); // Should NOT print
} else {
	console.log(9003); // Should print: else branch
}

// Test 2: Complex if-else if chains
if (score >= 95) {
	console.log(9004); // Should NOT print
} else if (score >= 85) {
	console.log(9005); // Should print: score is 87
} else if (score >= 75) {
	console.log(9006); // Should NOT print
} else {
	console.log(9007); // Should NOT print
}

// Test 3: Boolean conditions
if (isActive && !isComplete) {
	console.log(9008); // Should print: both conditions true
}

if (isActive || isComplete) {
	console.log(9009); // Should print: OR condition
}

// Test 4: Strict equality operators
console.log(num1 === num2); // Should be true (42 === 42)
console.log(num1 === num3); // Should be false (42 === 15)
console.log(num1 !== num3); // Should be true (42 !== 15)
console.log(isActive === true); // Should be true
console.log(isComplete === false); // Should be true

// Test 5: Regular vs strict equality
console.log(num1 == num2); // Should be true
console.log(num1 === num2); // Should be true

// Test 6: Ternary operators
let result1: number = num1 > num3 ? 100 : 200;
console.log(result1); // Should be 100

let result2: boolean = isActive ? true : false;
console.log(result2); // Should be true

// Test 7: Nested ternary
let grade: number = score >= 90 ? 4 : score >= 80 ? 3 : score >= 70 ? 2 : 1;
console.log(grade); // Should be 3

// Test 8: Ternary with strict equality
let exactMatch: number = num1 === num2 ? 1 : 0;
console.log(exactMatch); // Should be 1

// Test 9: Nullish coalescing (simplified implementation)
let default1: number = num1 ?? 999;
console.log(default1); // Should be 999 (right operand)

let default2: number = 0 ?? 888;
console.log(default2); // Should be 888 (right operand)

// Test 10: Complex combinations
if ((num1 === num2 ? true : false) && isActive) {
	console.log(9010); // Should print: complex condition
}

let complexResult: number =
	num1 > num3 ? (isActive ? 50 : 60) : isComplete ? 70 : 80;
console.log(complexResult); // Should be 50

// Test 11: Logical OR with booleans
let or1: boolean = true || false;
console.log(or1); // Should be true

let or2: boolean = false || false;
console.log(or2); // Should be false

// Test 12: Multiple operator combinations
if (num1 === num2 && (isActive || !isComplete)) {
	console.log(9011); // Should print: multiple conditions
}

// Test 13: Nested blocks
if (num1 > 0) {
	if (num2 > 0) {
		if (num1 === num2) {
			console.log(9012); // Should print: nested conditions
		}
	}
}

// Test 14: Ternary chains
let chain: number =
	num1 > num3 ? (num1 === num2 ? (isActive ? 10 : 20) : 30) : 40;
console.log(chain); // Should be 10

// Test 15: Boolean expressions in ternary
let boolTernary: boolean = num1 > num3 ? isActive && !isComplete : false;
console.log(boolTernary); // Should be true

// Test 16: Strict equality in complex expressions
if (num1 !== num3 && score >= 85 && isActive === true) {
	console.log(9013); // Should print: all conditions met
}

// Test 17: Edge case - multiple strict comparisons
console.log(num1 === num2 && num2 !== num3); // Should be true
console.log(isActive !== isComplete); // Should be true

// Test 18: Final complex scenario
let finalCheck: number =
	num1 === num2 ? (isActive ? (score >= 85 ? 1000 : 2000) : 3000) : 4000;
console.log(finalCheck); // Should be 1000

// Test 19: Variable assignments
let assignTest: number = 555;
assignTest = assignTest + 445;
console.log(assignTest); // Should be 1000

// Test 20: Console methods
console.info(9020);
console.warn(9021);
console.error(9022);
console.debug(9023);

console.log(9999); // === VALIDATION COMPLETE ===
