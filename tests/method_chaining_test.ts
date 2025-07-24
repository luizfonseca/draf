// Test for method chaining in Draf TypeScript compiler

// Test 1: Simple array method chaining
let numbers = [1, 2, 3, 4, 5];

console.log("=== METHOD CHAINING TEST ===");
console.log("Original array created");

// Test 2: Chain slice and join
let result1 = numbers.slice(1, 3).join("-");
console.log("Slice and join result:", result1);

// Test 3: Chain multiple array methods
let result2 = [10, 20, 30].slice(0, 2).join(",");
console.log("Multiple chain result:", result2);

// Test 4: Chain with literals
let result3 = [1, 2, 3, 4].slice(1).join("|");
console.log("Literal chain result:", result3);

// Test 5: Longer chain
let result4 = [5, 4, 3, 2, 1].reverse().slice(0, 3).join(" -> ");
console.log("Longer chain result:", result4);

console.log("=== METHOD CHAINING TESTS COMPLETED ===");
