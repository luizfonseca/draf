// Test for array instance methods in Draf TypeScript compiler

// Test 1: Basic array instance methods
let numbers = [1, 2, 3, 4, 5];

console.log("=== ARRAY INSTANCE METHODS TEST ===");
console.log("Original array created");

// Test 2: push method
let pushResult = numbers.push(6);
console.log("Push result:", pushResult);

// Test 3: pop method
let popResult = numbers.pop();
console.log("Pop result:", popResult);

// Test 4: join method
let joinResult = numbers.join(",");
console.log("Join result:", joinResult);

// Test 5: slice method
let sliceResult = numbers.slice(1, 3);
console.log("Slice result created");

// Test 6: indexOf method
let indexResult = numbers.indexOf(3);
console.log("Index of 3:", indexResult);

// Test 7: includes method
let includesResult = numbers.includes(4);
console.log("Includes 4:", includesResult);

// Test 8: concat method
let otherArray = [10, 11, 12];
let concatResult = numbers.concat(otherArray);
console.log("Concat result created");

// Test 9: reverse method
let reverseResult = numbers.reverse();
console.log("Reverse result created");

// Test 10: shift method
let shiftResult = numbers.shift();
console.log("Shift result:", shiftResult);

// Test 11: unshift method
let unshiftResult = numbers.unshift(0);
console.log("Unshift result:", unshiftResult);

// Test 12: Method chaining
let chainedResult = [1, 2, 3].slice(0, 2).join("-");
console.log("Chained result:", chainedResult);

console.log("=== ALL ARRAY INSTANCE METHODS TESTED ===");
