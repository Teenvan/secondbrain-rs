Sometimes, objects need to be converted to primitives.

Whenever you use an object in a calculation, for example, a primitive conversion is performed, and that conversion is made based on some rules.

The primitive conversion is also performed on some occasions, which do not involve real objects, but the runtime calls it nevertheless.

  

|Input Type|Result|
|---|---|
|[[JS Knowledge/In-Depth/Object-To-Primitive Conversion/Untitled Database/Undefined\|Undefined]]|undefined|
|[[JS Knowledge/In-Depth/Object-To-Primitive Conversion/Untitled Database/Null\|Null]]|null|
|[[JS Knowledge/In-Depth/Object-To-Primitive Conversion/Untitled Database/Boolean\|Boolean]]|The result equals the input argument.|
|[[JS Knowledge/In-Depth/Object-To-Primitive Conversion/Untitled Database/Number\|Number]]|The result equals the input argument.|
|[[JS Knowledge/In-Depth/Object-To-Primitive Conversion/Untitled Database/String\|String]]|The result equals the input argument.|
|[[JS Knowledge/In-Depth/Object-To-Primitive Conversion/Untitled Database/Object\|Object]]|Return the result of applying the algorithm below.|

  
  

## ToPrimitive

The algorithm to convert an object to a primitive, is defined within the operation `ToPrimitive(input [, preferredType])`

1. If Type(input) is Object, then
    1. If `preferredType` is not present, set `hint` to `"default"`
    2. Else If `preferredType` is hint String, set `hint` to `"string"`
    3. Else
        1. Set `hint` to `"number"`
    4. Set `exoticToPrim` to `input[Symbol.iterator]`
    5. If exoticToPrim is not undefined
        1. Set `result` to `input[Symbol.iterator](hint)`
        2. If Type(result) is not Object, return `result`
        3. Throw a TypeError exception
    6. If `hint` is `"default"`, set `hint` to `"number"`
    7. Call `OrdinaryToPrimitive(input, hint)`
2. Return `input`

  

## OrdinaryToPrimitive

The algorithm `OrdinaryToPrimitive(input, hint)`:

1. If `hint` is `"string"`, then
    1. Set `methodNames` to `["toString", "valueOf"]`
2. Else
    1. Set `methodNames` to `["valueOf", "toString"]`
3. For each `name` in list `methodNames` in order, do
    1. If `input[name]` is present (not undefined), then
        1. Set `result` to `input[name]()`
        2. If `result` is not Type(Object), return `result`
4. Throw a TypeError exception

  

  

  

  

  

  

  

1. Return `input`