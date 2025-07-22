// Test for the user's specific example: multiline assignment with boolean expression
let final_edge: number = 5;

let final_bool: boolean =
	(final_edge > 0 && true) || (false && final_edge == 1);

console.log(final_bool); // Should be true

// Additional variations to ensure robustness
let final_edge2: number = 1;

let final_bool2: boolean =
	(final_edge2 > 0 && true) || (false && final_edge2 == 1);

console.log(final_bool2); // Should be true

let final_edge3: number = 0;

let final_bool3: boolean =
	(final_edge3 > 0 && true) || (false && final_edge3 == 1);

console.log(final_bool3); // Should be false

// Test with different indentation and spacing
let final_edge4: number = 2;

let final_bool4: boolean =
	(final_edge4 > 0 && true) || (false && final_edge4 == 1);

console.log(final_bool4); // Should be true

// Test with multiple newlines
let final_edge5: number = 3;

let final_bool5: boolean =
	(final_edge5 > 0 && true) || (false && final_edge5 == 1);

console.log(final_bool5); // Should be true

console.log(9999); // Test completion marker
