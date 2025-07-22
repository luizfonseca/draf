// Test for number + boolean concatenation
// This should work by converting both to strings

let num: number = 42;
let flag: boolean = true;

// Test 1: Number + Boolean
let result1: string = num + flag;
console.log(result1);

// Test 2: Boolean + Number
let result2: string = flag + num;
console.log(result2);

// Test 3: More complex cases
let score: number = 95;
let passed: boolean = true;
let message1: string = score + " points scored";
let message2: string = passed + " - test result";
console.log(message1);
console.log(message2);

// Test 4: Mixed concatenation
let value: number = 100;
let active: boolean = false;
let combined: string = "Value: " + value + ", Active: " + active;
console.log(combined);
