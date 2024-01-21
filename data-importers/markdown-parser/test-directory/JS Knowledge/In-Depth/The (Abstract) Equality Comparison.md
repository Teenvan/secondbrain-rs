JavaScript knows two types of equality comparisons.

1. Strict ( === )
2. Abstract ( == )

This section is going to focus on the abstract equality comparison operation and the algorithm behind it.

  

An equality comparison is performed based on:

`x == y`

Where x is the left operand, and y is the right operand.

  

The algorithm goes as follows:

1. If Type(x) is the same as Type(y), then
    1. If Type(x) is Undefined, return `true`.
    2. If Type(x) is Null, return `true`.
    3. If Type(x) is Number, then
        1. If `x` is `NaN`, return `false`.
        2. If `y` is `NaN`, return `false`.
        3. If `x` has the same value as `y`, return `true`.
        4. If `x` is `+0` and `y` is `-0`, return `true`.
        5. If `x` is `-0` and `y` is `+0`, return `true`.
        6. Return `false`.
    4. If Type(x) is String, return `true` if and only if `x` and `y` are **exactly the same sequence of characters** (same length, same characters in sequence). Otherwise return `false`.
    5. If Type(x) is Boolean, return `true` if `x` and `y` are **both** `true`, **or** are **both** `false`; otherwise return `false`.
2. If `x` is `null` and `y` is `undefined`, return `true`.
3. If `x` is `undefined` and `y` is `null`, return `true`.
4. If Type(x) is Number and Type(y) is String, return the result of the comparison x == [ToNumber](https://www.notion.so/The-Number-Constructor-922e2aab8c9f439d9c4390c044d3805f?pvs=21)(y).
5. If Type(x) is String and Type(y) is Number, return the result of the comparison [ToNumber](https://www.notion.so/The-Number-Constructor-922e2aab8c9f439d9c4390c044d3805f?pvs=21)(x) == y.
6. If Type(x) is Boolean, return the result of the comparison [ToNumber](https://www.notion.so/The-Number-Constructor-922e2aab8c9f439d9c4390c044d3805f?pvs=21)(x) == y.
7. If Type(y) is Boolean, return the result of the comparison x == [ToNumber](https://www.notion.so/The-Number-Constructor-922e2aab8c9f439d9c4390c044d3805f?pvs=21)(y).
8. If Type(x) is either String or Number and Type(y) is Object, return the result of the comparison x == [ToPrimitive](https://www.notion.so/Object-To-Primitive-Conversion-8bca15d8d5354313b72248fafc3ad301?pvs=21)(y).
9. If Type(x) is Object and Type(y) is either String or Number, return the result of the comparison [ToPrimitive](https://www.notion.so/Object-To-Primitive-Conversion-8bca15d8d5354313b72248fafc3ad301?pvs=21)(x) == y.
10. Return `false`.