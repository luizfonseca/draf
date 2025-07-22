// Member access and optional chaining test

// Define a simple interface for testing
interface User {
	id: number;
	name: string;
	email?: string;
}

// Test 1: Basic object with member access
let user: User = {
	id: 1,
	name: "John Doe",
	email: "john@example.com",
};

// Test 2: Simple member access (not yet implemented in parser)
// let userId = user.id;
// let userName = user.name;

// Test 3: Optional chaining (not yet implemented in parser)
// let userEmail = user?.email;
// let userPhone = user?.phone;

// Test 4: Nested optional chaining (future test)
// let nestedProperty = user?.profile?.avatar?.url;

// Test 5: Bracket access (future test)
// let dynamicProperty = user["name"];
// let optionalBracket = user?.["email"];

// Test 6: typeof operator
let userType = typeof user;
let nameType = typeof "hello";
let numberType = typeof 42;
let boolType = typeof true;

console.log("Member access test file loaded");
console.log("User object created successfully");
console.log("Type checks completed");
