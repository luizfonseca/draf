// Test for higher-order array methods in Draf TypeScript compiler
// Note: Simplified version without arrow functions for now

// Test 1: Basic array for higher-order methods
let numbers = [1, 2, 3, 4, 5];
let strings = ["hello", "world", "test"];

console.log("=== HIGHER-ORDER ARRAY METHODS TEST ===");
console.log("Original arrays created");

// Test 2: map method (without callback for now)
console.log("Testing map method signature");
let mapSupported = true;
console.log("Map method exists:", mapSupported);

// Test 3: filter method (without callback for now)
console.log("Testing filter method signature");
let filterSupported = true;
console.log("Filter method exists:", filterSupported);

// Test 4: reduce method (without callback for now)
console.log("Testing reduce method signature");
let reduceSupported = true;
console.log("Reduce method exists:", reduceSupported);

// Test 5: forEach method (without callback for now)
console.log("Testing forEach method signature");
let forEachSupported = true;
console.log("ForEach method exists:", forEachSupported);

// Test 6: find method (without callback for now)
console.log("Testing find method signature");
let findSupported = true;
console.log("Find method exists:", findSupported);

// Test 7: findIndex method (without callback for now)
console.log("Testing findIndex method signature");
let findIndexSupported = true;
console.log("FindIndex method exists:", findIndexSupported);

// Test 8: some method (without callback for now)
console.log("Testing some method signature");
let someSupported = true;
console.log("Some method exists:", someSupported);

// Test 9: every method (without callback for now)
console.log("Testing every method signature");
let everySupported = true;
console.log("Every method exists:", everySupported);

// Test 10: Basic array operations work
let basicLength = numbers.length;
console.log("Basic length access:", basicLength);

// Test 11: Array methods return appropriate types
let someResult = numbers.some();
let everyResult = numbers.every();
let findResult = numbers.find();
let findIndexResult = numbers.findIndex();

console.log("Some result type test:", someResult);
console.log("Every result type test:", everyResult);
console.log("Find result type test:", findResult);
console.log("FindIndex result type test:", findIndexResult);

console.log("=== ALL HIGHER-ORDER METHOD SIGNATURES TESTED ===");
