// Test just null coalescing to see if it's implemented

// Test 1: Basic null coalescing
let test1 = null ?? "default";
console.log(test1);

// Test 2: Undefined coalescing
let test2 = undefined ?? "fallback";
console.log(test2);

// Test 3: Variable null coalescing
let maybeNull = null;
let test3 = maybeNull ?? "backup";
console.log(test3);
