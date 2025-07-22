// Final validation test with the exact user code pattern
// This test ensures the specific TypeScript code requested by the user works perfectly

let final_edge: number = 5;

let final_bool: boolean =
	(final_edge > 0 && true) || (false && final_edge == 1);

console.log(final_bool); // Should be true

// Additional validation with different values
let edge_zero: number = 0;
let bool_zero: boolean = (edge_zero > 0 && true) || (false && edge_zero == 1);

console.log(bool_zero); // Should be false

let edge_one: number = 1;
let bool_one: boolean = (edge_one > 0 && true) || (false && edge_one == 1);

console.log(bool_one); // Should be true

// Test with negative values
let edge_negative: number = -5;
let bool_negative: boolean =
	(edge_negative > 0 && true) || (false && edge_negative == 1);

console.log(bool_negative); // Should be false

// Test with larger values
let edge_large: number = 100;
let bool_large: boolean =
	(edge_large > 0 && true) || (false && edge_large == 1);

console.log(bool_large); // Should be true

console.log(9999); // Test completion marker
