// Debug test for console boolean output
// Isolating the difference between literals and expressions

// Test 1: Boolean literals only
console.log(true);
console.log(false);

// Test 2: Simple boolean expressions
let result1: boolean = true;
let result2: boolean = false;
console.log(result1);
console.log(result2);

// Test 3: Comparison expressions
console.log(5 > 3);
console.log(2 > 10);

// Test 4: Logical operations
console.log(true && true);
console.log(true && false);
console.log(false || true);

// Test 5: Negation
console.log(!true);
console.log(!false);
