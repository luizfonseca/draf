// Simple template literal test to isolate interpolation issues

// Test 1: Simple template literal without interpolation
let simpleTemplate: string = `Hello World`;
console.log(simpleTemplate);

// Test 2: Template literal with simple variable interpolation
let name: string = "Alice";
let greeting: string = `Hello, ${name}!`;
console.log(greeting);

// Test 3: Template literal with number interpolation
let count: number = 42;
let countMessage: string = `Count is: ${count}`;
console.log(countMessage);

// Test 4: Template literal with boolean interpolation
let isReady: boolean = true;
let readyMessage: string = `System ready: ${isReady}`;
console.log(readyMessage);

// Test 5: Template literal with expression interpolation
let x: number = 10;
let y: number = 5;
let mathResult: string = `Result: ${x + y}`;
console.log(mathResult);

// Test 6: Template literal with comparison expression
let comparison: string = `Is x greater than y? ${x > y}`;
console.log(comparison);
