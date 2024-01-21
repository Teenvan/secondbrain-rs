[https://twitter.com/oliverjumpertz/status/1287848520351965186?s=21](https://twitter.com/oliverjumpertz/status/1287848520351965186?s=21)

```JavaScript
// c is declared and initialized with value "constructor"
const c = "constructor";

// c is just a string
c; // -> "constructor"

// Accessing the property "constructor" of a string returns the string 
// constructor
c[c]; // -> [Function: String]

// Accessing the property "constructor" of the String constructor 
// returns the Function constructor
c[c][c]; // -> [Function: Function]

// Calling the Function constructor and passing
// the body of a new function as an argument
// Yes, this defines a function on the fly, with a string!
c[c][c]('console.log("WTH?!")'); // -> [Function: anonymous]

// Calling the anonymous function
// The result is console-logging a string "WTH?!"
c[c][c]('console.log("WTH?!")')(); // => prints WTH?!
```