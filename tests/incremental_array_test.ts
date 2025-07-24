// Incremental test to find the exact issue in comprehensive array test
// Adding sections one by one to isolate the problem

// Test 1: Array literal creation
let numbers = [1, 2, 3, 4, 5];
let strings = ["hello", "world", "test"];
let booleans = [true, false, true];
let mixed = [1, "hello", true];
let empty = [];

console.log("=== ARRAY LITERAL CREATION ===");
console.log("Numbers array created");
console.log("Strings array created");
console.log("Booleans array created");
console.log("Mixed array created");
console.log("Empty array created");

// Test 2: Array indexing/access
let firstNumber = numbers[0];
let secondString = strings[1];
let thirdBoolean = booleans[2];
let mixedFirst = mixed[0];

console.log("=== ARRAY INDEXING ===");
console.log("First number:", firstNumber);
console.log("Second string:", secondString);
console.log("Third boolean:", thirdBoolean);
console.log("Mixed first element:", mixedFirst);

// Test 3: Array.isArray static method
let isArray1 = Array.isArray(numbers);
let isArray2 = Array.isArray("not an array");
let isArray3 = Array.isArray([1, 2, 3]);
let isArray4 = Array.isArray(42);

console.log("=== ARRAY.isArray TESTS ===");
console.log("isArray(numbers):", isArray1);
console.log("isArray('not an array'):", isArray2);
console.log("isArray([1, 2, 3]):", isArray3);
console.log("isArray(42):", isArray4);

// Test 4: Array.of static method
let fromOf1 = Array.of(1, 2, 3);
let fromOf2 = Array.of("a", "b", "c");
let fromOf3 = Array.of(true, false);
let fromOf4 = Array.of();

console.log("=== ARRAY.of TESTS ===");
console.log("Array.of(1, 2, 3) created");
console.log("Array.of('a', 'b', 'c') created");
console.log("Array.of(true, false) created");
console.log("Array.of() created");

// Test 5: Array.from static method
let fromArray1 = Array.from([1, 2, 3]);
let fromArray2 = Array.from("hello");

console.log("=== ARRAY.from TESTS ===");
console.log("Array.from([1, 2, 3]) created");
console.log("Array.from('hello') created");

console.log("=== INCREMENTAL TEST COMPLETED SUCCESSFULLY ===");
