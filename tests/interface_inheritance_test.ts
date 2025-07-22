// Interface inheritance test - demonstrates structural typing with inheritance

// Base interface
interface Person {
	id: number;
	name: string;
	email: string;
}

// Interface extending Person
interface Employee extends Person {
	department: string;
	salary: number;
}

// Interface extending Employee
interface Manager extends Employee {
	teamSize: number;
	budget: number;
}

// Test 1: Valid Person assignment
let person: Person = {
	id: 1,
	name: "John Doe",
	email: "john@example.com",
};

// Test 2: Valid Employee assignment with all fields
let employee: Employee = {
	id: 2,
	name: "Jane Smith",
	email: "jane@example.com",
	department: "Engineering",
	salary: 75000,
};

// Test 3: Valid Manager assignment with all inherited fields
let manager: Manager = {
	id: 3,
	name: "Bob Johnson",
	email: "bob@example.com",
	department: "Engineering",
	salary: 95000,
	teamSize: 8,
	budget: 500000,
};

// Test 4: Interface inheritance - Manager can be assigned to Employee
let employeeFromManager: Employee = manager;

// Test 5: Interface inheritance - Employee can be assigned to Person
let personFromEmployee: Person = employee;

// Test 6: Multi-level inheritance - Manager can be assigned to Person
let personFromManager: Person = manager;

// Test 7: Object with extra properties assigned to base interface
let personWithExtra: Person = {
	id: 4,
	name: "Alice Wilson",
	email: "alice@example.com",
	age: 30,
	city: "San Francisco",
};

console.log("Interface inheritance test completed successfully");
console.log("All inheritance assignments validated correctly");
