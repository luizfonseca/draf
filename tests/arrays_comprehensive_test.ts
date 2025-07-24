// Comprehensive test for Array functionality in Draf TypeScript compiler

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

// Test 6: Array with various data types
let numberArray = [10, 20, 30, 40, 50];
let stringArray = ["apple", "banana", "cherry"];
let booleanArray = [true, false, true, false];
let nestedArray = [
	[1, 2],
	[3, 4],
	[5, 6],
];

console.log("=== TYPED ARRAYS ===");
console.log("Number array with 5 elements");
console.log("String array with fruits");
console.log("Boolean array alternating");
console.log("Nested array of arrays");

// Test 7: Array access with variables
let index0 = 0;
let index1 = 1;
let index2 = 2;

let elem0 = numberArray[index0];
let elem1 = stringArray[index1];
let elem2 = booleanArray[index2];

console.log("=== DYNAMIC ARRAY ACCESS ===");
console.log("numberArray[0]:", elem0);
console.log("stringArray[1]:", elem1);
console.log("booleanArray[2]:", elem2);

// Test 8: Array expressions in other contexts
let sum = numberArray[0] + numberArray[1];
let concatenated = stringArray[0] + " " + stringArray[1];
let logicalResult = booleanArray[0] && booleanArray[1];

console.log("=== ARRAY ELEMENTS IN EXPRESSIONS ===");
console.log("Sum of first two numbers:", sum);
console.log("Concatenated strings:", concatenated);
console.log("Logical AND of first two booleans:", logicalResult);

// Test 9: Array assignment and updates
let mutableArray = [1, 2, 3];
let firstElement = mutableArray[0];
let lastElement = mutableArray[2];

console.log("=== ARRAY ASSIGNMENT ===");
console.log("First element before:", firstElement);
console.log("Last element before:", lastElement);

// Test 10: Complex array expressions
let complexArray = [1 + 2, 3 * 4, 5 - 1];
let mathResult1 = complexArray[0];
let mathResult2 = complexArray[1];
let mathResult3 = complexArray[2];

console.log("=== COMPLEX ARRAY EXPRESSIONS ===");
console.log("1 + 2 =", mathResult1);
console.log("3 * 4 =", mathResult2);
console.log("5 - 1 =", mathResult3);

// Test 11: Array with function calls
let withGlobals = [Number.MAX_VALUE, Date.now(), Number.parseFloat("3.14")];
let maxValue = withGlobals[0];
let timestamp = withGlobals[1];
let parsed = withGlobals[2];

console.log("=== ARRAYS WITH GLOBAL FUNCTION CALLS ===");
console.log("MAX_VALUE in array:", maxValue);
console.log("Date.now() in array:", timestamp);
console.log("parseFloat result in array:", parsed);

// Test 12: Multiline array literals
let multilineNumbers = [1, 2, 3, 4, 5];

let multilineStrings = ["first", "second", "third"];

console.log("=== MULTILINE ARRAYS ===");
console.log("Multiline numbers array created");
console.log("Multiline strings array created");

// Test 13: Array type checking scenarios
let checkArray1 = Array.isArray(multilineNumbers);
let checkArray2 = Array.isArray(withGlobals);
let checkArray3 = Array.isArray(nestedArray);

console.log("=== ARRAY TYPE CHECKING ===");
console.log("multilineNumbers is array:", checkArray1);
console.log("withGlobals is array:", checkArray2);
console.log("nestedArray is array:", checkArray3);

// Test 14: Array access edge cases
let edgeArray = [100, 200, 300];
let validAccess = edgeArray[1];
let firstAccess = edgeArray[0];

console.log("=== ARRAY ACCESS EDGE CASES ===");
console.log("Valid access [1]:", validAccess);
console.log("First access [0]:", firstAccess);

// Test 15: Arrays in conditional expressions
let conditionalArray = [true, false, true];
let condition1 = conditionalArray[0] && conditionalArray[2];
let condition2 = conditionalArray[1] || conditionalArray[0];

console.log("=== ARRAYS IN CONDITIONALS ===");
console.log("First AND third:", condition1);
console.log("Second OR first:", condition2);

console.log("=== ALL ARRAY TESTS COMPLETED SUCCESSFULLY ===");
