// Test to narrow down the specific failing combination in chained concatenation

// Test 1: Basic combinations that should work
let test1a: string = "Value: " + 42;
console.log(test1a);

let test1b: string = "Active: " + true;
console.log(test1b);

let test1c: string = "Extra: " + 3.14;
console.log(test1c);

// Test 2: Two-element chains
let test2a: string = "Value: " + 42 + ", Active: ";
console.log(test2a);

let test2b: string = "Value: " + 42 + true;
console.log(test2b);

let test2c: string = ", Active: " + true + ", Extra: ";
console.log(test2c);

// Test 3: Three-element chains (where it might start failing)
let test3a: string = "Value: " + 42 + ", Active: " + true;
console.log(test3a);

let test3b: string = ", Active: " + true + ", Extra: " + 3.14;
console.log(test3b);

// Test 4: Four-element chain (closer to the problem)
let test4: string = "Value: " + 42 + ", Active: " + true + ", Extra: ";
console.log(test4);

// Test 5: The full problematic expression split differently
let test5a: string = "Value: " + 42 + ", Active: " + true;
let test5b: string = test5a + ", Extra: " + 3.14;
console.log(test5b);

// Test 6: Different operator grouping
let test6a: string =
	"Value: " + 42 + (", Active: " + true) + (", Extra: " + 3.14);
console.log(test6a);

// Test 7: Number + Boolean specifically in middle of chain
let test7: string = 42 + true + 3.14;
console.log(test7);
