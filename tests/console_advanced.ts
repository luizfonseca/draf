// Advanced console output tests for strongly typed TypeScript compiler
// Testing different data types, edge cases, and complex expressions

// Test 1: Basic numeric types and precision
let int_val: number = 42;
let float_val: number = 3.14159265359;
let negative: number = -123;
let zero: number = 0;

console.log(int_val);
console.log(float_val);
console.log(negative);
console.log(zero);

// Test 2: Boolean values
let true_val: boolean = true;
let false_val: boolean = false;
let comparison_result: boolean = 10 > 5;
let equality_result: boolean = 42 == 42;

console.log(true_val);
console.log(false_val);
console.log(comparison_result);
console.log(equality_result);

// Test 3: Mathematical expressions with different operations
let a: number = 15;
let b: number = 4;

console.log(a + b); // Addition
console.log(a - b); // Subtraction
console.log(a * b); // Multiplication
console.log(a / b); // Division

// Test 4: Complex nested expressions
let complex1: number = (a + b) * 2 - 10;
let complex2: number = a * b + (a - b) / 2;
let complex3: boolean = a > b && b > 0;

console.log(complex1);
console.log(complex2);
console.log(complex3);

// Test 5: Different console methods with various data types
console.info(42);
console.warn(true);
console.error(3.14);
console.debug(false);

// Test 6: Multiple arguments with mixed types
console.log(1, 2, 3, 4, 5);
console.log(true, false, true);
console.log(10, 20.5, 30);
console.info(100, 200, 300);
console.warn(1.1, 2.2, 3.3);

// Test 7: Variable combinations
let x: number = 10;
let y: number = 20;
let z: boolean = true;

console.log(x, y);
console.log(x, z);
console.log(y, z);
console.log(x, y, z);

// Test 8: More complex mathematical expressions
let math1: number = x + y * 2;
let math2: number = (x - y) / 2 + 5;
let math3: number = x * y - (a + b);

console.log(math1);
console.log(math2);
console.log(math3);

// Test 9: Edge cases with numbers
let very_large: number = 999999;
let very_small: number = 0.000001;
let fraction: number = 1 / 3;

console.log(very_large);
console.log(very_small);
console.log(fraction);

// Test 10: Logical operations
let and_result: boolean = true && true;
let or_result: boolean = false || true;
let not_result: boolean = !false;

console.log(and_result);
console.log(or_result);
console.log(not_result);

// Test 11: Comparison operations
console.log(10 == 10);
console.log(10 != 5);
console.log(15 > 10);
console.log(5 < 10);
console.log(10 >= 10);
console.log(9 <= 10);

// Test 12: Nested boolean expressions
let complex_bool1: boolean = a > b && x < y;
let complex_bool2: boolean = (true || false) && true;
let complex_bool3: boolean = !(false && true) || false;

console.log(complex_bool1);
console.log(complex_bool2);
console.log(complex_bool3);

// Test 13: Mathematical constants and operations
let pi: number = 3.14159;
let e: number = 2.71828;
let circle_area: number = pi * 5 * 5;
let compound: number = pi + e - 1;

console.log(pi);
console.log(e);
console.log(circle_area);
console.log(compound);

// Test 14: Precedence testing
let precedence1: number = 2 + 3 * 4;
let precedence2: number = (2 + 3) * 4;
let precedence3: boolean = 5 > 3 && 2 < 4;
let precedence4: boolean = 5 > 3 || 2 > 4;

console.log(precedence1);
console.log(precedence2);
console.log(precedence3);
console.log(precedence4);

// Test 15: Negative number expressions
let neg1: number = -x;
let neg2: number = -(-y);
let neg3: number = -(a + b);

console.log(neg1);
console.log(neg2);
console.log(neg3);

// Test 16: Zero and infinity cases
let zero_div: number = 0 / 5;
let infinity_case: number = 5 / 0;

console.log(zero_div);
console.log(infinity_case);

// Test 17: Final comprehensive test
let final_num: number = ((a + b) * 2) / (a - b + 1);
let final_bool: boolean = final_num > 0 && a != b;

console.log(final_num);
console.log(final_bool);
console.info(999);
