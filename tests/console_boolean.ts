// Boolean console output test for strongly typed TypeScript compiler
// This test focuses specifically on boolean values and their console output

// Basic boolean literals
let true_val: boolean = true;
let false_val: boolean = false;

console.log(true_val);
console.log(false_val);

// Boolean expressions
let expr1: boolean = 10 > 5;
let expr2: boolean = 3 < 2;
let expr3: boolean = 42 == 42;
let expr4: boolean = 15 != 15;

console.log(expr1);
console.log(expr2);
console.log(expr3);
console.log(expr4);

// Logical operations
let and_true: boolean = true && true;
let and_false: boolean = true && false;
let or_true: boolean = false || true;
let or_false: boolean = false || false;

console.log(and_true);
console.log(and_false);
console.log(or_true);
console.log(or_false);

// Negation
let not_true: boolean = !true;
let not_false: boolean = !false;

console.log(not_true);
console.log(not_false);

// Complex boolean expressions
let complex1: boolean = 10 > 5 && 20 < 30;
let complex2: boolean = 5 > 10 || 15 == 15;
let complex3: boolean = !(false && true);

console.log(complex1);
console.log(complex2);
console.log(complex3);

// Different console methods with booleans
console.info(true);
console.warn(false);
console.error(true);
console.debug(false);

// Mixed boolean and number arguments
console.log(true, 42);
console.log(false, 3.14);
console.log(100, true, 200);

// Variables in expressions
let x: number = 15;
let y: number = 10;
let comparison: boolean = x > y;

console.log(comparison);
console.log(x > y);
console.log(x == y);
console.log(x != y);
