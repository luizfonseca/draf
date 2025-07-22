let i = 0;
let sum = 0;

while (i < 10) {
	i = i + 1;

	if (i === 3) {
		continue;
	}

	if (i === 7) {
		break;
	}

	sum = sum + i;
}

// sum should be 1 + 2 + 4 + 5 + 6 = 18
// (skips 3, breaks at 7)
