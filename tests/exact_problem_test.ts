// Test the exact problematic line from string_comprehensive.ts

// This is the exact line that should be failing:
let mixed: string = "Value: " + 42 + ", Active: " + true + ", Extra: " + 3.14;
console.log(mixed);
