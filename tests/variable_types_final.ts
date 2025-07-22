// Final Comprehensive Test for Variable Declaration Types
// Demonstrating current support for let, const, and var in Draf compiler
//
// STATUS: All basic functionality working ✅
// - let declarations ✅
// - const declarations ✅ (with required initialization)
// - var declarations ✅ (mirrors let behavior)
// - Type inference ✅
// - Console output ✅
// - Mixed usage ✅

// ========================================
// CURRENT IMPLEMENTATION STATUS
// ========================================

// ✅ WORKING FEATURES:
// - All three variable declaration keywords (let, const, var)
// - Type annotations and type inference
// - Required initialization for const variables
// - Expression evaluation with all variable types
// - Console output with proper type formatting
// - Mixed usage of different declaration types

// ⚠️  NOTES:
// - const reassignment prevention not yet implemented
// - var has same scoping rules as let (no hoisting)
// - All declaration types behave identically at runtime

// ========================================
// SECTION 1: LET DECLARATIONS
// ========================================

// LET DECLARATIONS SECTION

// Basic let with explicit types
let num_explicit: number = 42;
let bool_explicit: boolean = true;

console.log(num_explicit);
console.log(bool_explicit);

// Let with type inference
let num_inferred = 100;
let bool_inferred = false;

console.log(num_inferred);
console.log(bool_inferred);

// Let without initializer
let uninitialized_let: number;
uninitialized_let = 250;
console.log(uninitialized_let);

// Let reassignment (works correctly)
let reassignable_let = 10;
console.log(reassignable_let);
reassignable_let = 20;
console.log(reassignable_let);

// ========================================
// SECTION 2: CONST DECLARATIONS
// ========================================

// CONST DECLARATIONS SECTION

// Basic const with explicit types
const CONST_NUM: number = 3.14159;
const CONST_BOOL: boolean = true;

console.log(CONST_NUM);
console.log(CONST_BOOL);

// Const with type inference
const INFERRED_CONST = 99;
const BOOL_CONST = false;

console.log(INFERRED_CONST);
console.log(BOOL_CONST);

// Const with expressions
const CALCULATED_CONST = 10 + 5 * 2;
const LOGICAL_CONST = true && false;

console.log(CALCULATED_CONST);
console.log(LOGICAL_CONST);

// Note: Uninitialized const would cause compile error
// const INVALID_CONST: number; // Error: Const variables must be initialized

// ========================================
// SECTION 3: VAR DECLARATIONS
// ========================================

// VAR DECLARATIONS SECTION

// Basic var with explicit types
var var_num: number = 77;
var var_bool: boolean = true;

console.log(var_num);
console.log(var_bool);

// Var with type inference
var var_inferred = 88;
var var_bool_inferred = false;

console.log(var_inferred);
console.log(var_bool_inferred);

// Var without initializer (works like let)
var uninitialized_var: number;
uninitialized_var = 300;
console.log(uninitialized_var);

// Var reassignment (works like let)
var reassignable_var = 50;
console.log(reassignable_var);
reassignable_var = 60;
console.log(reassignable_var);

// ========================================
// SECTION 4: MIXED VARIABLE USAGE
// ========================================

// MIXED USAGE SECTION

// Variables of different declaration types
let mixed_let = 5;
const MIXED_CONST = 10;
var mixed_var = 15;

// Mathematical operations with mixed types
console.log(mixed_let + MIXED_CONST + mixed_var);
console.log(mixed_let * MIXED_CONST);
console.log(mixed_var - mixed_let);

// Boolean operations with mixed types
console.log(mixed_let > MIXED_CONST);
console.log(mixed_var != mixed_let);
console.log(mixed_let < mixed_var && MIXED_CONST > 0);

// Complex expressions
let complex_result = ((mixed_let + MIXED_CONST) * mixed_var) / 2;
console.log(complex_result);

// ========================================
// SECTION 5: CONSOLE OUTPUT TESTING
// ========================================

// CONSOLE OUTPUT TESTING SECTION

// Test all console methods with different variable types
console.info(mixed_let);
console.warn(MIXED_CONST);
console.error(mixed_var);
console.debug(BOOL_CONST);

// Multiple arguments with mixed declaration types
console.log(mixed_let, MIXED_CONST, mixed_var);
console.info(num_explicit, CONST_NUM, var_num);
console.warn(bool_explicit, CONST_BOOL, var_bool);

// Mixed types in single console call
console.log(mixed_let, CONST_BOOL, var_inferred, LOGICAL_CONST);

// ========================================
// SECTION 6: TYPE SYSTEM VALIDATION
// ========================================

// TYPE SYSTEM VALIDATION SECTION

// Ensure type checking works across declaration types
let typed_let: boolean = true;
const TYPED_CONST: number = 123;
var typed_var: boolean = false;

// Operations preserving type safety
console.log(typed_let && typed_var);
console.log(TYPED_CONST + mixed_let);
console.log(typed_let || CONST_BOOL);

// Complex type validation
let type_result = TYPED_CONST > 100 && (typed_let || typed_var);
console.log(type_result);

// ========================================
// SECTION 7: EDGE CASES AND LIMITS
// ========================================

// EDGE CASES SECTION

// Large numbers
const LARGE_CONST = 999999;
let large_let = 888888;
var large_var = 777777;

console.log(LARGE_CONST + large_let + large_var);

// Small decimals
const SMALL_CONST = 0.001;
let small_let = 0.002;
var small_var = 0.003;

console.log(SMALL_CONST + small_let + small_var);

// Boolean combinations
const TRUE_CONST = true;
let true_let = true;
var false_var = false;

console.log(TRUE_CONST && true_let && !false_var);

// Mathematical edge cases
const ZERO_CONST = 0;
let infinity_let = 5 / 0;
var negative_var = -42;

console.log(ZERO_CONST);
console.log(infinity_let);
console.log(negative_var);

// ========================================
// FINAL STATUS REPORT
// ========================================

// FINAL STATUS SECTION

// Summary of functionality
// Variable declaration types implemented:
// - let: Full support
// - const: Basic support (initialization required)
// - var: Full support (mirrors let)

// Current limitations:
// - const reassignment not prevented
// - var scoping identical to let
// - no hoisting behavior

// Success indicators
console.info(999);
console.log(true);
// All variable declaration types working!
