// Simplified integration test to debug ternary and multiline issues
let base: number = 10;
let flag: boolean = true;

// Test 1: Simple multiline assignment
let test1: boolean = base > 5 && flag;

console.log(test1); // Should be true

// Test 2: Simple ternary
let test2: number = base > 5 ? 100 : 200;

console.log(test2); // Should be 100

// Test 3: Multiline ternary (single level)
let test3: number = base > 5 ? 300 : 400;

console.log(test3); // Should be 300

// Test 4: Boolean chain
let test4: boolean = base > 0 && base < 100 && flag;

console.log(test4); // Should be true

// Test 5: Mixed operators
let test5: number = base + 5 * 2;

console.log(test5); // Should be 20

// Test 6: Complex but single line ternary
let test6: number = base > 5 ? (flag ? 500 : 600) : 700;

console.log(test6); // Should be 500

// Test 7: Logical OR multiline
let test7: boolean = false || flag;

console.log(test7); // Should be true

// Test 8: Strict equality multiline
let test8: boolean = base === 10 && flag !== false;

console.log(test8); // Should be true

console.log(9999); // Test completion marker
