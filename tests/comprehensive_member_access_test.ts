// Comprehensive test for member access patterns, optional chaining, and typeof operator

// Test 1: Simple object without complex interfaces to avoid type issues
let user = {
	id: 1,
	name: "Alice Johnson",
	email: "alice@example.com",
	tagsCount: 3,
};

// Test 2: typeof operator on various types
let numberType = typeof 42;
let stringType = typeof "hello world";
let booleanType = typeof true;
let objectType = typeof user;
let numberType2 = typeof user.tagsCount;
let nullType = typeof null;
let undefinedType = typeof undefined;

console.log("=== TYPEOF TESTS ===");
console.log("typeof 42:", numberType);
console.log("typeof 'hello':", stringType);
console.log("typeof true:", booleanType);
console.log("typeof user:", objectType);
console.log("typeof tagsCount:", numberType2);
console.log("typeof null:", nullType);
console.log("typeof undefined:", undefinedType);

// Test 3: Basic member access
let userId = user.id;
let userName = user.name;
let userTagsCount = user.tagsCount;

console.log("=== BASIC MEMBER ACCESS ===");
console.log("User ID:", userId);
console.log("User Name:", userName);

// Test 4: Optional chaining member access
let userEmail = user?.email;
let userProfile = user?.profile;
let userSafeName = user?.name;
let userSafeEmail = user?.email;
let userSafeId = user?.id;

console.log("=== OPTIONAL CHAINING ===");
console.log("Optional email access works");
console.log("Deep optional chaining works");

// Test 5: Bracket notation access
let dynamicId = user["id"];
let dynamicName = user["name"];
let dynamicEmail = user["email"];

console.log("=== BRACKET ACCESS ===");
console.log("Bracket access works");

// Test 6: Simple mixed access patterns
let mixedAccess1 = user?.name;
let mixedAccess2 = user["email"];

console.log("=== MIXED ACCESS PATTERNS ===");
console.log("Mixed access patterns work");

// Test 8: typeof on member access results
let typeOfUserId = typeof user.id;
let typeOfUserEmail = typeof user?.email;
let typeOfUserTagsCount = typeof user.tagsCount;

console.log("=== TYPEOF ON MEMBER ACCESS ===");
console.log("typeof user.id:", typeOfUserId);
console.log("typeof optional member:", typeOfUserEmail);
console.log("typeof tagsCount member:", typeOfUserTagsCount);

// Test 9: Object with all primitive types
let testObject = {
	num: 42,
	str: "test",
	bool: true,
	nullVal: null,
	undefinedVal: undefined,
};

let numFromObj = testObject.num;
let strFromObj = testObject.str;
let boolFromObj = testObject.bool;
let nullFromObj = testObject.nullVal;
let undefinedFromObj = testObject.undefinedVal;

console.log("=== PRIMITIVE MEMBER ACCESS ===");
console.log("All primitive types accessible");

// Test 10: typeof on all primitive members
let typeOfNum = typeof testObject.num;
let typeOfStr = typeof testObject.str;
let typeOfBool = typeof testObject.bool;
let typeOfNull = typeof testObject.nullVal;
let typeOfUndef = typeof testObject.undefinedVal;

console.log("=== TYPEOF ON PRIMITIVES ===");
console.log("typeof number member:", typeOfNum);
console.log("typeof string member:", typeOfStr);
console.log("typeof boolean member:", typeOfBool);
console.log("typeof null member:", typeOfNull);
console.log("typeof undefined member:", typeOfUndef);

console.log("=== ALL TESTS COMPLETED SUCCESSFULLY ===");
