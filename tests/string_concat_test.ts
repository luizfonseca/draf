// Minimal string concatenation test to isolate issues

// Test 1: String + String
let str1: string = "Hello";
let str2: string = " World";
let result1: string = str1 + str2;
console.log(result1);

// Test 2: String + Number
let message: string = "Count: ";
let count: number = 42;
let result2: string = message + count;
console.log(result2);

// Test 3: Number + String
let num: number = 100;
let suffix: string = " points";
let result3: string = num + suffix;
console.log(result3);

// Test 4: String + Boolean
let prefix: string = "Status: ";
let isActive: boolean = true;
let result4: string = prefix + isActive;
console.log(result4);

// Test 5: Boolean + String
let passed: boolean = false;
let result5: string = passed + " result";
console.log(result5);
