// Comprehensive test for variable declarations: let, const, and var
// Testing all three variable declaration types with various scenarios

// ===== SECTION 1: LET DECLARATIONS =====

// Basic let declarations
let x: number = 10;
let y: number = 20;
let flag: boolean = true;

console.log(x);
console.log(y);
console.log(flag);

// Let with type inference
let inferred_num = 42;
let inferred_bool = false;

console.log(inferred_num);
console.log(inferred_bool);

// Let without initializer (should work)
let uninitialized: number;
uninitialized = 100;
console.log(uninitialized);

// ===== SECTION 2: CONST DECLARATIONS =====

// Basic const declarations (must be initialized)
const PI: number = 3.14159;
const MAX_SIZE: number = 1000;
const IS_ENABLED: boolean = true;

console.log(PI);
console.log(MAX_SIZE);
console.log(IS_ENABLED);

// Const with type inference
const auto_num = 99;
const auto_bool = true;

console.log(auto_num);
console.log(auto_bool);

// Const with expressions
const calculated = 10 + 5 * 2;
const logical_result = true && false;

console.log(calculated);
console.log(logical_result);

// ===== SECTION 3: VAR DECLARATIONS =====

// Basic var declarations (should work like let)
var a: number = 30;
var b: number = 40;
var enabled: boolean = false;

console.log(a);
console.log(b);
console.log(enabled);

// Var with type inference
var var_inferred_num = 77;
var var_inferred_bool = true;

console.log(var_inferred_num);
console.log(var_inferred_bool);

// Var without initializer (should work like let)
var var_uninitialized: number;
var_uninitialized = 200;
console.log(var_uninitialized);

// ===== SECTION 4: MIXED USAGE =====

// Using variables of different declaration types together
let mixed_let = 5;
const mixed_const = 10;
var mixed_var = 15;

console.log(mixed_let + mixed_const + mixed_var);
console.log(mixed_let > mixed_const);
console.log(mixed_var != mixed_let);

// ===== SECTION 5: EXPRESSIONS WITH DIFFERENT VARIABLE TYPES =====

// Mathematical operations
let math_result = mixed_let * mixed_const + mixed_var;
console.log(math_result);

// Boolean operations
const bool_result = mixed_let < mixed_const && mixed_var > mixed_let;
console.log(bool_result);

// Complex expressions
var complex_expr = ((mixed_let + mixed_const) * mixed_var) / 2;
console.log(complex_expr);

// ===== SECTION 6: SCOPE AND REASSIGNMENT TESTS =====

// Let reassignment (should work)
let reassignable = 1;
console.log(reassignable);
reassignable = 2;
console.log(reassignable);

// Var reassignment (should work like let)
var var_reassignable = 10;
console.log(var_reassignable);
var_reassignable = 20;
console.log(var_reassignable);

// Note: const reassignment would be a compile-time error
// This demonstrates that const variables cannot be reassigned
// const const_reassignable = 100;
// const_reassignable = 200; // This should cause an error

// ===== SECTION 7: CONSOLE OUTPUT WITH ALL TYPES =====

// Test console methods with all variable types
console.info(mixed_let);
console.warn(mixed_const);
console.error(mixed_var);
console.debug(bool_result);

// Multiple arguments with mixed declaration types
console.log(mixed_let, mixed_const, mixed_var);
console.info(PI, MAX_SIZE, IS_ENABLED);
console.warn(reassignable, var_reassignable, auto_bool);

// ===== SECTION 8: TYPE VALIDATION =====

// Ensure type checking works for all declaration types
let typed_let: boolean = true;
const typed_const: number = 42;
var typed_var: boolean = false;

console.log(typed_let && typed_var);
console.log(typed_const + mixed_let);
console.log(typed_let || typed_var);

// ===== FINAL STATUS =====
console.info(999);
console.log(true);
