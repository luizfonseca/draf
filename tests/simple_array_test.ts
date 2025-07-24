// Simple array test to isolate issues

// Test 1: Basic array literal creation
let numbers = [1, 2, 3];
let strings = ["hello", "world"];
let empty = [];

console.log("Arrays created successfully");

// Test 2: Array access
let first = numbers[0];
let second = numbers[1];

console.log("Array access works");

// Test 3: Array.isArray
let check1 = Array.isArray(numbers);
let check2 = Array.isArray("not array");

console.log("Array.isArray works");

// Test 4: Simple array operations
let value1 = first + second;
let value2 = numbers[2];

console.log("Array operations work");

console.log("Simple array test completed");
