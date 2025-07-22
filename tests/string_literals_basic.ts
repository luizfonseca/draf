// Basic string literals test
// This test validates support for double-quoted, single-quoted, and basic template literals

// Test double-quoted strings
let message1: string = "Hello, World!";
console.log(message1);

// Test single-quoted strings
let message2: string = "Hello from single quotes!";
console.log(message2);

// Test basic template literals (without interpolation)
let message3: string = `Hello from template literal!`;
console.log(message3);

// Test string with escape sequences
let message4: string = "Line 1\nLine 2\tTabbed";
console.log(message4);

// Test empty strings
let empty1: string = "";
let empty2: string = "";
let empty3: string = ``;
console.log(empty1);
console.log(empty2);
console.log(empty3);

// Test string concatenation with + operator
let firstName: string = "John";
let lastName: string = "Doe";
let fullName: string = firstName + " " + lastName;
console.log(fullName);

// Test string concatenation with numbers (type coercion)
let age: number = 25;
let ageMessage: string = "I am " + age + " years old";
console.log(ageMessage);

// Test string concatenation with booleans (type coercion)
let isActive: boolean = true;
let statusMessage: string = "Status: " + isActive;
console.log(statusMessage);

// Test multiline strings using template literals
let multiline: string = `This is a
multiline string
that spans multiple lines`;
console.log(multiline);

// Test string assignment and reassignment
let mutableString: string = "Initial value";
console.log(mutableString);
mutableString = "Updated value";
console.log(mutableString);

// Test string comparison
let str1: string = "test";
let str2: string = "test";
let str3: string = "different";

console.log(str1 === str2); // Should be true
console.log(str1 === str3); // Should be false
console.log(str1 !== str3); // Should be true

console.log(9999); // Test completion marker
