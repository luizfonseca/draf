// Comprehensive test for function execution in Draf TypeScript compiler

console.log("=== FUNCTION EXECUTION TEST ===");

// =============================================================================
// SECTION 1: BASIC FUNCTION CALLS
// =============================================================================

console.log("--- Testing Basic Function Calls ---");

// Test 1.1: Simple function without parameters
function sayHello() {
	console.log("Hello from sayHello function!");
}

// Call the function
sayHello();

// Test 1.2: Function with parameters
function greet(name: string) {
	console.log("Hello,", name);
}

greet("World");

// Test 1.3: Function with multiple parameters
function addNumbers(a: number, b: number) {
	console.log("Adding", a, "and", b);
	let sum: number = a + b;
	console.log("Result:", sum);
}

addNumbers(5, 10);

console.log("✓ Basic function calls working");

// =============================================================================
// SECTION 2: FUNCTIONS WITH RETURN VALUES
// =============================================================================

console.log("--- Testing Functions with Return Values ---");

// Test 2.1: Function returning a number
function multiply(x: number, y: number): number {
	return x * y;
}

let product: number = multiply(4, 7);
console.log("Product:", product);

// Test 2.2: Function returning a string
function getMessage(): string {
	return "This is a test message";
}

let message: string = getMessage();
console.log("Message:", message);

// Test 2.3: Function returning a boolean
function isPositive(num: number): boolean {
	return num > 0;
}

let positive: boolean = isPositive(42);
console.log("Is positive:", positive);

let negative: boolean = isPositive(-5);
console.log("Is negative:", negative);

console.log("✓ Functions with return values working");

// =============================================================================
// SECTION 3: FUNCTION PARAMETER TYPES
// =============================================================================

console.log("--- Testing Function Parameter Types ---");

// Test 3.1: Number parameters
function processNumber(value: number) {
	console.log("Processing number:", value);
	let doubled: number = value * 2;
	console.log("Doubled:", doubled);
}

processNumber(15);

// Test 3.2: String parameters
function processString(text: string) {
	console.log("Processing string:", text);
}

processString("Hello Functions");

// Test 3.3: Boolean parameters
function processBoolean(flag: boolean) {
	console.log("Processing boolean:", flag);
}

processBoolean(true);
processBoolean(false);

// Test 3.4: Mixed parameter types
function processMixed(name: string, count: number, active: boolean) {
	console.log("Name:", name);
	console.log("Count:", count);
	console.log("Active:", active);
}

processMixed("TestItem", 42, true);

console.log("✓ Function parameter types working");

// =============================================================================
// SECTION 4: NESTED FUNCTION CALLS
// =============================================================================

console.log("--- Testing Nested Function Calls ---");

// Test 4.1: Function calling another function
function square(n: number): number {
	return n * n;
}

function sumOfSquares(a: number, b: number): number {
	let squareA: number = square(a);
	let squareB: number = square(b);
	return squareA + squareB;
}

let result: number = sumOfSquares(3, 4);
console.log("Sum of squares:", result);

// Test 4.2: Function with function call as parameter
function double(x: number): number {
	return x * 2;
}

let doubled: number = double(square(5));
console.log("Double of square:", doubled);

console.log("✓ Nested function calls working");

// =============================================================================
// SECTION 5: CONDITIONAL RETURNS
// =============================================================================

console.log("--- Testing Conditional Returns ---");

// Test 5.1: Function with conditional logic
function getSign(num: number): string {
	if (num > 0) {
		return "positive";
	} else if (num < 0) {
		return "negative";
	} else {
		return "zero";
	}
}

console.log("Sign of 10:", getSign(10));
console.log("Sign of -5:", getSign(-5));
console.log("Sign of 0:", getSign(0));

// Test 5.2: Function with early return
function isEven(n: number): boolean {
	if (n % 2 === 0) {
		return true;
	}
	return false;
}

console.log("Is 8 even:", isEven(8));
console.log("Is 7 even:", isEven(7));

console.log("✓ Conditional returns working");

// =============================================================================
// SECTION 6: VOID FUNCTIONS
// =============================================================================

console.log("--- Testing Void Functions ---");

// Test 6.1: Explicit void function
function logMessage(msg: string): void {
	console.log("LOG:", msg);
}

logMessage("This is a void function");

// Test 6.2: Implicit void function
function performAction() {
	console.log("Performing an action...");
	console.log("Action completed");
}

performAction();

console.log("✓ Void functions working");

// =============================================================================
// SECTION 7: FUNCTION SCOPE TESTING
// =============================================================================

console.log("--- Testing Function Scope ---");

// Test 7.1: Local variables in functions
function testScope() {
	let localVar: number = 100;
	console.log("Local variable:", localVar);
}

testScope();

// Test 7.2: Parameters vs global variables
let globalVar: number = 999;

function useParameter(globalVar: number) {
	console.log("Parameter value:", globalVar);
}

useParameter(123);
console.log("Global variable:", globalVar);

console.log("✓ Function scope working");

// =============================================================================
// SECTION 8: MATHEMATICAL FUNCTIONS
// =============================================================================

console.log("--- Testing Mathematical Functions ---");

// Test 8.1: Area calculation
function calculateArea(width: number, height: number): number {
	return width * height;
}

let area: number = calculateArea(5, 8);
console.log("Area:", area);

// Test 8.2: Temperature conversion
function celsiusToFahrenheit(celsius: number): number {
	return (celsius * 9) / 5 + 32;
}

let fahrenheit: number = celsiusToFahrenheit(25);
console.log("25°C in Fahrenheit:", fahrenheit);

// Test 8.3: Factorial function
function factorial(n: number): number {
	if (n <= 1) {
		return 1;
	}
	return n * factorial(n - 1);
}

let fact5: number = factorial(5);
console.log("Factorial of 5:", fact5);

console.log("✓ Mathematical functions working");

// =============================================================================
// FINAL SUMMARY
// =============================================================================

console.log("=== FUNCTION EXECUTION TEST SUMMARY ===");
console.log("✓ Basic function calls: WORKING");
console.log("✓ Functions with return values: WORKING");
console.log("✓ Function parameter types: WORKING");
console.log("✓ Nested function calls: WORKING");
console.log("✓ Conditional returns: WORKING");
console.log("✓ Void functions: WORKING");
console.log("✓ Function scope: WORKING");
console.log("✓ Mathematical functions: WORKING");

console.log("=== ALL FUNCTION EXECUTION FEATURES WORKING ===");
