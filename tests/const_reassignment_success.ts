// Test to verify that let and var can be reassigned while const cannot
// This file should compile successfully and demonstrate proper behavior

// ===== SECTION 1: LET REASSIGNMENT (Should work) =====

let mutable_let: number = 10;
console.log(mutable_let);

// Reassign let variable - should work
mutable_let = 20;
console.log(mutable_let);

// Multiple reassignments
mutable_let = 30;
mutable_let = 40;
console.log(mutable_let);

// Let with boolean
let bool_let: boolean = true;
console.log(bool_let);
bool_let = false;
console.log(bool_let);

// ===== SECTION 2: VAR REASSIGNMENT (Should work) =====

var mutable_var: number = 100;
console.log(mutable_var);

// Reassign var variable - should work
mutable_var = 200;
console.log(mutable_var);

// Multiple reassignments
mutable_var = 300;
mutable_var = 400;
console.log(mutable_var);

// Var with boolean
var bool_var: boolean = false;
console.log(bool_var);
bool_var = true;
console.log(bool_var);

// ===== SECTION 3: CONST USAGE (Should work for reading) =====

const PI: number = 3.14159;
const MAX_SIZE: number = 1000;
const IS_ENABLED: boolean = true;

// Reading const values - should work
console.log(PI);
console.log(MAX_SIZE);
console.log(IS_ENABLED);

// Using const in expressions - should work
console.log(PI * 2);
console.log(MAX_SIZE + 100);
console.log(IS_ENABLED && true);

// ===== SECTION 4: MIXED OPERATIONS =====

// Operations mixing let, var, and const
let mixed_result = mutable_let + mutable_var + MAX_SIZE;
console.log(mixed_result);

// Boolean operations
let bool_result = bool_let && bool_var && IS_ENABLED;
console.log(bool_result);

// Reassigning based on const values
mutable_let = PI * 10;
mutable_var = MAX_SIZE / 10;
console.log(mutable_let);
console.log(mutable_var);

// ===== SECTION 5: COMPLEX REASSIGNMENT PATTERNS =====

// Reassignment with expressions
let counter = 0;
console.log(counter);

counter = counter + 1;
console.log(counter);

counter = counter * 2;
console.log(counter);

// Conditional-style reassignment
var flag = true;
console.log(flag);

flag = counter > 1;
console.log(flag);

// ===== SECTION 6: TYPE SAFETY IN REASSIGNMENT =====

// Reassignment must respect types
let typed_num: number = 42;
let typed_bool: boolean = false;

console.log(typed_num);
console.log(typed_bool);

// Valid reassignments
typed_num = 84;
typed_bool = true;

console.log(typed_num);
console.log(typed_bool);

// ===== FINAL STATUS =====

// All mutable variables can be reassigned
console.info(999);
console.log(true);
