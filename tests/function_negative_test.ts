// Test function calls with negative numbers to debug the unary operation issue

console.log("=== FUNCTION NEGATIVE NUMBER TEST ===");

// Test 1: Simple function with negative parameter
function testNegative(n: number) {
	console.log("Received number:", n);
}

console.log("Calling function with negative number...");
testNegative(-5);

// Test 2: Function that returns negative value
function getNegative(): number {
	return -10;
}

let negResult: number = getNegative();
console.log("Negative result:", negResult);

// Test 3: Function with negative number in expression
function checkSign(x: number): boolean {
	return x > 0;
}

let isPositive: boolean = checkSign(-7);
console.log("Is -7 positive:", isPositive);

// Test 4: Multiple negative numbers
function subtract(a: number, b: number): number {
	return a - b;
}

let result: number = subtract(-3, -8);
console.log("(-3) - (-8) =", result);

console.log("=== FUNCTION NEGATIVE NUMBER TEST COMPLETE ===");
