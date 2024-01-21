A Fisher-Yates shuffle is a shuffle algorithm which produces permutations of an array, with each permutation being equally likely to happen.

```JavaScript
/**
 * Implementation of a Fisher-Yates shuffle, producing unbiased permutations.
 * Every possible permutation of the array provided is equally likely to happen.
 *
 * @param {*} array the array to shuffle in-place
 */
const shuffle = array => {
  let current;
  let top;
  let tmp = (current = top = array.length);

  if (!top) {
    return array;
  }

  while (--top) {
    current = Math.floor(Math.random() * (top + 1));
    tmp = array[current];
    array[current] = array[top];
    array[top] = tmp;
  }

  return array;
};
```