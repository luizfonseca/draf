// Test for array length property in Draf TypeScript compiler

// Test 1: Basic array length
let numbers = [1, 2, 3, 4, 5];
let numbersLength = numbers.length;

console.log("=== ARRAY LENGTH TEST ===");
console.log("Numbers array length:", numbersLength);

// Test 2: Empty array length
let emptyArray = [];
let emptyLength = emptyArray.length;
console.log("Empty array length:", emptyLength);

// Test 3: String array length
let strings = ["hello", "world", "test"];
let stringsLength = strings.length;
console.log("Strings array length:", stringsLength);

// Test 4: Mixed array length
let mixed = [1, "hello", true, null];
let mixedLength = mixed.length;
console.log("Mixed array length:", mixedLength);

// Test 5: Array literal length access
let literalLength = [10, 20, 30, 40].length;
console.log("Literal array length:", literalLength);

// Test 6: Nested array length
let nested = [
	[1, 2],
	[3, 4],
	[5, 6],
];
let nestedLength = nested.length;
console.log("Nested array length:", nestedLength);

// Test 7: Boolean array length
let booleans = [true, false, true, false, true];
let booleansLength = booleans.length;
console.log("Booleans array length:", booleansLength);

// Test 8: Single element array length
let single = [42];
let singleLength = single.length;
console.log("Single element array length:", singleLength);

// Test 9: Length in expressions
let doubleLength = numbers.length * 2;
console.log("Double length:", doubleLength);

// Test 10: Length comparison
let isLongArray = numbers.length > 3;
console.log("Is long array:", isLongArray);

console.log("=== ALL ARRAY LENGTH TESTS COMPLETED ===");
