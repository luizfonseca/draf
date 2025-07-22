// Working type declarations test - demonstrates parser capabilities
// Note: This test shows parsing works but semantic analysis is not yet implemented

// Basic type aliases - parsing works correctly
type UserId = number;
type UserName = string;

// Union type - parsing works correctly
type StringOrNumber = string | number;

// Simple interface - parsing works correctly
interface User {
	id: UserId;
	name: UserName;
	email: string;
	age?: number;
	readonly created: number;
}

// Interface inheritance - parsing works correctly
interface Admin extends User {
	permissions: string;
	level: number;
}

// Object type - parsing works correctly
type Config = {
	apiUrl: string;
	timeout: number;
	debug?: boolean;
	readonly version: string;
};

// Working runtime code with basic types only
let count: number = 42;
let message: string = "Types parsed successfully!";
let active: boolean = true;

console.log("Type declarations parsed successfully");
console.log("Count:", count);
console.log("Message:", message);
console.log("Active:", active);
