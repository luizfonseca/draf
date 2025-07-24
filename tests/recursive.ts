function factorial(n: number): number {
	if (n < 0) {
		console.log("Error: Factorial is not defined for negative numbers.");
		return 0;
	}

	if (n === 0 || n === 1) {
		return 1;
	}
	return n * factorial(n - 1);
}

console.log(factorial(20));

function fibonacci(n: number): number {
	if (n < 0) {
		console.log("Error: Fibonacci is not defined for negative numbers.");
		return 0;
	}

	if (n === 0) {
		return 0;
	} else if (n === 1) {
		return 1;
	}
	return fibonacci(n - 1) + fibonacci(n - 2);
}

console.log(fibonacci(3));

// function sumArray(arr: number[], index: number = 0): number {
// 	if (index >= arr.length) {
// 		return 0;
// 	}
// 	return arr[index] + sumArray(arr, index + 1);
// }

// console.log(sumArray([1, 2, 3, 4, 5]));
