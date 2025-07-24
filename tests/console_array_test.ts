// Test console.log statements with array values to find the string conversion issue

// Basic arrays
let numbers = [1, 2, 3];
let strings = ["hello", "world"];
let mixed = [1, "hello", true];

console.log("=== BASIC CONSOLE TESTS ===");

// Test 1: Console.log with simple messages (known to work)
console.log("Arrays created");

// Test 2: Console.log with array element access (potential issue)
console.log("First number:", numbers[0]);

// Test 3: Console.log with string array element
console.log("First string:", strings[0]);

// Test 4: Console.log with mixed array elements
console.log("Mixed number:", mixed[0]);
console.log("Mixed string:", mixed[1]);
console.log("Mixed boolean:", mixed[2]);

// Test 5: Console.log with expressions involving arrays
let sum = numbers[0] + numbers[1];
console.log("Sum result:", sum);

// Test 6: Console.log with global methods and arrays
let maxValue = Number.MAX_VALUE;
console.log("Max value:", maxValue);

// Test 7: Console.log with Array methods
let isArrayResult = Array.isArray(numbers);
console.log("Is array result:", isArrayResult);

// Test 8: Console.log with multiple array accesses
console.log("Multiple values:", numbers[0], numbers[1]);

console.log("=== CONSOLE ARRAY TEST COMPLETED ===");
