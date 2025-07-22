let sum = 0;
for (let i = 0; i < 10; i = i + 1) {
	if (i === 2) {
		continue;
	}

	if (i === 7) {
		break;
	}

	sum = sum + i;
}

// sum should be 0 + 1 + 3 + 4 + 5 + 6 = 19
// (skips 2, breaks at 7)
