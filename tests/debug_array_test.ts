// Debug array test to identify the problematic part

// Test sections from comprehensive test one by one

// Section 1: Basic array literals (known to work)
let numbers = [1, 2, 3, 4, 5];
let strings = ["hello", "world", "test"];
let booleans = [true, false, true];

console.log("=== SECTION 1: BASIC LITERALS ===");
console.log("Basic literals work");

// Section 2: Array indexing (known to work)
let firstNumber = numbers[0];
let secondString = strings[1];

console.log("=== SECTION 2: INDEXING ===");
console.log("Indexing works");

// Section 3: Mixed arrays (potential issue)
let mixed = [1, "hello", true];

console.log("=== SECTION 3: MIXED ARRAYS ===");
console.log("Mixed array created");

// Section 4: Array.isArray (known to work)
let isArray1 = Array.isArray(numbers);

console.log("=== SECTION 4: ARRAY.isArray ===");
console.log("isArray works");

// Section 5: Array.of (potential issue)
let fromOf1 = Array.of(1, 2, 3);

console.log("=== SECTION 5: ARRAY.of ===");
console.log("Array.of works");

// Section 6: Complex expressions (potential issue)
let complexArray = [1 + 2, 3 * 4, 5 - 1];

console.log("=== SECTION 6: COMPLEX EXPRESSIONS ===");
console.log("Complex expressions work");

// Section 7: Nested arrays (likely problematic)
let nestedArray = [
	[1, 2],
	[3, 4],
];

console.log("=== SECTION 7: NESTED ARRAYS ===");
console.log("Nested arrays work");

// Section 8: Arrays with global calls (potential issue)
let withGlobals = [Number.MAX_VALUE, Date.now()];

console.log("=== SECTION 8: ARRAYS WITH GLOBALS ===");
console.log("Arrays with globals work");

// Section 9: Multiline arrays (potential formatting issue)
let multilineNumbers = [1, 2, 3];

console.log("=== SECTION 9: MULTILINE ARRAYS ===");
console.log("Multiline arrays work");

console.log("=== DEBUG TEST COMPLETED ===");
