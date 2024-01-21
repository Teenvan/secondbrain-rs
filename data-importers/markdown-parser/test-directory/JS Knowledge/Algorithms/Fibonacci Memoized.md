The Fibonacci sequence is a sequence of numbers such that each number is the sum of the two preceding ones.

It starts with 0 and 1.

The sequence thus goes like:

0, 1, 1, 2, 3, 5, 8, 13, 21, ...

  

The algorithm below is a recursive implementation. By using memoization (a result once calculated gets stored and does not have to be recalculated) it gets close to the runtime of an iterative implementation, with the additional advantage of being more readable.

```JavaScript
/**
 * Given a number n, this function returns the fibonacci
 * number at index n of the sequence (index starting with 0).
 *
 * The space complexity of this algorithm is O(n).
 * The time complexity  of this algorithm is O(2n) = O(n).
 *
 * This algorithm should execute as fast as a usual loop implementation
 * while taking more space on both the heap and the call stack.
 * @param {*} n the index of the fibonacci number to obtain
 * @param {*} memo an optional object for memoization.
 */
function fibonacci(n, memo) {
  memo = memo || {};

  // no need to calculate if we already have a number memoized
  if (memo[n]) {
    return memo[n];
  }

  if (n <= 1) {
    return 1;
  }

  return (memo[n] = fibonacci(n - 1, memo) + fibonacci(n - 2, memo));
}
```