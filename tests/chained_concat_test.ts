// Test for complex chained string concatenation with mixed types

// Test 1: Simple chained concatenation
let result1: string = "Value: " + 42 + " End";
console.log(result1);

// Test 2: Multiple type coercions in chain
let mixed: string = "Value: " + 42 + ", Active: " + true + ", Extra: " + 3.14;
console.log(mixed);

// Test 3: Chained concatenation with variables
let name: string = "Alice";
let age: number = 25;
let isStudent: boolean = true;
let gpa: number = 3.8;
let info: string =
	"Name: " +
	name +
	", Age: " +
	age +
	", Student: " +
	isStudent +
	", GPA: " +
	gpa;
console.log(info);

// Test 4: Mixed number and boolean operations
let score: number = 95;
let passed: boolean = true;
let bonus: number = 5;
let report: string =
	"Score: " + score + ", Passed: " + passed + ", Bonus: " + bonus;
console.log(report);

// Test 5: Complex expression with parentheses
let x: number = 10;
let y: number = 20;
let calculation: string = "Result: " + (x + y) + ", Average: " + (x + y) / 2;
console.log(calculation);

// Test 6: Boolean arithmetic in concatenation
let a: boolean = true;
let b: boolean = false;
let boolResult: string = "A: " + a + ", B: " + b + ", Combined: " + (a || b);
console.log(boolResult);

// Test 7: Number + Boolean + Number pattern
let num1: number = 10;
let flag: boolean = true;
let num2: number = 20;
let pattern: string = "Start: " + num1 + ", Flag: " + flag + ", End: " + num2;
console.log(pattern);
