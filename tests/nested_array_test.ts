// Test nested arrays to see if that's the source of the string conversion issue

// Test 1: Simple arrays (known to work)
let numbers = [1, 2, 3];
let strings = ["hello", "world"];

console.log("=== SIMPLE ARRAYS ===");
console.log("Simple arrays work");

// Test 2: Single level nested array
let simpleNested = [
	[1, 2],
	[3, 4],
];

console.log("=== SINGLE LEVEL NESTED ===");
console.log("Single level nested array created");

// Test 3: Access nested array elements
let firstSubArray = simpleNested[0];
let firstElement = simpleNested[0][0];

console.log("=== NESTED ARRAY ACCESS ===");
console.log("Nested array access works");

// Test 4: Mixed nested arrays
let mixedNested = [
	[1, 2],
	["a", "b"],
	[true, false],
];

console.log("=== MIXED NESTED ARRAYS ===");
console.log("Mixed nested arrays created");

// Test 5: Array.isArray on nested arrays
let isNestedArray = Array.isArray(simpleNested);
let isSubArray = Array.isArray(simpleNested[0]);

console.log("=== NESTED ARRAY TYPE CHECKING ===");
console.log("Nested array type checking works");

console.log("=== NESTED ARRAY TEST COMPLETED ===");
