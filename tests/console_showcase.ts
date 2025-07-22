// Comprehensive console demonstration for the Draf strongly typed TypeScript compiler
// This file showcases all implemented console functionality

// ===== SECTION 1: Basic Data Types =====

// Numbers
let integer: number = 42;
let decimal: number = 3.14159;
let negative: number = -17;
let zero: number = 0;

console.log(integer);
console.log(decimal);
console.log(negative);
console.log(zero);

// Booleans
let bool_true: boolean = true;
let bool_false: boolean = false;

console.log(bool_true);
console.log(bool_false);

// ===== SECTION 2: Mathematical Expressions =====

let a: number = 15;
let b: number = 4;

// Basic arithmetic
console.log(a + b); // Addition: 19
console.log(a - b); // Subtraction: 11
console.log(a * b); // Multiplication: 60
console.log(a / b); // Division: 3.75

// Complex expressions
let complex_math: number = (a + b) * 2 - 10;
console.log(complex_math);

// ===== SECTION 3: Boolean Expressions =====

// Comparison operations
console.log(a > b); // true
console.log(a < b); // false
console.log(a == 15); // true
console.log(a != b); // true
console.log(a >= 15); // true
console.log(b <= 5); // true

// Logical operations
let logical_and: boolean = true && true;
let logical_or: boolean = false || true;
let logical_not: boolean = !false;

console.log(logical_and); // true
console.log(logical_or); // true
console.log(logical_not); // true

// Complex boolean expressions
let complex_bool: boolean = a > b && b > 0;
console.log(complex_bool); // true

// ===== SECTION 4: Console Methods =====

// Different console methods with various data types
console.info(100);
console.warn(true);
console.error(3.14);
console.debug(false);

// Multiple arguments with mixed types
console.log(1, 2, 3);
console.log(true, false, true);
console.log(42, true, 3.14);
console.info(a, b, complex_bool);

// ===== SECTION 5: Variable Combinations =====

let x: number = 10;
let y: number = 20;
let flag: boolean = true;

console.log(x, y);
console.log(x, flag);
console.log(y, flag);
console.log(x, y, flag);

// ===== SECTION 6: Edge Cases =====

// Large numbers
let large_num: number = 999999;
let small_num: number = 0.000001;

console.log(large_num);
console.log(small_num);

// Division by zero (should produce infinity)
let infinity_test: number = 5 / 0;
console.log(infinity_test);

// Fractions
let fraction: number = 1 / 3;
console.log(fraction);

// ===== SECTION 7: Nested Expressions =====

// Mathematical precedence
let precedence1: number = 2 + 3 * 4; // Should be 14
let precedence2: number = (2 + 3) * 4; // Should be 20

console.log(precedence1);
console.log(precedence2);

// Boolean precedence
let bool_precedence1: boolean = true || (false && false); // Should be true
let bool_precedence2: boolean = (true || false) && false; // Should be false

console.log(bool_precedence1);
console.log(bool_precedence2);

// ===== SECTION 8: Unary Operations =====

let positive: number = +42;
let negated: number = -positive;
let double_negative: number = -(-42);

console.log(positive);
console.log(negated);
console.log(double_negative);

// Boolean negation
let not_true: boolean = !true;
let not_false: boolean = !false;
let double_not: boolean = !!true;

console.log(not_true);
console.log(not_false);
console.log(double_not);

// ===== SECTION 9: Final Comprehensive Test =====

// A complex expression combining multiple features
let final_number: number = ((a + b) * 2) / (a - b + 1);
let final_boolean: boolean = final_number > 0 && a != b && flag;

console.log(final_number);
console.log(final_boolean);

// Final status message
console.info(999);
console.log(true);
