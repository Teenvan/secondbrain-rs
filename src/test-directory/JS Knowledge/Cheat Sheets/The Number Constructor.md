The following table shows which input arguments lead to which result when passed to the Number constructor function.

The spec also refers to this operation as: **ToNumber**

#### Arguments To Result

|Argument Type|Value(s)|Result|
|---|---|---|
|[[Undefined]]|undefined|NaN|
|[[Null]]|null|+0|
|[[Boolean]]|true|1|
|[[JS Knowledge/Cheat Sheets/The Number Constructor/Arguments To Result/Untitled\|Untitled]]|false|+0|
|[[Number]]||The same number as the one passed in|
|[[String]]|"123", "1.63",  <br>"1.434e4", "+1.5",  <br>"-20", ...|The result of parsing the number. (+\|-), ([0 1 2 3 4 5 6 7 8 9]), and (e \| E) are allowed characters|
|[[Untitled 2]]|""|0|
|[[Untitled 3]]|Any string containing another character as the numbers and characters stated above|NaN|
|[[Object]]|[], [...], {}, ...|The result 'res' of calling toPrimitive("number") , valueOf("number") , or finally toString() and then calling Number(res) (whichever returns a primitive first)|

  
  

## Where The Number Contructor Is Used

### In Expressions Involving Operations

```JavaScript
// Is evaluated as Number("5") - Number("2")
const result = "5" - "2"; // => 3
```

```JavaScript
// Is evaluated as Number(5) * Number("2")
const result = 5 * "2" // => 10
```

```JavaScript
// Is evaluated as Number("10") / Number(2)
const result = "10" / 2; // => 5
```

### Calling It Explicitly

```JavaScript
// No one can stop you from calling the Number constructor explicitly!
const num = 5;
const numAsString = String(num); // => "5"
```