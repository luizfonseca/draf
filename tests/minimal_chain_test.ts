// Minimal test to isolate chained concatenation issue

// Test 1: Simple string + number (should work)
let test1: string = "Value: " + 42;
console.log(test1);

// Test 2: String + number + string (should work)
let test2: string = "Value: " + 42 + " End";
console.log(test2);

// Test 3: String + number + boolean (this might fail)
let test3: string = "Value: " + 42 + true;
console.log(test3);

// Test 4: The problematic case from comprehensive test
let test4: string = "Value: " + 42 + ", Active: " + true;
console.log(test4);

// Test 5: Step by step breakdown
let step1: string = "Value: " + 42;
let step2: string = step1 + ", Active: ";
let step3: string = step2 + true;
console.log(step3);
