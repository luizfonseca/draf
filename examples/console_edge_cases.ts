// Edge cases and boundary conditions test for console functionality
// Testing various edge cases, boundary values, and potential error conditions

// ===== SECTION 1: Numerical Edge Cases =====

// Very large numbers
let max_safe: number = 9007199254740991;
let very_large: number = 1000000000000000;
console.log(max_safe);
console.log(very_large);

// Very small numbers
let tiny: number = 0.000000000001;
let epsilon: number = 0.0000000001;
console.log(tiny);
console.log(epsilon);

// Special numeric values
let zero_positive: number = 0;
let zero_negative: number = -0;
let divide_by_zero: number = 1 / 0;
let negative_infinity: number = -1 / 0;

console.log(zero_positive);
console.log(zero_negative);
console.log(divide_by_zero);
console.log(negative_infinity);

// ===== SECTION 2: Boolean Edge Cases =====

// Complex boolean chains
let chain1: boolean = true && true && true;
let chain2: boolean = false || false || true;
let chain3: boolean = (true && false) || true;
let chain4: boolean = false || (true && false);

console.log(chain1);
console.log(chain2);
console.log(chain3);
console.log(chain4);

// Nested boolean expressions
let nested1: boolean = (true && false) || false || true;
let nested2: boolean = !(true && false) && !(false || false);
let nested3: boolean = !(!true || !false);

console.log(nested1);
console.log(nested2);
console.log(nested3);

// ===== SECTION 3: Mathematical Boundary Conditions =====

// Division edge cases
let div1: number = 0 / 1;
let div2: number = 1 / 1;
let div3: number = 100 / 1;
let div4: number = 1 / 100;

console.log(div1);
console.log(div2);
console.log(div3);
console.log(div4);

// Multiplication edge cases
let mult1: number = 0 * 1000;
let mult2: number = 1 * 1;
let mult3: number = 1000 * 0;
let mult4: number = -1 * -1;

console.log(mult1);
console.log(mult2);
console.log(mult3);
console.log(mult4);

// ===== SECTION 4: Comparison Edge Cases =====

// Equality with floating point
let float1: number = 0.1 + 0.2;
let float2: number = 0.3;
console.log(float1);
console.log(float2);
console.log(float1 == float2);

// Comparisons with zero
console.log(0 == 0);
console.log(0 != 0);
console.log(0 > 0);
console.log(0 < 0);
console.log(0 >= 0);
console.log(0 <= 0);

// ===== SECTION 5: Console Method Stress Tests =====

// Single argument console calls
console.log(1);
console.info(2);
console.warn(3);
console.error(4);
console.debug(5);

// Many arguments console calls
console.log(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
console.info(true, false, true, false, true);
console.warn(1.1, 2.2, 3.3, 4.4, 5.5);

// Mixed type arguments in various combinations
console.log(true, 1, false, 2, true, 3);
console.info(0, true, 1, false, 2);
console.error(false, 0, true, 1, false, 2);

// ===== SECTION 6: Expression Complexity Tests =====

// Deeply nested mathematical expressions
let deep_math1: number = ((1 + 2) * 3 - 4) / 5 + 6;
let deep_math2: number = 1 + 2 * 3 - 4 / 5 + 6;

console.log(deep_math1);
console.log(deep_math2);

// Deeply nested boolean expressions
let deep_bool1: boolean = ((true && false) || true) && (false || true);
let deep_bool2: boolean = (true && false) || (true && false) || true;

console.log(deep_bool1);
console.log(deep_bool2);

// ===== SECTION 7: Variable Name Edge Cases =====

// Variables with similar names
let x: number = 1;
let xx: number = 2;
let xxx: number = 3;

console.log(x);
console.log(xx);
console.log(xxx);

// Variables with underscores and numbers (if supported)
let var_1: number = 10;
let var_2: number = 20;
let var_12: number = 30;

console.log(var_1);
console.log(var_2);
console.log(var_12);

// ===== SECTION 8: Order of Operations Stress Test =====

// Complex precedence test
let precedence_test: number = 2 + 3 * 4 + 5 * 6 - 7;
console.log(precedence_test);

// Parentheses override
let paren_test: number = (2 + 3) * (4 + 5) * (6 - 7);
console.log(paren_test);

// Boolean precedence with numbers
let bool_num_test: boolean = (5 > 3 && 10 < 20) || 15 == 14;
console.log(bool_num_test);

// ===== SECTION 9: Final Comprehensive Edge Case =====

// Combining all edge cases
let final_edge: number = ((0 / 1 + 1 * 1) * (10 - 5)) / (2 + 3);
let final_bool: boolean =
	(final_edge > 0 && true) || (false && final_edge == 1);

console.log(final_edge);
console.log(final_bool);

// Status output
console.info(999);
console.log(true);
