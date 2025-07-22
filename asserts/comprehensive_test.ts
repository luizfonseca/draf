// Comprehensive test for all new features: if/else, ternary, strict equality, nullish coalescing
let x: number = 10;
let y: number = 5;
let z: number = 10;

console.log("=== IF/ELSE STATEMENTS ===");

// Basic if statement
if (x > y) {
    console.log("x is greater than y");
}

// If-else statement
if (x < y) {
    console.log("This should not print");
} else {
    console.log("x is not less than y");
}

// If-else if-else chain
let score: number = 85;
if (score >= 90) {
    console.log("Grade A");
} else if (score >= 80) {
    console.log("Grade B - this should print");
} else if (score >= 70) {
    console.log("Grade C");
} else {
    console.log("Grade F");
}

// Boolean conditions
let isReady: boolean = true;
let isComplete: boolean = false;

if (isReady) {
    console.log("System is ready");
}

if (!isComplete) {
    console.log("System is not complete");
}

// Complex boolean expressions
let age: number = 25;
let hasLicense: boolean = true;

if (age >= 18 && hasLicense) {
    console.log("Can drive");
}

console.log("=== TERNARY OPERATORS ===");

// Simple ternary
let result1: number = x > y ? 1 : 0;
console.log(result1); // Should be 1

// Ternary with expressions
let result2: number = x > y ? x + y : x - y;
console.log(result2); // Should be 15

// Nested ternary
let grade: number = score >= 90 ? 4 : score >= 80 ? 3 : score >= 70 ? 2 : 1;
console.log(grade); // Should be 3

// Boolean ternary
let status: number = isReady ? 1 : 0;
console.log(status); // Should be 1

// Ternary in expression
console.log(x > y ? 100 : 200); // Should be 100

// Complex condition in ternary
let canDrive: number = age >= 18 && hasLicense ? 1 : 0;
console.log(canDrive); // Should be 1

console.log("=== STRICT EQUALITY ===");

// Strict equality with numbers
console.log(x === z); // Should be true (10 === 10)
console.log(x === y); // Should be false (10 === 5)

// Strict inequality with numbers
console.log(x !== y); // Should be true (10 !== 5)
console.log(x !== z); // Should be false (10 !== 10)

// Regular vs strict equality
console.log(x == z);  // Should be true
console.log(x === z); // Should be true

// Boolean strict equality
console.log(isReady === true);    // Should be true
console.log(isComplete === false); // Should be true
console.log(isReady === false);   // Should be false
console.log(isReady !== false);   // Should be true

// Strict equality in conditionals
if (x === z) {
    console.log("x strictly equals z");
}

// Strict equality in ternary
let strictResult: number = x === z ? 1 : 0;
console.log(strictResult); // Should be 1

console.log("=== NULLISH COALESCING ===");

// Nullish coalescing with numbers (simplified implementation)
let value1: number = 0 ?? 99;
console.log(value1); // Should be 99 (right operand)

let value2: number = 15 ?? 99;
console.log(value2); // Should be 99 (right operand)

// Nullish coalescing in expressions
let defaultValue: number = x ?? 42;
console.log(defaultValue); // Should be 42 (right operand)

console.log("=== LOGICAL OR ===");

// Logical OR with booleans
let boolResult1: boolean = false || true;
console.log(boolResult1); // Should be true

let boolResult2: boolean = true || false;
console.log(boolResult2); // Should be true

let boolResult3: boolean = false || false;
console.log(boolResult3); // Should be false

console.log("=== COMPLEX COMBINATIONS ===");

// Complex if with ternary and strict equality
if ((x === z ? true : false) && isReady) {
    console.log("Complex condition met");
}

// Nested ternary with strict equality
let complexResult: number = x === z ? (isReady ? 1 : 0) : (isComplete ? 2 : 3);
console.log(complexResult); // Should be 1

// If-else with multiple conditions
if (x === z && isReady || age >= 18) {
    console.log("Multiple condition check passed");
}

// Ternary chain
let finalResult: number = x > y ? (x === z ? 10 : 20) : (y > 0 ? 30 : 40);
console.log(finalResult); // Should be 10

console.log("=== ALL TESTS COMPLETE ===");
