// Simple function execution test for Draf TypeScript compiler

console.log("=== SIMPLE FUNCTION TEST ===");

// Test 1: Basic function without parameters
function sayHello() {
	console.log("Hello from function!");
}

sayHello();

// Test 2: Function with one parameter
function greet(name: string) {
	console.log("Hello", name);
}

greet("World");

// Test 3: Function with return value
function add(a: number, b: number): number {
	return a + b;
}

let result: number = add(5, 3);
console.log("Result:", result);

// Test 4: Function with boolean return
function isPositive(x: number): boolean {
	return x > 0;
}

let positive: boolean = isPositive(10);
console.log("Is positive:", positive);

console.log("=== SIMPLE FUNCTION TEST COMPLETE ===");
