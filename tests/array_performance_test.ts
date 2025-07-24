// Performance test for array optimizations in Draf TypeScript compiler

// Test 1: Constant index access optimization
let numbers = [10, 20, 30, 40, 50];

console.log("=== ARRAY PERFORMANCE OPTIMIZATIONS TEST ===");
console.log("Testing constant index access optimization");

// These should be optimized with compile-time bounds checking
let first = numbers[0]; // Valid constant index
let second = numbers[1]; // Valid constant index
let third = numbers[2]; // Valid constant index
let fourth = numbers[3]; // Valid constant index
let fifth = numbers[4]; // Valid constant index

console.log("Constant access 0:", first);
console.log("Constant access 1:", second);
console.log("Constant access 2:", third);
console.log("Constant access 3:", fourth);
console.log("Constant access 4:", fifth);

// Test 2: Array literal length optimization
console.log("Testing array literal length optimization");

// These should return constant values without memory access
let literalLength1 = [1, 2, 3].length;
let literalLength2 = [10, 20, 30, 40].length;
let literalLength3 = ["a", "b", "c", "d", "e"].length;
let emptyLength = [].length;

console.log("Literal length [1,2,3]:", literalLength1);
console.log("Literal length [10,20,30,40]:", literalLength2);
console.log("Literal length ['a','b','c','d','e']:", literalLength3);
console.log("Empty array length []:", emptyLength);

// Test 3: Empty array optimization
console.log("Testing empty array optimization");

let empty1 = [];
let empty2 = [];
let empty3 = [];

let emptyLen1 = empty1.length;
let emptyLen2 = empty2.length;
let emptyLen3 = empty3.length;

console.log("Empty array 1 length:", emptyLen1);
console.log("Empty array 2 length:", emptyLen2);
console.log("Empty array 3 length:", emptyLen3);

// Test 4: Bounds checking optimization
console.log("Testing bounds checking optimization");

// Valid indices (should be optimized)
let validAccess1 = numbers[0];
let validAccess2 = numbers[4];

// Invalid constant indices (should be optimized out)
let invalidAccess1 = numbers[10]; // Out of bounds
let invalidAccess2 = numbers[100]; // Way out of bounds

console.log("Valid access [0]:", validAccess1);
console.log("Valid access [4]:", validAccess2);
console.log("Invalid access [10]:", invalidAccess1);
console.log("Invalid access [100]:", invalidAccess2);

// Test 5: Type-specialized operations
console.log("Testing type-specialized operations");

// Homogeneous arrays should get specialized operations
let intArray = [1, 2, 3, 4, 5];
let floatArray = [1.1, 2.2, 3.3, 4.4, 5.5];
let stringArray = ["hello", "world", "test", "performance"];
let boolArray = [true, false, true, false];

let intLen = intArray.length;
let floatLen = floatArray.length;
let stringLen = stringArray.length;
let boolLen = boolArray.length;

console.log("Int array length:", intLen);
console.log("Float array length:", floatLen);
console.log("String array length:", stringLen);
console.log("Bool array length:", boolLen);

// Test 6: Method call optimization
console.log("Testing method call optimization");

// These should benefit from optimized array structure access
let hasElements1 = intArray.includes();
let hasElements2 = stringArray.some();
let allElements1 = intArray.every();
let allElements2 = boolArray.every();

console.log("Int array has elements:", hasElements1);
console.log("String array some test:", hasElements2);
console.log("Int array every test:", allElements1);
console.log("Bool array every test:", allElements2);

// Test 7: Individual access optimization
console.log("Testing individual access optimization");

// Multiple individual accesses that could be optimized
let firstNum = numbers[0];
let secondNum = numbers[1];
let thirdNum = numbers[2];

console.log("First number:", firstNum);
console.log("Second number:", secondNum);
console.log("Third number:", thirdNum);

// Test 8: Memory allocation optimization
console.log("Testing memory allocation patterns");

// These should demonstrate different allocation strategies
let smallArray = [1, 2]; // Small allocation
let mediumArray = [1, 2, 3, 4, 5, 6, 7, 8]; // Medium allocation
let emptyThenUsed = []; // Empty then potential growth

let smallLen = smallArray.length;
let mediumLen = mediumArray.length;
let emptyLen = emptyThenUsed.length;

console.log("Small array length:", smallLen);
console.log("Medium array length:", mediumLen);
console.log("Empty-then-used length:", emptyLen);

// Test 9: Expression optimization in array context
console.log("Testing expression optimization");

// These expressions should be optimized when possible
let expressionLength = numbers.length > 0;
let expressionAccess = numbers[0] > 0;

console.log("Expression with length:", expressionLength);
console.log("Expression with access:", expressionAccess);

// Test 10: Performance comparison indicators
console.log("Testing performance indicators");

// Simple operations that show optimization effectiveness
let firstElement = numbers[0];
let secondElement = numbers[1];
let arrayLength = numbers.length;

console.log("First element:", firstElement);
console.log("Second element:", secondElement);
console.log("Array length:", arrayLength);

console.log("=== ARRAY PERFORMANCE OPTIMIZATIONS COMPLETED ===");
