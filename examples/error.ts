// This file contains intentional type errors to test our strong typing

// Type mismatch - should fail
let x: number = "hello";

// Cannot add different types - should fail
let y = 5 + "world";

// Using undefined variable - should fail
let z = undefinedVariable;
