The following table shows which input arguments lead to which result when passed to the Boolean constructor function.

The spec also refers to this operation as: **ToBoolean**

#### Arguments To Result

|Argument Type|Value(s)|Result|
|---|---|---|
|[[JS Knowledge/Cheat Sheets/The Boolean Constructor/Arguments To Result/Undefined\|Undefined]]|undefined|false|
|[[JS Knowledge/Cheat Sheets/The Boolean Constructor/Arguments To Result/Null\|Null]]|false|false|
|[[JS Knowledge/Cheat Sheets/The Boolean Constructor/Arguments To Result/Boolean\|Boolean]]|true|true|
|[[JS Knowledge/Cheat Sheets/The Boolean Constructor/Arguments To Result/Untitled 3\|Untitled 3]]|false|false|
|[[JS Knowledge/Cheat Sheets/The Boolean Constructor/Arguments To Result/Number\|Number]]|+0|false|
|[[JS Knowledge/Cheat Sheets/The Boolean Constructor/Arguments To Result/Untitled 2\|Untitled 2]]|-0|false|
|[[JS Knowledge/Cheat Sheets/The Boolean Constructor/Arguments To Result/Untitled\|Untitled]]|NaN|false|
|[[Untitled 4]]|All other numbers|true|
|[[JS Knowledge/Cheat Sheets/The Boolean Constructor/Arguments To Result/String\|String]]|""|false|
|[[Untitled 5]]|Any other non-empty String|true|
|[[JS Knowledge/Cheat Sheets/The Boolean Constructor/Arguments To Result/Object\|Object]]|[], {}, ...|true|

  
  

## Where The Boolean Constructor Is Used

### In Conditions

```JavaScript
const value = "I'm a String";

if (value) {
	doSomething();
}

// is actually evaluated as:
if (Boolean(value)) { // is true, see the table above under String
  doSomething();
}
```

```JavaScript

const value = 5;

function myFunction(value) {
  if (!value) {
    doSomething();
  }
}

// is actually evaluated as:
if (!Boolean(value)) { // is false, see the table above under Number
  doSomething();
}
```

```JavaScript
// Special case, take a careful look!
const value = "false";

if (value) { // runs because "false" is a non-empty String and thus true! See the table under String
  doSomething();
}
```

### Calling It Explicitly

```JavaScript
// No one can stop you from calling the constructor yourself
const value = "false";
const valueConvertedToBoolean = Boolean(value); // => true
```

```JavaScript
const value = 5;
// Yes, you can also create a boolean like this, 
// and this will also lead to Boolean(value) being called
// under the hood.
// The first exclamation mark takes value, converts it to a boolean and negates it.
// It's like writing !Boolean(value).
// The second exclamation mark negates it once again, so it is returned to its
// original interpretion.
const valueConvertedToBoolean = !!value; // => true
```