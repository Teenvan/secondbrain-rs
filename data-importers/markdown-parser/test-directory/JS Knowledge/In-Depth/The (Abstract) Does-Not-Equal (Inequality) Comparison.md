  

JavaScript knows two types of inequality comparisons.

1. Strict ( !== )
2. Abstract ( != )

This section is going to focus on the abstract inequality comparison operation and the algorithm behind it.

  

An inequality comparison is performed based on:

`x != y`

Where x is the left operand, and y is the right operand.

  

The algorithm goes as follows:

1. Set `result` to the result of performing the [(abstract) equality comparison](https://www.notion.so/The-Abstract-Equality-Comparison-4869119164b44c34b60310356a9b09a3?pvs=21) `x == y`.
2. If `result` is `true`
    1. Return `false`.
3. Return `true`.