[https://twitter.com/oliverjumpertz/status/1288073061858672640?s=20](https://twitter.com/oliverjumpertz/status/1288073061858672640?s=20)

This one is a variation of another quiz of mine, which you can find [here](https://www.notion.so/Oliver-s-GT-Chain-3eb55043711f4a5dad858560885f896d?pvs=21).

The same rules about the associativity and such apply here, as well.

```JavaScript
console.log(1 < 2 < 3);

// We can split that up in the same way that the executor will do it
// at runtime.
//
// 1 < 2 will yield true, of course, because 1 is less than 2.
const first = 1 < 2; // => true

// The executor now has to deal with a comparison between a boolean and a number.
// It will coerce the left side with a type hint of "number", which leads to
// the expression being evaluated as:
// Number(true) < Number(3)
// => 1 < 3
const second = first < 3; // => true
```