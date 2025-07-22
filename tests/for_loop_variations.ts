// Test various forms of for loops

let result1 = 0;
// Standard for loop
for (let i = 0; i < 3; i = i + 1) {
	result1 = result1 + i;
}

let result2 = 0;
let j = 0;
// For loop with external init
for (; j < 3; j = j + 1) {
	result2 = result2 + j;
}

let result3 = 0;
let k = 0;
// For loop with no update (manual increment)
for (; k < 3; ) {
	result3 = result3 + k;
	k = k + 1;
}

let result4 = 0;
let count = 0;
// For loop with no condition (infinite loop with break)
for (let m = 0; ; m = m + 1) {
	if (m >= 3) {
		break;
	}
	result4 = result4 + m;
	count = count + 1;
}

// result1, result2, result3, result4 should all be 3 (0+1+2)
