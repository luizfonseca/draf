// Basic variable declarations with strong typing
let x: number = 42;
let name: string = "Hello, Draf!";
let isReady: boolean = true;

// Type inference
let inferredNumber = 10.5;
let inferredString = "TypeScript";
let inferredBoolean = false;

// Basic arithmetic operations
let sum = x + inferredNumber;
let difference = x - 10;
let product = sum * 2;
let quotient = product / 4;

// Boolean operations
let logicalAnd = isReady && inferredBoolean;
let logicalOr = isReady || false;
let logicalNot = !isReady;

// Variable reassignment
x = 100;
name = "Updated name";
isReady = false;

// Expressions
let complexExpression = (x + 5) * 2 - 10;
let booleanExpression = x > 50 && name == "Updated name";

// Null and undefined (explicit in strongly typed mode)
let nullValue: null = null;
let undefinedValue: undefined = undefined;

// Any type (requires explicit handling)
let anyValue: any = 42;
