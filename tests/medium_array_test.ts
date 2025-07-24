// Medium complexity array test to debug string conversion issues

// Test 1: Simple array literals (these work)
let numbers = [1, 2, 3];
let strings = ["hello", "world"];

console.log("=== BASIC ARRAYS ===");
console.log("Number array created");
console.log("String array created");

// Test 2: Array access (check if this works)
let firstNumber = numbers[0];
let firstString = strings[0];

console.log("=== ARRAY ACCESS ===");
console.log("First number accessed");
console.log("First string accessed");

// Test 3: Array access in expressions (potential issue)
let sum = numbers[0] + numbers[1];

console.log("=== ARRAY EXPRESSIONS ===");
console.log("Sum calculated");

// Test 4: Mixed type arrays (potential issue)
let mixed = [1, "hello"];

console.log("=== MIXED ARRAYS ===");
console.log("Mixed array created");

// Test 5: Mixed array access (likely problematic)
let mixedNumber = mixed[0];
let mixedString = mixed[1];

console.log("=== MIXED ARRAY ACCESS ===");
console.log("Mixed number accessed");
console.log("Mixed string accessed");

// Test 6: Array.isArray with different types
let check1 = Array.isArray(numbers);
let check2 = Array.isArray(mixed);

console.log("=== ARRAY TYPE CHECKING ===");
console.log("Type checking completed");

// Test 7: Console.log with array elements directly (potential issue)
console.log("Direct number:", numbers[0]);

console.log("=== MEDIUM ARRAY TEST COMPLETED ===");
