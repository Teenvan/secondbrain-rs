[https://twitter.com/oliverjumpertz/status/1287466982535770117?s=20](https://twitter.com/oliverjumpertz/status/1287466982535770117?s=20)

```JavaScript
console.log(3 > 2 > 1); // actually prints false

// While this is a perfectly valid mathematical comparison, 
// which would usually yield true, programming languages evaluate expressions
// differently, and JavaScript itself is no different.
//
// We have to split the expression into two parts:
// First of all, 3 > 2 gets evaluated, which yields true.
// 3 > 2 === true
//
// Now the rest has to be evaluated:
// true > 1
// This isn't directly possible, as the types are different, 
// so coercion happens under the hood.
// In case of the greater-than (>) operator, a type hint of number is passed,
// so true is converted to a number.
// Number(true) > Number(1)
// => 1 > 1
// => false
```