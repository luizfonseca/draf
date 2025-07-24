// Comprehensive test for global objects Number and Date

// Test Number static properties
let maxValue = Number.MAX_VALUE;
let minValue = Number.MIN_VALUE;
let maxSafeInt = Number.MAX_SAFE_INTEGER;
let minSafeInt = Number.MIN_SAFE_INTEGER;
let posInf = Number.POSITIVE_INFINITY;
let negInf = Number.NEGATIVE_INFINITY;
let nanValue = Number.NaN;
let epsilon = Number.EPSILON;

console.log("=== NUMBER STATIC PROPERTIES ===");
console.log("MAX_VALUE:", maxValue);
console.log("MIN_VALUE:", minValue);
console.log("MAX_SAFE_INTEGER:", maxSafeInt);
console.log("MIN_SAFE_INTEGER:", minSafeInt);
console.log("POSITIVE_INFINITY:", posInf);
console.log("NEGATIVE_INFINITY:", negInf);
console.log("NaN:", nanValue);
console.log("EPSILON:", epsilon);

// Test Number static methods
let parsed1 = Number.parseInt("42");
let parsed2 = Number.parseInt("3.14");
let parsed3 = Number.parseFloat("3.14159");
let parsed4 = Number.parseFloat("42");

console.log("=== NUMBER STATIC METHODS ===");
console.log("parseInt('42'):", parsed1);
console.log("parseInt('3.14'):", parsed2);
console.log("parseFloat('3.14159'):", parsed3);
console.log("parseFloat('42'):", parsed4);

// Test Number.isNaN
let isNaN1 = Number.isNaN(42);
let isNaN2 = Number.isNaN(Number.NaN);
let isNaN3 = Number.isNaN(3.14);

console.log("=== NUMBER.isNaN TESTS ===");
console.log("isNaN(42):", isNaN1);
console.log("isNaN(NaN):", isNaN2);
console.log("isNaN(3.14):", isNaN3);

// Test Number.isFinite
let isFinite1 = Number.isFinite(42);
let isFinite2 = Number.isFinite(Number.POSITIVE_INFINITY);
let isFinite3 = Number.isFinite(Number.NEGATIVE_INFINITY);
let isFinite4 = Number.isFinite(Number.NaN);

console.log("=== NUMBER.isFinite TESTS ===");
console.log("isFinite(42):", isFinite1);
console.log("isFinite(POSITIVE_INFINITY):", isFinite2);
console.log("isFinite(NEGATIVE_INFINITY):", isFinite3);
console.log("isFinite(NaN):", isFinite4);

// Test Date static methods
let now = Date.now();
let parsed = Date.parse("2022-01-01");
let utc = Date.UTC(2022, 0, 1);

console.log("=== DATE STATIC METHODS ===");
console.log("Date.now():", now);
console.log("Date.parse('2022-01-01'):", parsed);
console.log("Date.UTC(2022, 0, 1):", utc);

// Test Date constructor (simplified)
// Note: In our implementation, Date constructor behavior is simplified
console.log("=== DATE CONSTRUCTOR ===");
console.log("Date constructor tests would go here");

// Mixed global operations
let calculation1 = Number.MAX_SAFE_INTEGER + Number.MIN_SAFE_INTEGER;
let calculation2 = Number.EPSILON * 1000000;
let calculation3 = Date.now() - parsed;

console.log("=== MIXED GLOBAL OPERATIONS ===");
console.log("MAX_SAFE + MIN_SAFE:", calculation1);
console.log("EPSILON * 1000000:", calculation2);
console.log("now() - parsed:", calculation3);

// Test combining global methods with regular operations
let complexCalc1 = Number.parseInt("100") + Number.parseFloat("3.14");
let complexCalc2 = Number.isNaN(42) && Number.isFinite(100);

console.log("=== COMPLEX GLOBAL CALCULATIONS ===");
console.log("parseInt + parseFloat:", complexCalc1);
console.log("isNaN && isFinite:", complexCalc2);

console.log("=== ALL GLOBAL OBJECTS TESTS COMPLETED ===");
