[https://twitter.com/oliverjumpertz/status/1287676004572504065?s=20](https://twitter.com/oliverjumpertz/status/1287676004572504065?s=20)

```JavaScript
console.log("b" + "a" + +"a" + "a"); // prints baNaNa

// Why is this?
// Let's take a look:
// "b" + "a" yields "ba"
// "ba" + +"a" => Attention, do you see the unary plus there?
//
// The unary plus has a higher precedence, so we have to evaluate it first.
// +"a" is actually the same as writing Number("a"), which yields NaN!
// So what now gets evaluated is:
//
// "ba" + NaN
//
// This is now a string + a number.
// The plus operator can be used both with strings and with numbers, but strings
// take priority when they are present.
// Thus what gets evaluated is:
//
// String("ba") + String(NaN)"
// => "ba" + "NaN"
// => "baNaN"
//
// What we're left with now is:
// "baNaN" + "a"
// => "baNaNa"
//
// And there we are. That's the result!
```