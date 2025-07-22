// Simple interface type checking test

// Define a basic interface
interface User {
	id: number;
	name: string;
	email: string;
}

// Test 1: Valid object assignment to interface
let user1: User = {
	id: 123,
	name: "John Doe",
	email: "john@example.com",
};

// Test 2: Object with extra properties (should work - structural typing)
let user2: User = {
	id: 456,
	name: "Jane Smith",
	email: "jane@example.com",
	department: "Engineering",
};

// Test 3: Simple interface usage
console.log("User 1 created successfully");
console.log("User 2 created successfully");

console.log("Simple interface type checking test completed");
