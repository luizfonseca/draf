// Clean example demonstrating strongly typed compilation
// Only uses single-line expressions to avoid parser limitations

// Basic variable declarations with type inference
let count = 0;
let price = 99.99;
let quantity = 5;

// Explicit type annotations
let cost: number = 15.5;
let discount: number = 0.1;
let tax: number = 0.08;

// Complex arithmetic expressions
let subtotal = price * quantity;
let discountAmount = subtotal * discount;
let discountedTotal = subtotal - discountAmount;
let taxAmount = discountedTotal * tax;
let finalTotal = discountedTotal + taxAmount;

// Boolean operations and comparisons
let isExpensive = price > 50;
let isLargeOrder = quantity > 10;
let hasDiscount = discount > 0;
let isTaxable = tax > 0;

// Complex boolean expressions
let qualifiesForDiscount = isLargeOrder && price > 25;
let needsReview = isExpensive || quantity > 100;
let isValidOrder = quantity > 0 && price > 0;

// Arithmetic with precedence and parentheses
let complexCalculation = (price + cost) * quantity - discount * 100;
let average = (price + cost + tax) / 3;
let weighted = price * 0.6 + cost * 0.4;

// More boolean logic
let shouldProceed = isValidOrder && !needsReview;
let requiresApproval = isExpensive && isLargeOrder;
let canShip = isValidOrder && hasDiscount;

// Variable reassignment with proper typing
count = count + 1;
price = price * 1.05;
quantity = quantity + 2;

// Chained calculations
let step1 = price * quantity;
let step2 = step1 - discountAmount;
let step3 = step2 + taxAmount;
let result = step3 / quantity;

// Comparison operations
let isPriceEqual = price == cost;
let isPriceDifferent = price != cost;
let isPriceHigher = price > cost;
let isPriceLower = price < cost;
let isPriceAtLeast = price >= cost;
let isPriceAtMost = price <= cost;

// Unary operations
let negativePrice = -price;
let positiveDiscount = +discount;
let notExpensive = !isExpensive;
