[https://twitter.com/profulsadangi/status/1287694104323735552?s=20](https://twitter.com/profulsadangi/status/1287694104323735552?s=20)

```JavaScript
// Operator precedence applies: division/multiplication > addition/subtraction
console.log("10" / 2 * "5" - "2" + "5");

// The expression is evaluated in this order:
// Multiplication is only defined for numbers.
// As a String is present, coercion happens under the hood.
// "10" / 2
// => Number("10") / Number(2)
// => 10 / 2
const first = "10" / 2; // => 5

// Division is only defined for numbers.
// Coercion happens under the hood again.
// 5 * "5"
// => Number(5) * Number("5")
// => 5 * 5
const second = first * "5"; // => 25

// Subtraction is only defined for numbers.
// Coercion again.
// 25 - "2"
// => Number(25) - Number("2")
// => 25 - 2
const third = second - "2"; // => 23

// Addition is both defined for numbers and for strings.
// Strings always take priority when they're part of an operation involving addition.
// The left side is coerced.
// 23 + "5"
// => String(23) + String("5")
// => "23" + "5"
const fourth = third + "5"; // => "235"
```