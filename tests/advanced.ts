// Advanced example demonstrating more complex strongly typed features

// Basic variable declarations with type inference
let count = 0;
let message = "Hello, Draf!";
let isActive = true;

// Explicit type annotations
let price: number = 99.99;
let productName: string = "TypeScript Compiler";
let inStock: boolean = false;

// Complex expressions with proper type checking
let total = price * count;
let discountedPrice = price - price * 0.1;
let taxAmount = total * 0.08;

// Boolean logic operations
let canPurchase = inStock && price > 0;
let shouldAlert = !inStock || count < 1;
let isValidOrder = canPurchase && total > 10;

// Arithmetic operations with precedence
let calculation = (price + taxAmount) * count - discountedPrice;
let average = (price + discountedPrice + taxAmount) / 3;

// Comparison operations
let isExpensive = price > 100;
let isAffordable = price <= 50;
let exactMatch = count == 5;
let notZero = count != 0;

// Chained assignments
let temp: number = 42;
temp = temp + 10;
temp = temp * 2;
temp = temp - 5;

// Complex boolean expressions
let complexCondition = price > 50 && count > 0 && isActive;
let alternativeCheck = price < 20 || count > 100 || !inStock;

// Nested arithmetic with mixed operations
let formula = (price * 1.2 + count * 0.5) / 2;

// Variable reassignment with type safety
count = count + 1;
price = price * 1.15;
isActive = !isActive;

// Final calculations
let grandTotal = total + taxAmount;
let savings = price - discountedPrice;
let efficiency = grandTotal / count;

// Explicit null and undefined handling (strongly typed)
let nullableValue: null = null;
let undefinedValue: undefined = undefined;

// Any type (requires explicit handling in strongly typed mode)
let dynamicValue: any;
