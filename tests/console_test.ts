// Console output test for strongly typed TypeScript compiler
// This example tests console.log, console.info, console.warn, console.error

// Basic console.log with numbers
console.log(42);
console.log(3.14159);

// Multiple arguments
console.log(1, 2, 3);

// Different console methods
console.info(100);
console.warn(200);
console.error(404);
console.debug(123);

// With variables
let x: number = 10;
let y: number = 20;
let sum: number = x + y;

console.log(sum);
console.info(x);
console.warn(y);

// Complex expressions
console.log(x + y);
console.log(x * y);
console.log(x - y);

// Boolean results
let isGreater: boolean = x > y;
console.log(isGreater);

// Mathematical operations
let result: number = (x + y) * 2 - 5;
console.log(result);

// Final message
console.info(999);
