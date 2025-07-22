// Test for ternary expressions in string concatenation contexts

// Test 1: Simple ternary in string concatenation
let choice: number = 1;
let choiceMessage: string =
	"You chose: " + (choice === 1 ? "Option A" : "Option B");
console.log(choiceMessage);

// Test 2: Ternary with different choice
let choice2: number = 2;
let choiceMessage2: string =
	"You chose: " + (choice2 === 1 ? "Option A" : "Option B");
console.log(choiceMessage2);

// Test 3: Ternary with boolean condition
let isActive: boolean = true;
let statusMessage: string = "Status: " + (isActive ? "Active" : "Inactive");
console.log(statusMessage);

// Test 4: Ternary with number results
let score: number = 85;
let gradeMessage: string = "Grade: " + (score >= 90 ? "A" : "B");
console.log(gradeMessage);

// Test 5: Nested ternary in concatenation
let value: number = 50;
let category: string =
	"Category: " + (value > 75 ? "High" : value > 25 ? "Medium" : "Low");
console.log(category);

// Test 6: Multiple ternaries in same expression
let x: number = 10;
let y: number = 5;
let comparison: string =
	"Results: " +
	(x > y ? "X wins" : "Y wins") +
	" and " +
	(x + y > 10 ? "Sum is high" : "Sum is low");
console.log(comparison);
