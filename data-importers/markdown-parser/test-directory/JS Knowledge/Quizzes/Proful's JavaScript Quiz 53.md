[https://twitter.com/profulsadangi/status/1286986554393714693?s=20](https://twitter.com/profulsadangi/status/1286986554393714693?s=20)

Yes, it can.  
  
JavaScript's type coercion does not only know how to convert primitive values, but also has a specification to define how objects are to be converted to primitives.  
  
There initially were 2 methods known to the runtime, that it would use to perform the calculation.  
But recently a third one was added, that even takes priority now.  
  
The order is:  

1. toPrimitive(hint)
2. valueOf()
3. toString()

And the executor will stop calling the methods as soon as one of those methods returns a primitive.

Notice the 'hint' parameter on toPrimitive? The runtime will actually pass a string parameter into it to request a special type.

If you implement this yourself, you can then decide what to return.

  

In the quiz above however, only toString() is defined and returns a Number.

The executor will take that number, call Number(result) (because it assumes toString to always return a string), and then use this primitive for the operation at hand.

Every time the executor does this, count is incremented afterwards.

So after the first comparison, count is 3332, after the second it's 3333, and after the third it's actually 3334.

This leads to all checks being true!