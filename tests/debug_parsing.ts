// Debug the exact parsing of the problematic expression
// Breaking down "Value: " + 42 + ", Active: " + true + ", Extra: " + 3.14

// Test 1: Just the first part
let part1: string = "Value: " + 42;
console.log(part1);

// Test 2: First two parts
let part2: string = "Value: " + 42 + ", Active: ";
console.log(part2);

// Test 3: First three parts (this might be where it breaks)
let part3: string = "Value: " + 42 + ", Active: " + true;
console.log(part3);

// Test 4: The last two parts separately
let part4: string = ", Active: " + true;
console.log(part4);

// Test 5: The last three parts
let part5: string = ", Active: " + true + ", Extra: ";
console.log(part5);

// Test 6: The very last part
let part6: string = ", Extra: " + 3.14;
console.log(part6);

// Test 7: Just numbers and boolean (this is likely the issue)
let directCombination: string = 42 + true + 3.14;
console.log(directCombination);

// Test 8: Different grouping with parentheses
let grouped: string =
	"Value: " + 42 + (", Active: " + true) + (", Extra: " + 3.14);
console.log(grouped);

// Test 9: Step by step assignment
let step1: string = "Value: " + 42;
let step2: string = step1 + ", Active: ";
let step3: string = step2 + true;
let step4: string = step3 + ", Extra: ";
let step5: string = step4 + 3.14;
console.log(step5);

// Test 10: The issue might be associativity - let's test right-to-left grouping
let rightAssoc: string =
	"Value: " + (42 + ", Active: " + (true + ", Extra: " + 3.14));
console.log(rightAssoc);
