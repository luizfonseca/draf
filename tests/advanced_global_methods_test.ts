// Advanced global objects test showcasing method call chaining and complex operations

// Test 1: Number method chaining and combinations
let base = Number.parseFloat("123.456");
let rounded = Number.parseInt("999.999");
let maxVal = Number.MAX_SAFE_INTEGER;
let minVal = Number.MIN_SAFE_INTEGER;

console.log("=== ADVANCED NUMBER OPERATIONS ===");
console.log("parseFloat result:", base);
console.log("parseInt result:", rounded);
console.log("MAX_SAFE_INTEGER:", maxVal);
console.log("MIN_SAFE_INTEGER:", minVal);

// Test 2: Complex calculations with Number globals
let calculation1 = Number.parseFloat("42.7") + Number.parseInt("58.3");
let calculation2 = Number.MAX_VALUE / Number.MIN_VALUE;
let calculation3 = Number.POSITIVE_INFINITY - Number.NEGATIVE_INFINITY;
let calculation4 = Number.EPSILON * Number.MAX_SAFE_INTEGER;

console.log("=== COMPLEX NUMBER CALCULATIONS ===");
console.log("parseFloat + parseInt:", calculation1);
console.log("MAX_VALUE / MIN_VALUE:", calculation2);
console.log("POS_INF - NEG_INF:", calculation3);
console.log("EPSILON * MAX_SAFE:", calculation4);

// Test 3: Boolean operations with Number methods
let isValid1 = Number.isFinite(42) && Number.isFinite(100);
let isValid2 = Number.isNaN(Number.NaN) || Number.isNaN(42);
let isValid3 = !Number.isNaN(123.45) && Number.isFinite(123.45);

console.log("=== BOOLEAN OPERATIONS WITH NUMBER METHODS ===");
console.log("isFinite(42) && isFinite(100):", isValid1);
console.log("isNaN(NaN) || isNaN(42):", isValid2);
console.log("!isNaN(123.45) && isFinite(123.45):", isValid3);

// Test 4: Date operations and timestamps
let now = Date.now();
let parsed = Date.parse("2023-12-25");
let utc = Date.UTC(2024, 0, 1, 12, 0, 0);

console.log("=== DATE OPERATIONS ===");
console.log("Date.now():", now);
console.log("Date.parse('2023-12-25'):", parsed);
console.log("Date.UTC(2024, 0, 1, 12, 0, 0):", utc);

// Test 5: Mixed global object operations
let timeCalc1 = Date.now() - Date.parse("2020-01-01");
let timeCalc2 = Date.UTC(2025, 11, 31) - Date.now();
let numberTime = Number.parseFloat("1000") + Date.now();

console.log("=== MIXED GLOBAL OPERATIONS ===");
console.log("now() - parse('2020-01-01'):", timeCalc1);
console.log("UTC(2025,11,31) - now():", timeCalc2);
console.log("parseFloat('1000') + now():", numberTime);

// Test 6: Complex conditional operations
let isValidNumber = Number.isFinite(42.5) && !Number.isNaN(42.5);
let isValidTime = Date.now() > Date.parse("1970-01-01");
let complexCondition = isValidNumber && isValidTime;

console.log("=== COMPLEX CONDITIONAL OPERATIONS ===");
console.log("Valid number check:", isValidNumber);
console.log("Valid time check:", isValidTime);
console.log("Complex condition:", complexCondition);

// Test 7: Nested global property access
let infinitySum = Number.POSITIVE_INFINITY + Number.NEGATIVE_INFINITY;
let extremeValues = Number.MAX_VALUE + Number.MIN_VALUE;
let safeRange = Number.MAX_SAFE_INTEGER - Number.MIN_SAFE_INTEGER;

console.log("=== NESTED GLOBAL PROPERTY ACCESS ===");
console.log("POS_INF + NEG_INF:", infinitySum);
console.log("MAX_VALUE + MIN_VALUE:", extremeValues);
console.log("MAX_SAFE - MIN_SAFE:", safeRange);

// Test 8: Global methods in expressions
let result1 = Number.parseInt("50") * 2 + Number.parseFloat("0.75");
let result2 = Date.now() / Number.parseFloat("1000");
let result3 = Number.MAX_SAFE_INTEGER % Number.parseInt("997");

console.log("=== GLOBAL METHODS IN EXPRESSIONS ===");
console.log("(parseInt('50') * 2) + parseFloat('0.75'):", result1);
console.log("Date.now() / parseFloat('1000'):", result2);
console.log("MAX_SAFE_INTEGER % parseInt('997'):", result3);

// Test 9: Verification operations
let verifyFinite = Number.isFinite(result1) && Number.isFinite(result2);
let verifyNotNaN = !Number.isNaN(result1) && !Number.isNaN(result3);
let verifyPositive = result1 > 0 && result2 > 0;

console.log("=== VERIFICATION OPERATIONS ===");
console.log("Results are finite:", verifyFinite);
console.log("Results not NaN:", verifyNotNaN);
console.log("Results positive:", verifyPositive);

// Test 10: Edge case handling
let edgeCase1 = Number.parseFloat("123");
let edgeCase2 = Number.parseInt("456");
let edgeCase3 = Number.isFinite(Number.POSITIVE_INFINITY);

console.log("=== EDGE CASE HANDLING ===");
console.log("parseFloat('123'):", edgeCase1);
console.log("parseInt('456'):", edgeCase2);
console.log("isFinite(POS_INFINITY):", edgeCase3);

console.log("=== ADVANCED GLOBAL METHODS TEST COMPLETED ===");
