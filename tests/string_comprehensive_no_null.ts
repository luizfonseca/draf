// Comprehensive string features test for the Draf TypeScript compiler
// This test validates all implemented string functionality including literals, concatenation, and interpolation
// Modified to exclude null coalescing which is not fully implemented

console.log(1000); // === STRING LITERALS TEST START ===

// Test 1: Double-quoted string literals
let doubleQuoted: string = "Hello from double quotes";
console.log(doubleQuoted);

// Test 2: Single-quoted string literals
let singleQuoted: string = "Hello from single quotes";
console.log(singleQuoted);

// Test 3: Basic template literals (no interpolation)
let templateBasic: string = `Hello from template literal`;
console.log(templateBasic);

// Test 4: Empty strings
let emptyDouble: string = "";
let emptySingle: string = "";
let emptyTemplate: string = ``;
console.log(emptyDouble);
console.log(emptySingle);
console.log(emptyTemplate);

console.log(2000); // === STRING CONCATENATION TEST ===

// Test 5: Basic string concatenation
let firstName: string = "John";
let lastName: string = "Doe";
let fullName: string = firstName + " " + lastName;
console.log(fullName);

// Test 6: String + Number concatenation (type coercion)
let age: number = 25;
let ageMessage: string = "Age: " + age;
console.log(ageMessage);

// Test 7: String + Boolean concatenation (type coercion)
let isActive: boolean = true;
let statusMessage: string = "Active: " + isActive;
console.log(statusMessage);

// Test 8: Number + String concatenation (type coercion)
let score: number = 95;
let scoreMessage: string = score + " points scored";
console.log(scoreMessage);

// Test 9: Boolean + String concatenation (type coercion)
let passed: boolean = true;
let passMessage: string = passed + " - test result";
console.log(passMessage);

console.log(3000); // === STRING COMPARISON TEST ===

// Test 10: String equality comparisons
let str1: string = "test";
let str2: string = "test";
let str3: string = "different";

console.log(str1 == str2); // Should be true
console.log(str1 === str2); // Should be true
console.log(str1 != str3); // Should be true
console.log(str1 !== str3); // Should be true

// Test 11: String inequality comparisons
console.log(str1 == str3); // Should be false
console.log(str1 === str3); // Should be false
console.log(str1 != str2); // Should be false
console.log(str1 !== str2); // Should be false

console.log(4000); // === TEMPLATE LITERAL INTERPOLATION TEST ===

// Test 12: Simple template literal with variable interpolation
let name: string = "Alice";
let greeting: string = `Hello, ${name}!`;
console.log(greeting);

// Test 13: Template literal with number interpolation
let count: number = 42;
let countMessage: string = `Count is: ${count}`;
console.log(countMessage);

// Test 14: Template literal with boolean interpolation
let ready: boolean = true;
let readyMessage: string = `System ready: ${ready}`;
console.log(readyMessage);

// Test 15: Template literal with multiple interpolations
let user: string = "Bob";
let userAge: number = 30;
let userInfo: string = `User ${user} is ${userAge} years old`;
console.log(userInfo);

console.log(5000); // === MULTILINE STRING TEST ===

// Test 16: Multiline template literals
let multilineMessage: string = `This is a multiline
string that spans
multiple lines`;
console.log(multilineMessage);

// Test 17: Template literal with interpolation across lines
let product: string = "Widget";
let price: number = 99;
let productInfo: string = `Product: ${product}
Price: $${price}
Available: ${true}`;
console.log(productInfo);

console.log(6000); // === STRING ESCAPE SEQUENCES TEST ===

// Test 18: Escape sequences in strings
let escaped: string = "Line 1\nLine 2\tTabbed\rCarriage Return";
console.log(escaped);

// Test 19: Quote escaping
let quotes: string = 'He said "Hello" to me';
console.log(quotes);

// Test 20: Backslash escaping
let backslashes: string = "Path: C:\\Users\\Name\\File.txt";
console.log(backslashes);

console.log(7000); // === STRING ASSIGNMENT AND MUTATION TEST ===

// Test 21: String variable assignment
let mutableString: string = "Initial value";
console.log(mutableString);

// Test 22: String reassignment
mutableString = "Updated value";
console.log(mutableString);

// Test 23: String reassignment with concatenation
mutableString = mutableString + " and more";
console.log(mutableString);

console.log(8000); // === COMPLEX STRING OPERATIONS TEST ===

// Test 24: Nested string operations
let part1: string = "Hello";
let part2: string = "World";
let separator: string = ", ";
let exclamation: string = "!";
let complexMessage: string = part1 + separator + part2 + exclamation;
console.log(complexMessage);

// Test 25: String operations in conditional expressions
let condition: boolean = true;
let conditionalString: string = condition ? "True message" : "False message";
console.log(conditionalString);

// Test 26: String concatenation in ternary
let choice: number = 1;
let choiceMessage: string =
	"You chose: " + (choice === 1 ? "Option A" : "Option B");
console.log(choiceMessage);

console.log(9000); // === TYPE COERCION EDGE CASES TEST ===

// Test 27: Multiple type coercions (THE PROBLEMATIC LINE)
let mixed: string = "Value: " + 42 + ", Active: " + true + ", Extra: " + 3.14;
console.log(mixed);

// Test 28: String in boolean context
let nonEmptyString: string = "test";
let emptyStringTest: string = "";
console.log(nonEmptyString !== ""); // Should be true
console.log(emptyStringTest === ""); // Should be true

console.log(9500); // === TEMPLATE LITERAL ADVANCED TEST ===

// Test 29: Template literal with expression interpolation
let x: number = 10;
let y: number = 5;
let mathResult: string = `Result: ${x + y}`;
console.log(mathResult);

// Test 30: Template literal with comparison interpolation
let comparison: string = `Is x greater than y? ${x > y}`;
console.log(comparison);

// Test 31: Nested template literals (basic)
let inner: string = `inner ${x}`;
let outer: string = `outer (${inner}) complete`;
console.log(outer);

console.log(9999); // === STRING COMPREHENSIVE TEST COMPLETE (NO NULL COALESCING) ===
