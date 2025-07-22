// Comprehensive test combining while and for loops with break and continue

let total = 0;
let iterations = 0;

// Nested while and for loops
let i = 0;
while (i < 3) {
	for (let j = 0; j < 4; j = j + 1) {
		iterations = iterations + 1;

		// Skip even numbers in inner loop
		if (j === 2) {
			continue;
		}

		// Break out of inner loop early on last outer iteration
		if (i === 2 && j === 3) {
			break;
		}

		total = total + (i * 10 + j);
	}
	i = i + 1;
}

// Nested for and while loops
for (let x = 0; x < 2; x = x + 1) {
	let y = 0;
	while (y < 3) {
		if (y === 1) {
			y = y + 1;
			continue;
		}

		total = total + (x + y);
		y = y + 1;
	}
}

// Complex loop with multiple breaks and continues
let sum = 0;
for (let a = 0; a < 5; a = a + 1) {
	if (a === 1) {
		continue; // Skip a=1
	}

	let b = 0;
	while (b < 3) {
		if (a === 3 && b === 1) {
			break; // Break inner while when a=3, b=1
		}

		if (b === 0) {
			b = b + 1;
			continue; // Skip b=0
		}

		sum = sum + a + b;
		b = b + 1;
	}

	if (a === 4) {
		break; // Break outer for when a=4
	}
}

// Expected results:
// total should include values from nested loops
// sum should include values with breaks/continues
// iterations should count all loop iterations
