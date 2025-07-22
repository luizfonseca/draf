// Test with null coalescing context to see if it affects string concatenation

// Test 1: Null coalescing (this might not be implemented)
let nullValue: string = null ?? "Default string";
console.log(nullValue);

// Test 2: The problematic line that follows in comprehensive test
let mixed: string = "Value: " + 42 + ", Active: " + true + ", Extra: " + 3.14;
console.log(mixed);

// Test 3: Same line without null coalescing context
let simpleMixed: string =
	"Value: " + 42 + ", Active: " + true + ", Extra: " + 3.14;
console.log(simpleMixed);

// Test 4: Just the null coalescing to see if it compiles
let testNull: string = null ?? "test";
console.log(testNull);
