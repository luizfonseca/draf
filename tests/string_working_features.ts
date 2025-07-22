// Working string features test for the Draf TypeScript compiler
// This test validates the currently implemented and working string functionality

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

console.log(4000); // === MULTILINE STRING TEST ===

// Test 12: Multiline template literals
let multilineMessage: string = `This is a multiline
string that spans
multiple lines`;
console.log(multilineMessage);

console.log(5000); // === STRING ESCAPE SEQUENCES TEST ===

// Test 13: Escape sequences in strings
let escaped: string = "Line 1\nLine 2\tTabbed";
console.log(escaped);

// Test 14: Quote escaping
let quotes: string = "He said 'Hello' to me";
console.log(quotes);

// Test 15: Backslash escaping
let backslashes: string = "Path: C:\\Users\\Name\\File.txt";
console.log(backslashes);

console.log(6000); // === STRING ASSIGNMENT AND MUTATION TEST ===

// Test 16: String variable assignment
let mutableString: string = "Initial value";
console.log(mutableString);

// Test 17: String reassignment
mutableString = "Updated value";
console.log(mutableString);

// Test 18: String reassignment with concatenation
mutableString = mutableString + " and more";
console.log(mutableString);

console.log(7000); // === COMPLEX STRING OPERATIONS TEST ===

// Test 19: Nested string operations
let part1: string = "Hello";
let part2: string = "World";
let separator: string = ", ";
let exclamation: string = "!";
let complexMessage: string = part1 + separator + part2 + exclamation;
console.log(complexMessage);

// Test 20: String operations in conditional expressions
let condition: boolean = true;
let conditionalString: string = condition ? "True message" : "False message";
console.log(conditionalString);

// Test 21: String concatenation in ternary
let choice: number = 1;
let choiceMessage: string =
	"You chose: " + (choice === 1 ? "Option A" : "Option B");
console.log(choiceMessage);

console.log(8000); // === TYPE COERCION TEST ===

// Test 22: Null coalescing with strings
let nullValue: string = "test" ?? "Default string";
console.log(nullValue);

// Test 23: Multiple type coercions
let mixed: string = "Value: " + 42 + ", Active: " + true;
console.log(mixed);

// Test 24: String in boolean context
let nonEmptyString: string = "test";
let emptyStringTest: string = "";
console.log(nonEmptyString !== ""); // Should be true
console.log(emptyStringTest === ""); // Should be true

console.log(9000); // === INTEGRATION WITH EXISTING FEATURES ===

// Test 25: Strings in if statements
if (firstName === "John") {
	console.log("First name is John");
}

// Test 26: String concatenation in if conditions
if (firstName + lastName === "JohnDoe") {
	console.log("Full name match");
}

// Test 27: String variables in logical operations
let user: string = "admin";
let isAdmin: boolean = user === "admin";
console.log(isAdmin);

// Test 28: String with console methods
console.info("Info message");
console.warn("Warning message");
console.error("Error message");
console.debug("Debug message");

// Test 29: Mixed types in console output
console.log("Mixed:", 42, true, "end");

console.log(9999); // === STRING WORKING FEATURES TEST COMPLETE ===
