// Test for unary operators in Draf TypeScript compiler

console.log("=== UNARY OPERATORS TEST ===");

// Test 1: Positive numbers
let positiveNum: number = +5;
console.log("Positive number:", positiveNum);

// Test 2: Negative numbers
let negativeNum: number = -10;
console.log("Negative number:", negativeNum);

// Test 3: Logical NOT
let flag: boolean = true;
let notFlag: boolean = !flag;
console.log("NOT true:", notFlag);

let falseBool: boolean = false;
let notFalse: boolean = !falseBool;
console.log("NOT false:", notFalse);

// Test 4: Typeof operator
let x: number = 42;
let y: string = "hello";
let z: boolean = true;

console.log("typeof number:", typeof x);
console.log("typeof string:", typeof y);
console.log("typeof boolean:", typeof z);

// Test 5: Negative in function call
function testNegative(n: number) {
	console.log("Received:", n);
}

testNegative(-7);

console.log("=== UNARY OPERATORS TEST COMPLETE ===");
