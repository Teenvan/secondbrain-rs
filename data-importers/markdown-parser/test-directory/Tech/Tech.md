- SCSS
    - Deep Modifier
        
        `.el-table /deep/ .el-table__body-wrapper::-webkit-scrollbar-thumb {`
        
        if you use the deep in the first one then everything below will have the deep modifier
        
        so if you do `/deep/.el-table {`
        
        everything inside will be deep also it can be rwritten like
        
        ```Scss
        /deep/ .el-table {
          &__body-wrapper::-webkit-scrollbar-thumb {
        ...
        ```
        
        looks a bit cleaner when you use the current selector ampersand whenever you have `parent_child` you can do `&__child` inside the parent scope.
        
    - Variables
        
        In SCSS you can declare variables easily. To create a variable just add a `$`sign to the variable name and set them like a normal CSS property.
        
        ```Scss
        // Font-weight for a project
        $font-light : 400
        $font-thick: 500
        $font-heavy: 600
        .element{ Font-weight: $font-light;
        }
        //color palette for a project to maintain consistency
        $product-dark-blue: \#324e85
        $product-light-blue:\#4c7396
        $product-lighter-blue:\#9bb7cf
        .element{ color: $product-dark-blue;
        }
        ```
        
          
        
    - Nesting
        
        SCSS allows you to nest CSS rules
        
        ```Scss
        // vanilla CSS
        .container { Width: 100%; Color: grey; Background-color: green;
        }
        .container div { Border: 1px solid black;
        }
        .container div a { text-decoration: none; color: \#f2f2f2;
        }
        .container div a::hover { color: \#b2b2b2;
        }
        ```
        
        ```Scss
        // SCSS
        .container { width: 100%; color: grey; background-color: green; div { border: 1px solid black; a { text-decoration: none; color: \#f2f2f2; &::hover { color: \#b2b2b2; } } }
        }
        ```
        
    - Inheritance
        
        @extend
        
        `@extends` helps you inherit the properties of another class.
        
        ```Scss
        // SCSS
        .header { color: grey;
        }
        .sub-header { @extend .header; font-size: 40px;
        }
        ```
        
        Below is what the SCSS compiles to
        
        ```CSS
        // Compiled CSS
        .header, .sub-header { color: grey;
        }
        .sub-header { font-size: 40px;
        }
        ```
        
        Mixin
        
        Mixin is another way SCSS implement inheritance using `@mixin`. You can achieve the same effects of `@extend` using `mixin`. You first create the mixin using `@mixin` then add it to any class that needs that property using `@include`
        
        ```Scss
        // SCSS
        //create mixin
        @mixin red-color{ color: grey;
        }
        .header{ @include red-color; /* add mixin */
        }
        .sub-header{ @include red-color; font-size: 40px;
        }
        ```
        
        ```CSS
        .header { color: grey;
        }
        .sub-header { color: grey; font-size: 40px;
        }
        ```
        
        Can you see the different between the compiled `css` of `@extend` and `@mixin`, `@mixin` puts the style in both selectors(header and sub-header) while `@extend` seperates both seletors with a comma and then apply this style to them.
        
        There is one thing though `@mixin` can do that `@extend` cannot, That is pass parameters and use it. `@mixin` can also take default values for the parameter.
        
        ```Scss
        // SCSS
        @mixin fontSize ($params: 10px){ font-size: $params;
        }
        .header{ @include fontSize(20px);
        }
        .sub-header{ @include fontSize(20px);
        }
        ```
        
        ```Scss
        // Compiled CSS
        .header { font-size: 20px;
        }
        .sub-header { font-size: 20px;
        }
        ```
        
        You can use any of them if you need to inherit a class but the best practice according to [CSS tricks](https://css-tricks.com/the-extend-concept/) is to use @extend when you are not passing parameters.
        
    - Import
        
        SCSS allow you to import other SCSS stylesheet into a SCSS file using `@import`
        
        ```Scss
        @import “button.scss” or @import “button”
        ```
        
    - Partials
        
        Partials are SCSS files you don’t what to be compiled to CSS but you want to import them(using `@import`) into another file. To create a partial you just need to add an underscore to the beginning of the file name `_font.scss`, then you can import them with or without the underscore. Partials help to modularize your code and separate concerns. For example in my projects I partials for colors, fonts, buttons e.t.c then I import them into a main.scss file.
        
        ```Scss
        /* _colors.scss(partial) */
        $light-gray: \#F2F2F2
        $dark-gray: \#737373
        /*EOF colors.scss*/
        /* _buttons.scss(partial) */
        .button-primary{ color: \#4c7396; background-color: \#ffffff;
        }
        .button-secondary{ background: \#4c7396; color: \#FFFFFF;
        }
        /*EOF buttons.scss*/
        /* main.scss */
        @import "_buttons.scss"
        @import "_colors.scss"
        ```
        
          
        
    - Operators
        
        Scss offers you different kinds of operators that you can use in your CSS. Arithmetic operators like :
        
        - Addition(+)
        - Subtraction(-)
        - Division(/)
        - Multiplication(*) e.t.c
        
        ```Scss
        // SCSS
        @mixin top-margin ($margin){ margin-top: 30px + $margin;
        }
        .container{ width: 800px - 80px; @include top-margin(10px);
        }
        //Compiled CSS
        .container { width: 720px; margin-top: 40px;
        }
        ```
        
        Note that arithmetic operators only work when both values use the same unit i.e `rem`, `em`, `px`
        
        They also support comparison operators like `==`, `!=`, `<`, `>`, `>=`, `<=` and logical operators `and`, `or`, `not`
        
          
        
    - Color Functions
        
        Scss provides some function that can be used to manipulate colors. Some of them include:
        
        - `mix($colorX, $colorY, weight)` : This function is used to mix two color together. The first argument is the first color, second is the second color and the third argument is the percentage of the first color you want to mix.
        
        ```Scss
        mix(blue, grey, 30%) /*results 30% blue and 70% grey*/
        ```
        
        - `lighten($color, $amount)`: this function is used to return a lighter color. The first argument is the color and the second is the percentage of how much you want to lighten it.
        
        ```Scss
        lighten(\#ff0000, 30 ) /*results \#ff9999/*
        ```
        
        - `darken($color, $amount)`: Takes similar arguments as lighten function but this returns a darker color specified.
        
        ```Scss
        darken(\#ff0000, 30 ) /*results \#660000*/
        ```
        
        - `opacify($color, $amount)`: This function returns a color with the opacity increase. The first argument is the color and the second is a value between 0 and 1.
        - `transparentize($color, $amount)`: This function makes a color more transparent, it takes similar arguments to opacify. It returns a color with the opacity reduced. You can say it is the opposite of opacify.
    - Loops
        
        ```Scss
        /* Generate utility classes for font-size */
        @for $x from 1 through 70 { .font-size-#{$x} { font-size: 0px + $x; }
        }
        ```
        
        So I can easily have a class of font-size-20 if I need that on an element.
        
        ```Scss
        /* Generate utility classes for margin */
        @for $i from 0 through 500 { 
        .m#{$i} { margin: 0px + $i; } 
        .mt#{$i} { margin-top: 0px + $i; } 
        .mb#{$i} { margin-bottom: 0px + $i; } 
        .ml#{$i} { margin-left: 0px + $i; } 
        .mr#{$i} { margin-right: 0px + $i; }
        }
        ```
        
          
        
    - Conditionals
        
        ```Scss
        $bg: pink;
        $bg-mobile: red;
        
        p {
          @if $bg == pink {
            color: blue;
          } @else if $bg-mobile == red {
            color: green;
          } @else {
            color: grey;
          }
        }
        ```
        
- Javascript
    
    - Export default
        
        If a module defines a default export:
        
        ```JavaScript
        export default function() { console.log("hello!") }
        ```
        
        then you can import that default export by omitting the curly braces:
        
        ```JavaScript
        import foo from "foo";
        foo(); // hello!
        ```
        
    - Named parameters and positional parameters
        
        **Named parameters and options.** The optional parameters of a method are sometimes called its options. Positional parameters are awkward if parameters can be omitted anywhere (not just at the end). Then any subset of the options can be active and one often has to insert blanks such as null for the inactive ones. Named parameters help with options in two ways: They clearly describe what parameters are active and they allow one to simply drop the inactive ones.
        
        **JavaScript.** JavaScript does not have native support for named parameters like Python and many other languages. But there is a reasonably elegant simulation: provide optional parameters via an object literal. The result is a so-called options object that is assigned to a single formal parameter. Using this technique, an invocation of select() looks as follows:
        
        ```Plain
        entries.select({ from: 3, to: 5 });
        ```
        
        The method receives an options object with the properties
        
        from and to. You can drop any of the options:
        
        ```Plain
        entries.select({ from: 3 });
        entries.select({ to: 5 });
        entries.select();
        ```
        
        Obviously, you could also introduce positional parameters. It is customary for the options to come last, but JavaScript does not force you to do so.
        
        ```Plain
        entries.select(value1, value2, { from: 3, to: 5 });
        ```
        
          
        
        **EcmaScript 6.**
        
        In ECMAScript 6 (ES6), we get a much more comfortable solution (
        
        [reminder](https://mail.mozilla.org/pipermail/es-discuss/2011-November/018670.html) via Brendan Eich):
        
        ```Plain
        Entries.prototype.select = function (
          { from = 0, to = this.length }) {
          
          // Use `from` and `to`
        };
        ```
        
        Allen Wirfs-Brock
        
        [points out](https://mail.mozilla.org/pipermail/es-discuss/2011-November/018673.html)
        
        that if you want people to be able to omit the options object then the above changes to:
        
        ```Plain
        Entries.prototype.select = function (
          { from = 0, to = this.length } = {}) {
          
          // Use `from` and `to`
        };
        ```
        
        JavaScript engines should be able to optimize invocations of functions and methods with ES6-style named parameters such that no temporary object is created.
        
        **Read more -**
        
        [https://2ality.com/2011/11/keyword-parameters.html](https://2ality.com/2011/11/keyword-parameters.html)
        
        [https://medium.com/@afontcu/cool-javascript-9-named-arguments-functions-that-get-and-return-objects-337b6f8cfa07](https://medium.com/@afontcu/cool-javascript-9-named-arguments-functions-that-get-and-return-objects-337b6f8cfa07)
        
    - Parsing or Unescaping HTML
        
        using the dom parser for this since there are way more escaped elements than just `<,>,&`
        
        ```JavaScript
          const doc = new DOMParser().parseFromString(htmlStr, 'text/html');  
        	return doc.documentElement.textContent;
        ```
        
          
        
    - Anonymous functions
        
        An anonymous function is a function that was declared without any named identifier to refer to it. As such, an anonymous function is usually not accessible after its initial creation.
        
        Normal function definition:
        
        ```JavaScript
        function hello() { alert('Hello world');
        }
        hello();
        ```
        
        **Anonymous function definition**:
        
        ```JavaScript
        var anon = function() { alert('I am anonymous');
        }
        anon();
        ```
        
        One common use for anonymous functions is as arguments to other functions. Another common use is as a closure.  
          
        
        Use as an **argument to other functions**:
        
        ```JavaScript
        setTimeout(function() { alert('hello');
        }, 1000);
        ```
        
        Above, the anonymous function is passed to setTimeout, which will execute the function in 1000 milliseconds.
        
    - Destructuring
        
        The destructuring assignment syntax is a JavaScript expression that makes it possible to unpack values from arrays, or properties from objects, into distinct variables.
        
        ```JavaScript
        let a, b, rest;
        [a, b] = [10, 20];
        console.log(a); // 10
        console.log(b); // 20
        
        [a, b, ...rest] = [10, 20, 30, 40, 50];
        console.log(a); // 10
        console.log(b); // 20
        console.log(rest); // [30, 40, 50]
        
        ({ a, b } = { a: 10, b: 20 });
        console.log(a); // 10
        console.log(b); // 20
        
        
        // Stage 4(finished) proposal
        ({a, b, ...rest} = {a: 10, b: 20, c: 30, d: 40});
        console.log(a); // 10
        console.log(b); // 20
        console.log(rest); // {c: 30, d: 40}
        ```
        
          
        
    - Parcel
        
        Install Parcel by doing `npm install -D parcel-bundler`.
        
        Now inside of your `package.json` put:
        
        `"scripts" { "dev": "parcel src/index.html" }`
        
        Now open [http://localhost:1234](http://localhost:1234/). You should see your site come up. The difference here is now it's being run through Parcel which means we can leverage all the features that Parcel allows us which we will explore shortly.
        
        So how does it work? We gave the entry point, which is index.html. It then reads that index.html file and finds its dependencies, which are the two React files and the oneApp.js file that we linked to. It's smart enough to detect that those two React files are remote so it doesn't do anything with those, but it sees that App.js is local and so it reads it and compiles its dependencies. Right now it has no dependencies so let's fix that.
        
    - ESLint
        
        First of all, run `npm install -D eslint eslint-config-prettier` to install eslint in your project development dependencies. Then you may configure its functionalities.
        
        There are dozens of present configs for ESLint and you're welcome to use any one of them. The Airbnb config is very popular, as is the standard config (both of which I taught in previous versions of this class). I'm going to use a looser one for this class: `eslint:recommended`. Let's create an `.eslintrc.json` file to start linting our project.
        
        Create this file called `.eslintrc.json`.
        
        ```JavaScript
        { "extends": ["eslint:recommended", "prettier", "prettier/react"], 
        "plugins": [], "parserOptions": { "ecmaVersion": 2018, "sourceType": "module", "ecmaFeatures": { "jsx": true } }, 
        "env": { "es6": true, "browser": true, "node": true }
        }
        ```
        
        This is a combination of the recommended configs of ESLint and Prettier. This will lint for both normal JS stuff as well as JSX stuff. Run `npx eslint script.js` now and you should see we have a few errors. Run it again with the `--fix` flag and see it will fix some of it for us! Go fix the rest of your errors and come back. Let's go add this to our npm scripts.
        
        `"lint": "eslint \"src/**/*.{js,jsx}\" --quiet",`
        
        Worth adding three things here:
        
        - With npm scripts, you can pass additional parameters to the command if you want. Just add a `--` and then put whatever else you want to tack on after that. For example, if I wanted to get the debug output from ESLint, I could run `npm run lint -- --debug` which would translate to `eslint **/*.js --debug`.
        - We can use our fix trick this way: `npm run lint -- --fix`.
        - We're going to both JS and JSX.
    - ESLint + React
        
          
        
        We need to give ESLint a hand to get it to recognize React and not yell about React not being used. Right now it thinks we're importing React and not using because it doesn't know what to do with React. Let's help it.
        
        Run this: `npm install -D babel-eslint eslint-plugin-import eslint-plugin-jsx-a11y eslint-plugin-react`
        
        Update your .eslintrc.json to:
        
        ```JavaScript
        { "extends": [ "eslint:recommended", "plugin:import/errors", "plugin:react/recommended", "plugin:jsx-a11y/recommended", "prettier", "prettier/react" ], 
        "rules": { "react/prop-types": 0 }, 
        "plugins": ["react", "import", "jsx-a11y"], 
        "parserOptions": { "ecmaVersion": 2018, "sourceType": "module", "ecmaFeatures": { "jsx": true } }, 
        "env": { "es6": true, "browser": true, "node": true }, 
        "settings": { "react": { "version": "detect" } }
        }
        ```
        
          
        
    - Type Coercion
        
        Type coercion can be explicit and implicit.
        
        When a developer expresses the intention to convert between types by writing the appropriate code, like `Number(value)`, it’s called **explicit type coercion** (or type casting).
        
        Since JavaScript is a weakly-typed language, values can also be converted between different types automatically, and it is called **implicit type coercion**. It usually happens when you apply operators to values of different types, like`1 == null`, `2/’5'`, `null + new Date()`, or it can be triggered by the surrounding context, like with `if (value) {…}`, where `value` is coerced to boolean.
        
        One operator that does not trigger implicit type coercion is `===`, which is called the strict equality operator. The loose equality operator `==` on the other hand does both comparison and type coercion if needed.
        
    - Arrow Function
        
        An arrow function expression is a syntactically compact alternative to a regular function expression, although without its own bindings to the this, arguments, super, or new.target keywords. Arrow function expressions are ill-suited as methods, and they cannot be used as constructors.
        
        ```JavaScript
        var elements = [
          'Hydrogen',
          'Helium',
          'Lithium',
          'Beryllium'
        ];
        
        // This statement returns the array: [8, 6, 7, 9]
        elements.map(function(element) {
          return element.length;
        });
        
        // The regular function above can be written as the arrow function below
        elements.map((element) => {
          return element.length;
        }); // [8, 6, 7, 9]
        
        // When there is only one parameter, we can remove the surrounding parentheses
        elements.map(element => {
          return element.length;
        }); // [8, 6, 7, 9]
        
        // When the only statement in an arrow function is `return`, we can remove `return` and remove
        // the surrounding curly brackets
        elements.map(element => element.length); // [8, 6, 7, 9]
        
        // In this case, because we only need the length property, we can use destructuring parameter:
        // Notice that the `length` corresponds to the property we want to get whereas the
        // obviously non-special `lengthFooBArX` is just the name of a variable which can be changed
        // to any valid variable name you want
        elements.map(({ length: lengthFooBArX }) => lengthFooBArX); // [8, 6, 7, 9]
        
        // This destructuring parameter assignment can also be written as seen below. However, note that in
        // this example we are not assigning `length` value to the made up property. Instead, the literal name
        // itself of the variable `length` is used as the property we want to retrieve from the object.
        elements.map(({ length }) => length); // [8, 6, 7, 9] 
        ```
        
        Before arrow functions, every new function defined its own [`this`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/this) value based on how the function was called:
        
        - A new object in the case of a constructor.
        - `undefined` in [strict mode](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Strict_mode) function calls.
        - The base object if the function was called as an "object method".
        - etc.
        
        This proved to be less than ideal with an object-oriented style of programming.
        
        ```JavaScript
        function Person() {
          // The Person() constructor defines `this` as an instance of itself.
          this.age = 0;
        
          setInterval(function growUp() {
            // In non-strict mode, the growUp() function defines `this`
            // as the global object (because it's where growUp() is executed.), 
            // which is different from the `this`
            // defined by the Person() constructor.
            this.age++;
          }, 1000);
        }
        
        var p = new Person();
        ```
        
        In ECMAScript 3/5, the `this` issue was fixable by assigning the value in `this` to a variable that could be closed over.
        
        ```JavaScript
        function Person() {
          var that = this;
          that.age = 0;
        
          setInterval(function growUp() {
            // The callback refers to the `that` variable of which
            // the value is the expected object.
            that.age++;
          }, 1000);
        }
        ```
        
        Alternatively, a [bound function](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/bind) could be created so that a preassigned `this` value would be passed to the bound target function (the `growUp()` function in the example above).
        
        An arrow function does not have its own `this`. The `this` value of the enclosing lexical scope is used; arrow functions follow the normal variable lookup rules. So while searching for `this` which is not present in the current scope, an arrow function ends up finding the `this` from its enclosing scope.
        
        Thus, in the following code, the `this` within the function that is passed to `setInterval` has the same value as the `this` in the lexically enclosing function:
        
        ```JavaScript
        function Person(){
          this.age = 0;
        
          setInterval(() => {
            this.age++; // |this| properly refers to the Person object
          }, 1000);
        }
        
        var p = new Person();
        ```
        
        Arrow functions come with 3 shorthands, meaning even shorter functions
        
        - The parentheses around parameters can be removed if there is just one
        - Curly braces for function body can be removed entirely
        - No need for the return keyword; arrow functions have an implicit return (they return by default without curly braces
    - Replace all
        
        We know that the string.replace() function replaces only the first occurrence. You can replace all the occurrences by adding /g at the end of the regex.
        
        ```JavaScript
        var example = "potato potato";
        console.log(example.replace(/pot/, "tom"));
        // "tomato potato"
        console.log(example.replace(/pot/g, "tom"));
        // "tomato tomato"
        ```
        
          
        
    - Extract unique values
        
        We can create a new array only with the unique values by using the Set object and the Spread operator.
        
        ```JavaScript
        var entries = [1, 2, 2, 3, 4, 5, 6, 6, 7, 7, 8, 4, 2, 1]
        var unique_entries = [...new Set(entries)];
        console.log(unique_entries);
        // [1, 2, 3, 4, 5, 6, 7, 8]
        ```
        
    - Flatten multi-dimensional array
        
        Simply by using the Spread operator.
        
        ```JavaScript
        var entries = [1, [2, 5], [6, 7], 9];
        var flat_entries = [].concat(...entries);
        ```
        
          
        
    - Dynamic Property names
        
        I always thought that I first had to declare an object before being able to assign a dynamic property.
        
        ```JavaScript
        const dynamic = 'flavour';
        var item = { name: 'Coke', [dynamic]: 'Cherry'
        }
        console.log(item);
        // { name: "Coke", flavour: "Cherry" }
        ```
        
    - Use length to resize an array
        
        We basically overwrite the length of the array.
        
        If we want to resize the array:
        
        ```JavaScript
        var entries = [1, 2, 3, 4, 5, 6, 7];
        console.log(entries.length);
        // 7
        entries.length = 4;
        console.log(entries.length);
        // 4
        console.log(entries);
        // [1, 2, 3, 4]
        ```
        
        If we want to empty the array:
        
        ```JavaScript
        var entries = [1, 2, 3, 4, 5, 6, 7];
        console.log(entries.length);
        // 7
        entries.length = 0;
        console.log(entries.length);
        // 0
        console.log(entries);
        // []
        ```
        
    - Spread operator
        
        Extracting key/value pairs of one object, and adding them as children of another object, is a very common scenario. Historically, there have been a few ways to accomplish this, but all of those methods are pretty clunky:
        
        ```JavaScript
        const obj1 = { dog: 'woof' };
        const obj2 = { cat: 'meow' };
        const merged = Object.assign({}, obj1, obj2);
        console.log(merged) // prints { dog: 'woof', cat: 'meow' }
        ```
        
        This pattern is incredibly common, so the above approach quickly becomes tedious. Thanks to the "spread operator" there's never a need to use it again:
        
        ```JavaScript
        const obj1 = { dog: 'woof' };
        const obj2 = { cat: 'meow' };
        console.log({ ...obj1, ...obj2 }); // prints { dog: 'woof', cat: 'meow' }
        ```
        
        The great part is, this also works seamlessly with arrays:
        
        ```JavaScript
        const arr1 = [1, 2];
        const arr2 = [3, 4];
        console.log([ ...arr1, ...arr2 ]); // prints [1, 2, 3, 4]
        ```
        
    - Template literals
        
        Template literals natively, and conveniently solve the two biggest problems with writing strings, adding dynamic content, and writing strings that bridge multiple lines:
        
        ```JavaScript
        const name = 'Ryland';
        const helloString =
        `Hello ${name}`;
        ```
        
    - Event Loop
        
        JS is single-threaded, but not single-file (as in lines at school). Even though it isn't parallel, it's still concurrent. Sending an HTTP request may take seconds or even minutes, if JS stopped executing code until a response came back from the request, the language would be unusable.
        
        JavaScript solves this with an [event loop](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop). The event loop, loops through registered events and executes them based on internal scheduling/prioritization logic. This is what enables sending 1000's of "simultaneous" HTTP requests or reading multiple files from disk at the "same time". Here's the catch, JavaScript can only utilize this capability if you utilize the correct features. The most simple example is the for loop:
        
        ```JavaScript
        let sum = 0;
        const myArray = [1, 2, 3, 4, 5, ... 99, 100];
        for (let i = 0; i < myArray.length; i += 1) { sum += myArray[i];
        }
        ```
        
        In JavaScript, traditional for loops should only be used if absolutely necessary. Otherwise, utilize the following constructs:
        
        - _map_
            
            ```JavaScript
            // in decreasing relevancy :0
            const urls = ['google.com', 'yahoo.com', 'aol.com', 'netscape.com'];
            const resultingPromises = urls.map((url) => makHttpRequest(url));
            const results = await Promise.all(resultingPromises);
            ```
            
        - _map with index_
            
            ```JavaScript
            // in decreasing relevancy :0
            const urls = ['google.com', 'yahoo.com', 'aol.com', 'netscape.com'];
            const resultingPromises = urls.map((url, index) => makHttpRequest(url, index));
            const results = await Promise.all(resultingPromises);
            ```
            
        - _for-each_
            
            ```JavaScript
            const urls = ['google.com', 'yahoo.com', 'aol.com', 'netscape.com'];
            // note this is non blocking
            urls.forEach(async (url) => { try { await makHttpRequest(url); } catch (err) { console.log(`${err} bad practice`); }
            });
            ```
            
        
        **Why use these over vanilla for loops ?**
        
        Instead of executing each "iteration" in order (sequentially), constructs such as map take all of the elements and submit them as individual events to the user-defined map function. This directly communicates to the runtime, that the individual "iterations" have no connection or dependence to each other, allowing them to run concurrently.
        
    - Numbers
        
        Numbers in JavaScript just suck, always use a radix parameter with `parseInt`.
        
    - DOM
        - Main parts of a browser
            - Window
                
                The window is the browser tab that a web page is loaded into; this is represented in JavaScript by the Window object. Using methods available on this object you can do things like return the window's size (see Window.innerWidth and Window.innerHeight), manipulate the document loaded into that window, store data specific to that document on the client-side (for example using a local database or other storage mechanism), attach an event handler to the current window, and more.
                
            - Navigator
                
                The navigator represents the state and identity of the browser (i.e. the user-agent) as it exists on the web. In JavaScript, this is represented by the Navigator object. You can use this object to retrieve things like the user's preferred language, a media stream from the user's webcam, etc.
                
            - Document
                
                The document (represented by the DOM in browsers) is the actual page loaded into the window, and is represented in JavaScript by the Document object. You can use this object to return and manipulate information on the HTML and CSS that comprises the document, for example get a reference to an element in the DOM, change its text content, apply new styles to it, create new elements and add them to the current element as children, or even delete it altogether.
                
        - Document Object Model
            
            The document currently loaded in each one of your browser tabs is represented by a document object model. This is a "tree structure" representation created by the browser that enables the HTML structure to be easily accessed by programming languages — for example the browser itself uses it to apply styling and other information to the correct elements as it renders a page, and developers like you can manipulate the DOM with JavaScript after the page has been rendered.
            
            As an example -
            
            ```HTML
            <!DOCTYPE html>
            <html>
            <head>
            	<meta charset="utf-8">
            		<title>Simple DOM example</title>
            </head>
            <body>
            	<section>
            		<img src="dinosaur.png" alt="A red Tyrannosaurus Rex: A two legged dinosaur standing upright like a human, with small arms, and a large head with lots of sharp teeth.">
            		<p>Here we will add a link to the 
            		<a href="https://www.mozilla.org/">Mozilla homepage</a>
            		</p>
            	</section>
            </body>
            </html>
            ```
            
            DOM for this ⇒
            
              
            
            ![[/Untitled 2.png|Untitled 2.png]]
            
            **Note: You can use DOM tree diagram creator -** [**Live DOM viewer**](https://software.hixie.ch/utilities/js/live-dom-viewer/)**.**
            
            - Node
                
                You can see here that each element and bit of text in the document has its own entry in the tree — each one is called a **node**. You will also encounter various terms used to describe the type of node, and their position in the tree in relation to one another:
                
                - **Element node**: An element, as it exists in the DOM.
                - **Root node**: The top node in the tree, which in the case of HTML is always the `HTML` node (other markup vocabularies like SVG and custom XML will have different root elements).
                - **Child node**: A node _directly_ inside another node. For example, `IMG` is a child of `SECTION` in the above example.
                - **Descendant node**: A node _anywhere_ inside another node. For example, `IMG` is a child of `SECTION` in the above example, and it is also a descendant. `IMG` is not a child of `BODY`, as it is two levels below it in the tree, but it is a descendant of `BODY`.
                - **Parent node**: A node which has another node inside it. For example, `BODY` is the parent node of `SECTION` in the above example.
                - **Sibling nodes**: Nodes that sit on the same level in the DOM tree. For example, `IMG` and `P` are siblings in the above example.
                - **Text node**: A node containing a text string.
            
              
            
        - DOM Manipulation
            - Selecting an element
                
                ```JavaScript
                const link = document.querySelector('a');
                ```
                
                `Document.querySelector()` is the recommended modern approach, which is convenient because it allows you to select elements using CSS selectors. The above `querySelector()` call will match the first `<a>` element that appears in the document. If you wanted to match and do things to multiple elements, you could use `Document.querySelectorAll()`, which matches every element in the document that matches the selector, and stores references to them in an array-like object called a NodeList
                
            - Manipulating an element
                
                Let's change the text inside the link by updating the value of the
                
                [`Node.textContent`](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)
                
                property. Add the following line below the previous one:
                
                ```JavaScript
                link.textContent = 'Mozilla Developer Network';
                ```
                
                We should also change the URL the link is pointing to, so that it doesn't go to the wrong place when it is clicked on. Add the following line, again at the bottom:
                
                ```JavaScript
                link.href = 'https://developer.mozilla.org';
                ```
                
                  
                
            
    - React
        
        1. One way data flow - data flows from parent to the child components but not the other way round.
        2. Props are immutable. A child can just receive the prop from the parent but not modify it.
        
        - Reach Router
            
            Let's quickly make a second page so we can switch between the two. Make a file called Details.js.
            
            ```JavaScript
            import React from "react";
            const Details = () => { return <h1>hi!</h1>;
            };
            export default Details;
            ```
            
            Now the Results page is its own component. This makes it easy to bring in the router to be able to switch pages. Run `npm install @reach/router`.
            
            Now we have two pages and the router available. Let's go make it ready to switch between the two. In `App.js`:
            
            ```JavaScript
            // at top
            import { Router } from "@reach/router";
            import Details from "./Details";
            // replace <SearchParams />
            <Router> <SearchParams path="/" /> <Details path="/details/:id" />
            </Router>;
            ```
            
            Now we have the router working! Try navigating to [http://localhost:1234/](http://localhost:1234/) and then to [http://localhost:1234/details/1](http://localhost:1234/details/1). Both should work!
            
              
            
            - With Reach Router, you make your component the Route component (unlike React Router) by giving it a path attribute. Reach Router then will find the path that it most matches. (It figures this out by a scoring algorithm that functions intuitively, similar to a CSS selector.)
            - The `:id` part is a variable. In `http://localhost:1234/details/1`, `1` would be the variable.
            - The killer feature of Reach Router is that it's really accessible. It manages things like focus so you don't have to. Pretty great.
            
            So now let's make the two pages link to each other. Go to Pet.js.
            
            ```JavaScript
            // at top
            import { Link } from "@reach/router";
            // change wrapping <a>
            <Link to={`/details/${id}`} className="pet"> […]
            </Link>;
            ```
            
            Now each result is a link to a details page! And that id is being passed as a prop to Details. Try replacing the return of Details with `return <h1>{props.id}</h1>;`. You should see the number.
            
            Let's make the Adopt Me! header clickable too.
            
            ```JavaScript
            // import Link too
            import { Router, Link } from "@reach/router";
            // replace h1
            <header> <Link to="/">Adopt Me!</Link>
            </header>;
            ```
            
        - Hooks
            
            When you type in the input, React detects that a DOM event happens. When that happens, React thinks _something_ may have changed so it runs a re-render. Providing your render functions are fast, this is a very quick operation. It then diffs what's currently there and what its render pass came up with. It then updates the minimum amount of DOM necessary.
            
            Notice we're using `className` instead of `class` on the HTML element for CSS classes. This is because `class` is a reserved word in JS and JSX is still just JS. So instead they opted to use `className` which is the [name of the JS API](https://developer.mozilla.org/en-US/docs/Web/API/Element/className) for interacting with class names.
            
            Like `className`, `htmlFor` is used because `for` is a reserved word in JS.
            
            So if we type in our input and it re-renders, what gets out in the `input` tag? Well, its value is tied to `location` and nothing changed that, so it remains the same. In other words, two way data binding is _not_ free in React. I say this is a feature because it makes you explicit on how you handle your data. Let's go make it work.
            
            ```JavaScript
            // in SearchParams.js
            import React, { useState } from "react";
            // replace location
            const [location, updateLocation] = useState("Seattle, WA");
            // replace input
            <input id="location" value={location} placeholder="Location" onChange={e => updateLocation(e.target.value)}
            />;
            ```
            
            - This is called a hook and as-of-writing one of the very newest features to React and a fundamental shift on how you'll approach to writing React.
            - A hook called such (in my head) because it's a hook that gets caught every time the render function gets called. Because the hooks get called in the same order every single time, they'll always point to the same piece of state. Because of that they can be stateful: you can keep pieces of mutable state using hooks and then modify them later using their provided updater functions.
            - An _absolutely key_ concept for you to grasp is hooks rely on this strict ordering. As such, **do not put hooks inside if statements or loops**. If you do, you'll have insane bugs that involve `useState` returning _the wrong state_. If you see `useState` returning the wrong piece of state, this is likely what you did.
            - Because the previous point is so absolutely critical, the React team has provided us with a lint rule that help us not fall into that trap. That lint rule relies on us, the developers, to follow the convention of calling our hooks `useXxxxxx`. If you're willing to do that, the lint rules will guard you from calling the hooks out of order.
            - The argument given to `useState` is the default value. In our case, we gave it `"Seattle, WA"` as our default value.
            - `useState` returns to us an array with two things in it: the current value of that state and a function to update that function. We're using a feature of JavaScript called destructuring to get both of those things out of the array.
            - We use the `updateLocation` function in the `onChange` attribute of the input. Every time the input is typed into, it's going to call that function which calls `updateLocation` with what has been typed into the input. When `updateLocation` is called, React knows that its state has been modified and kicks off a re-render.
            - You can make your own custom hooks; `useState` is just one of many.
            - You can use `useState` as many times as you need for various pieces of state! Again, this is why ordering is important because React relies on `useState` to be called in strictly the same order every time so it can give you the same piece of state.
            - Similar to above. We're using `onChange` and `onBlur` because it makes it more accessible.
            
            Let's make a third dropdown so you can select a breed as well as an animal.
            
            ```JavaScript
            // under your other state inside the component
            const [breed, updateBreed] = useState("");
            const [breeds, updateBreeds] = useState([]);
            // under the animal label
            <label htmlFor="breed"> Breed <select disabled={!breeds.length} id="breed" value={breed} onChange={e => updateBreed(e.target.value)} onBlur={e => updateBreed(e.target.value)} > <option /> {breeds.map(breed => ( <option key={breed} value={breed}> {breed} </option> ))} </select>
            </label>;
            ```
            
            So now we have a breed dropdown. The only really new thing we did was use the `disabled` property to disable the dropdown when you don't have any breeds. We're going to use the Petfinder API to request breeds based on the animal selected. If you select `dog`, you want to see poodles, labradors, and chihuahuas and parrots, tabbies, and Maine coons. Petfinder has and endpoint that if you give it a valid animal. We'll show you how to do that in the next lesson with effects.
            
            For now, we're going to make a custom hook of our own. Just like `useState` is a hook, there are a few others like `useEffect` (which we'll use in the next lesson), `useReducer` (for doing Redux-like reducers), `useRefs` (for when you need to have programmatic access to a DOM node), and `useContext` (for using React's context which we'll do shortly as well.) But like React hooks, we can use these hooks to make our re-usable hooks. Let's make one that creates a stateful dropdown for us so we can avoid duplication of our code.
            
            Make a new file called `useDropdown.js`. Noticed we prefixed it with `use` because it's a hook. You should follow that convention.
            
            ```JavaScript
            import React, { useState } from "react";
            const useDropdown = (label, defaultState, options) => { const [state, updateState] = useState(defaultState); const id = `use-dropdown-${label.replace(" ", "").toLowerCase()}`; const Dropdown = () => ( <label htmlFor={id}> {label} <select id={id} value={state} onChange={e => updateState(e.target.value)} onBlur={e => updateState(e.target.value)} disabled={!options.length} > <option /> {options.map(item => ( <option key={item} value={item}> {item} </option> ))} </select> </label> ); return [state, Dropdown];
            };
            export default useDropdown;
            ```
            
            This looks like we just made the previous dropdowns generic which is exactly what we did. We use hooks internally to keep the state and then we return the component and the state to the user via an array which can destructure later.
            
            In SearchParam.js
            
            ```JavaScript
            // import at the top
            import useDropdown from "./useDropdown";
            // delete the animal and breed useState calls
            // under breeds useState
            const [animal, AnimalDropdown] = useDropdown("Animal", "dog", ANIMALS);
            const [breed, BreedDropdown] = useDropdown("Breed", "", breeds);
            // replace animal and breed label
            <AnimalDropdown />
            <BreedDropdown />
            ```
            
              
            
        - Effects
            
            In SearchParams.js:
            
            ```JavaScript
            // at the top
            import React, { useState, useEffect } from "react";
            import pet, { ANIMALS } from "@frontendmasters/pet";
            // inside render method, below useDropdown calls
            useEffect(() => { pet.breeds("dog").then(console.log, console.error);
            });
            ```
            
            - Here we're using an effect to retrieve a list of breeds from the API. An effect is run after every render (which happens after state changes.) You're going to use effects to do things like AJAX calls, modify ambient state, integrate with other libraries, and many other things. Basically it's a way to delay work until after render happens and to deal with asynchronous side effects.
            - If you're familiar with previous versions of React, effects can take the place of _most_ life cycle methods. In this case we're going to use it instead of `componentDidMount` and `componentDidUpdate`.
            
            So rather than just having `dog` be the static animal, let's make that dynamic and let's make it actually save the breed it gets.
            
            ```JavaScript
            // replace effect
            useEffect(() => { updateBreeds([]); updateBreed(""); pet.breeds(animal).then(({ breeds }) => { const breedStrings = breeds.map(({ name }) => name); updateBreeds(breedStrings); }, console.error);
            }, [animal]);
            ```
            
            - Due to JavaScript closures (the fact that state is preserved for various render function calls) we're able to reference updateBreeds from the outer scope. We use this to update the breed after the successful call to the petfinder API.
            - The array at the end is peculiar but essential. By default, effects will run at the end of every re-render. This is problematic for us because we're updating breeds, which causes a re-render, which causes another effect, which causes another re-render, etc. What you can to prevent this spiral is give it an array of variables as a second parameter. Now this effect will only happen if one of those variables changes. In this case, it will only cause the effect if `animal` changes. Which is exactly what we want.
            - Effects are always called after the first render no matter what.
            - We have to pull the strings out of the objects from the API since the dropdown expects a list of strings, hence the map which does just that.
            
              
            
            Whenever a user selects a new animal, we need to programmatically update the breed. Since we put this into a custom hook, we have no way to do that. Let's go make it do that. In useDropdown.js:
            
            ```JavaScript
            // update return
            return [state, Dropdown, updateState];
            ```
            
            Now users can optionally programatically accept that function to update their components. Let's use this in the component. In SearchParams.js
            
            ```JavaScript
            // replace BreedDropdown declaration
            const [breed, BreedDropdown, updateBreed] = useDropdown("Breed", "", breeds);
            // first line of the function inside useEffect
            updateBreed("");
            ```
            
            Now it updates the breed to empty whenever you change animal since you can't have a poodle cat
            
        - Class Components
            1. One hard requirement is to have a `render()` method inside the class.
            
        - Elements and JSX
            
            - The basic syntax for a React element
            
            ```JavaScript
            // In a nutshell, JSX allows us to write HTML in our JS
            // JSX can use any valid html tags (i.e. div/span, h1-h6, form/input, etc)
            <div>Hello React</div>
            ```
            
            - JSX elements are expressions
            
            ```JavaScript
            // as an expression, JSX can be assigned to variables...
            const greeting = <div>Hello React</div>;
            const isNewToReact = true;
            // ... or can be displayed conditionally
            function sayGreeting() { if (isNewToReact) { // ... or returned from functions, etc. 
            return greeting; // displays: Hello React } 
            else { return <div>Hi again, React</div>; }
            }
            ```
            
            - JSX allows us to nest expressions
            
            ```JavaScript
            const year = 2020;
            // we can insert primitive JS values in curly braces: {}
            const greeting = <div>Hello React in {year}</div>;
            // trying to insert objects will result in an error
            ```
            
            - JSX allows us to nest elements
            
            ```JavaScript
            // to write JSX on multiple lines, wrap in parentheses: ()
            const greeting = ( // div is the parent element 
            	<div> {/* h1 and p are child elements */} 
            		<h1>Hello!</h1> 
            		<p>Welcome to React</p> 
            	</div>
            );
            // 'parents' and 'children' are how we describe JSX elements in relation
            // to one another, like we would talk about HTML elements
            ```
            
            - HTML and JSX have a slightly different syntax
            
            ```JavaScript
            // Empty div is not <div></div> (HTML), but <div/> (JSX)
            <div/>
            // A single tag element like input is not <input> (HTML), but <input/> (JSX)
            <input name="email" />
            // Attributes are written in camelcase for JSX (like JS variables
            <button className="submit-button">Submit</button> // not 'class' (HTML)
            ```
            
            - The most basic React app requires three things:
                - 1. ReactDOM.render() to render our app
                - 2. A JSX element (called a root node in this context)
                - 3. A DOM element within which to mount the app (usually a div with an id of root in an index.html file)
            
            ```JavaScript
            // imports needed if using NPM package; not if from CDN links
            import React from "react";
            import ReactDOM from "react-dom";
            const greeting = <h1>Hello React</h1>;
            // ReactDOM.render(root node, mounting point)
            ReactDOM.render(greeting, document.getElementById("root"));
            ```
            
              
            
        - Components and Props
            
            - The syntax for a basic React component
            
            ```JavaScript
            import React from "react";
            // 1st component type: function component
            function Header() { // function components must be capitalized unlike normal JS functions // note the capitalized name here: 'Header' 
            return <h1>Hello React</h1>;
            }
            // function components with arrow functions are also valid
            const Header = () => <h1>Hello React</h1>;
            // 2nd component type: class component
            // (classes are another type of function)
            class Header extends React.Component { // class components have more boilerplate (with extends and render method) 
            render() { return <h1>Hello React</h1>; }
            }
            ```
            
            - How components are used
            
            ```JavaScript
            // do we call these function components like normal functions?
            // No, to execute them and display the JSX they return...
            const Header = () => <h1>Hello React</h1>;
            // ...we use them as 'custom' JSX elements
            ReactDOM.render(<Header />, document.getElementById("root"));
            // renders: <h1>Hello React</h1>
            ```
            
            - Components can be reused across our app
            
            ```JavaScript
            // for example, this Header component can be reused in any app page
            // this component shown for the '/' route
            function IndexPage() { return ( <div> <Header /> <Hero /> <Footer /> </div> );
            }
            // shown for the '/about' route
            function AboutPage() { 
            return ( <div> 
            <Header /> 
            <About /> 
            <Testimonials /> 
            <Footer /> 
            </div> );
            }
            ```
            
              
            
            - Data can be dynamically passed to components with props
            
            ```JavaScript
            // What if we want to pass data to our component from a parent?
            // I.e. to pass a user's name to display in our Header?
            const username = "John";
            // we add custom 'attributes' called props
            ReactDOM.render( <Header username={username} />, document.getElementById("root")
            );
            // we called this prop 'username', but can use any valid JS identifier
            // props is the object that every component receives as an argument
            function Header(props) { // the props we make on the component (i.e. username) // become properties on the props object 
            return <h1>Hello {props.username}</h1>;
            }
            ```
            
            - Props must never be directly changed (mutated)
            
            ```JavaScript
            // Components must ideally be 'pure' functions.
            // That is, for every input, we be able to expect the same output
            // we cannot do the following with props:
            function Header(props) { // we cannot mutate the props object, we can only read from it 
            props.username = "Doug"; 
            return <h1>Hello {props.username}</h1>;
            }
            // But what if we want to modify a prop value that comes in?
            // That's where we would use state (see the useState section)
            ```
            
              
            
        - Styled Components
    - Variables in paths
        
        Variables in paths can be defined by adding : with the variable. For example -
        
        `/details/:id`
        
    - Performance
        - Javascript Compilation
            
            1. Javascript is a compiled language. However, we never actually compile to create a binary.
            2. Most browsers use something called JIT or Just in time compilers to compile JS moments before it is executed.
            
            - V8 Javascript Compiler - Chrome
                - Parsing
                    1. Parsing can be a slow process. (It is the process of converting the source code to the abstract syntax tree)
                    2. Two types of parsing - Eager or immediately parse and Lazy parsing or do the bare minimum now and parse later for real later.
                    3. Basic Rules
                        1. Scan through the top level scope. Parse all the code that you see actually does something.
                        2. Skip things like function declarations and classes for now. We will parse them when we need them.
                    4. Adding something in parenthesis gets eager parsed - Example - `( function(a, b) { return a + b; });`
                    5. Library called optimize-js can help with optimal parsing of javascript. ([https://nolanlawson.github.io/test-optimize-js/](https://nolanlawson.github.io/test-optimize-js/))
                    6. Try to avoid nesting functions.
                - AST
                    1. Go from a big long string of text to an actual data structure representing our code.
                    2. The baseline compiler takes the AST and starts to execute our code as we wrote it.
                - Optimizing Compiler (Turbofan)
                    - Speculative optimization
                        
                          
                        
                    - Hidden classes for dynamic lookups
                    - Function inlining
                    - Performance Measuring in Javascript
                        
                        ```JavaScript
                        const { performance } = require('perf_hooks');
                        
                        // SETUP 🏁
                        
                        let iterations = 1e7;
                        
                        const a = 1;
                        const b = 2;
                        
                        const add = (x, y) => x + y;
                        
                        // 🔚 SETUP
                        
                        performance.mark('start');
                        
                        // EXERCISE 💪
                        
                        while (iterations--) {
                          add(a, b);
                        }
                        
                        // 🔚 EXERCISE
                        
                        performance.mark('end');
                        
                        performance.measure('My Special Benchmark', 'start', 'end');
                        ```
                        
                        To see what is sent to the optimizing compiler, we can run the command - `node —trace-opt benchmark.js`. This will return the following:
                        
                        ```Shell
                        [marking 0x2d26c4238de1 <JSFunction (sfi = 0x2d268b45e479)> for optimized recompilation, reason: hot and stable]
                        [marking 0x2d2684b04a99 <JSFunction add (sfi = 0x2d268b45e541)> for optimized recompilation, reason: small function]
                        [compiling method 0x2d2684b04a99 <JSFunction add (sfi = 0x2d268b45e541)> using TurboFan]
                        [compiling method 0x2d26c4238de1 <JSFunction (sfi = 0x2d268b45e479)> using TurboFan OSR]
                        [optimizing 0x2d26c4238de1 <JSFunction (sfi = 0x2d268b45e479)> - took 0.168, 0.513, 0.021 ms]
                        [optimizing 0x2d2684b04a99 <JSFunction add (sfi = 0x2d268b45e541)> - took 0.438, 0.562, 0.008 ms]
                        [completed optimizing 0x2d2684b04a99 <JSFunction add (sfi = 0x2d268b45e541)>]
                        ```
                        
                        To get a less verbose response and more pertaining to the add function use the following - `node —trace-opt benchmark.js | grep add`.
                        
                        ```Shell
                        [marking 0x183a8c5c83b9 <JSFunction add (sfi = 0x183a0e05e541)> for optimized recompilation, reason: small function]
                        [compiling method 0x183a8c5c83b9 <JSFunction add (sfi = 0x183a0e05e541)> using TurboFan]
                        [optimizing 0x183a8c5c83b9 <JSFunction add (sfi = 0x183a0e05e541)> - took 0.480, 0.546, 0.009 ms]
                        [completed optimizing 0x183a8c5c83b9 <JSFunction add (sfi = 0x183a0e05e541)>]
                        ```
                        
                        Now, let us change the code a little bit and see the changes on the optimization.
                        
                        Add the following line between the mark start and end.  
                          
                        `add('foo', 'bar');`
                        
                        ```JavaScript
                        performance.mark('start');
                        
                        // EXERCISE 💪
                        
                        while (iterations--) {
                          add(a, b);
                        }
                        
                        // 🔚 EXERCISE
                        
                        add('foo', 'bar');
                        
                        performance.mark('end');
                        performance.mark('end');
                        ```
                        
                        And now run the following command to see whether we can observe a deoptimization after optimization. (Kind of like a rollback)
                        
                        ```Shell
                        [marking 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)> for optimized recompilation, reason: small function]
                        [compiling method 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)> using TurboFan]
                        [optimizing 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)> - took 0.487, 0.537, 0.008 ms]
                        [completed optimizing 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)>]
                        12: 0x1e5248ce7169 ; rcx 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)>
                        0x7ffeefbfeac0: [top + 48] <- 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)> ; stack parameter (input \#12)
                        [deoptimizing (DEOPT eager): begin 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)> (opt \#0) @0, FP to SP delta: 24, caller sp: 0x7ffeefbfea98]
                        reading input frame add => bytecode_offset=0, args=3, height=0, retval=0(\#0); inputs:
                        0: 0x1e5248ce7169 ; [fp - 16] 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)>
                        translating interpreted frame add => bytecode_offset=0, variable_frame_size=8, frame_size=80
                        0x7ffeefbfea60: [top + 24] <- 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)> ; function (input \#0)
                        [deoptimizing (eager): end 0x1e5248ce7169 <JSFunction add (sfi = 0x1e522c65e571)> @0 => node=0, pc=0x000100961220, caller sp=0x7ffeefbfea98, took 0.205 ms]
                        ```
                        
                        Why though ?
                        
                        Adding different types of arguments hits the performance hard.
                        
                          
                        
    - Template Literals
        
        - Template literals are much more powerful, dynamic strings than basic JavaScript strings using single or double quotes
            - Interpolating or inserting values into strings is much easier; uses the `${}` syntax to insert valid JS expressions
            - No need to use the + operator to concatenate or combine strings as before
            - Easier to write multiline strings
                - No need to write new lines with the newline character (`\n`) or carriage return (`\r`)
            - Can use nested quotes (with single or double quotes) within template literals strings without errors
        
        ```JavaScript
        const username = "Fred"; // connecting strings with the + operator is hard to read and not intuitive 
        const greeting = "Hi " + username + ", how are you?"; // template literals (``) are a breeze and much easier to write 
        const username = "Anna"; // dynamic values are inserted with the ${} syntax 
        const greeting = `Hi ${username}, how are you?`;
        ```
        
          
        
    - Default Parameters
        
          
        
        **Why use default parameters in JavaScript?**
        
        - To handle the event that a function that doesn't have values passed to it that it needs as arguments
        - Default parameters helps us prevent errors and have more predictable code by giving arguments default values (with the equals sign) if none are provided
        
        ```JavaScript
        // without default parameters
        function sayHi(name) { return "Hi" + name;
        }
        sayHi(); // "Hi undefined"
        // with default parameters
        function sayHi(name = 'Bob') { return "Hi" + name;
        }
        sayHi(); // "Hi Bob"
        // with default parameters using an arrow function
        const sayHi = (name = 'Jane') => "Hi" + name;
        sayHi(); // "Hi Jane"
        ```
        
        **How are default parameters used in React?**
        
        - Default parameters are often used in the case of props
        - In this example, we are using object destructuring to grab a prop called 'username' from the props object. But no prop value has been passed, so we set a default parameter value of 'guest' and our component can still works.
        
        ```JavaScript
        const Header = ({ username = "guest" }) => { return <header>Welcome, {username}!</header>; } <Header /> // displays: Welcome, guest!
        ```
        
    - Intersection, Union and Difference between 2 arrays
        
        ![[/Untitled 1 2.png|Untitled 1 2.png]]
        
          
        
    - Miscellaneous
        1. [https://yomguithereal.github.io/mnemonist/](https://yomguithereal.github.io/mnemonist/)
        2. [https://github.com/google/closure-library/](https://github.com/google/closure-library/)
        3. [https://www.npmjs.com/package/pino](https://www.npmjs.com/package/pino)
        4. [https://www.npmjs.com/package/json-api-serializer](https://www.npmjs.com/package/json-api-serializer)
        
    - Async Functions
        - Understanding
            
            ![[/Untitled 2 2.png|Untitled 2 2.png]]
            
    - XSS - Cross site scripting attack
        
        ![[Untitled 3.png]]
        
    - Boolean Logic
        
        ![[Untitled 4.png]]
        
    - Array Cheatsheet
        
        ![[Untitled 5.png]]
        
    - Counting the occurrences of substring inside another string
        
        ![[Untitled 6.png]]
        
    - Array from
        
        ![[Untitled 7.png]]
        
    - Check if var is integer
        
        ![[Untitled 8.png]]
        
    - Date time in Javascript
        
        ![[Untitled 9.png]]
        
    
    [[JS Knowledge]]
    
- Typescript
    - Why Typescript ?
        1. Helps in writing better code
- Docker
    - Restart Landing Page
        
        ```Shell
        d-c stop landing_page
        cd landing_page
        git checkout master -f
        git pull
        cd ..
        d-c pull landing_page
        d-c up -d landing_page
        ```
        
          
        
    - Restart PUMA
        
        d-c ps to check for running containers
        
        ```Bash
        d-c exec app bash
        bundle install
        rake db:migrate
        ```
        
        try inside the app bash supervisorctl status
        
        try supervisorctl start puma and then check again the status
        
    - Solve 504 Errors
        
        hmm gateway timeouts usually occur to me when I switch branches
        
        or maybe you forgot a debugger somewhere?
        
        usually d-c restart app fixes it
        
    - Docker-Compose Changes: infinte command
        
        Sometimes, you would want to run puma inside the container manually. But you would also want the container to run indefinitely. For this purpose, we can use an infinite command such as, for example,  
          
        `tail -F /dev/null`
        
        which basically is trying to read a null file.
        
        `echo 'Hi'`, on the other hand, will not work since it would just exit and the container would stop then.  
        Then we can run puma inside the app bash, like so,  
        
        `bundle exec puma -C /myapp/config/puma.rb`
        
        And now we can run the debugger. Yay!
        
    - Blank frontend - Dev environment memory full
        
        `docker system prune`
        
        then `stop pull up`.
        
        **Reason for the fill up of the memory -**
        
        containers are not cleaned so every time you pull containers it creates new ones and the old ones are left in there
        
    - Open docker in daemon mode
        
        To start a docker command in daemon mode just enter the -d flag.  
        This allows you to execute a docker command without showing any output (somewhat like a non-verbose execution)  
        
    - Simple explanation of docker
        
        ![[Untitled 10.png]]
        
    - Checking logs
        
        Inside the app container
        
        `tail -f log/development.log`
        
    - Testing on staging
        
        [https://console.cloud.google.com/kubernetes/deployment/europe-west3-a/demodesk-staging/demodesk-app/demodesk-app?project=demodesk-staging&tab=overview&deployment_overview_active_revisions_tablesize=50&duration=PT1H&pod_summary_list_tablesize=20&service_list_datatablesize=20](https://console.cloud.google.com/kubernetes/deployment/europe-west3-a/demodesk-staging/demodesk-app/demodesk-app?project=demodesk-staging&tab=overview&deployment_overview_active_revisions_tablesize=50&duration=PT1H&pod_summary_list_tablesize=20&service_list_datatablesize=20)
        
          
        
        ![[Untitled 11.png]]
        
- Ruby on Rails
    - Debugger
        
        we use byebug debugger
        
        so to use it its pretty simple
        
        add a line where you want it to stop with `debugger`
        
        then go to the app bash (d-c exec app bash) and do
        
        ---
        
        ```Bash
        ss -lptn 'sport = :$PORT' (Mention the port puma is running on => Generaly, 3000)
        bundle exec puma -C /myapp/config/puma.rb
        ```
        
        then you should be able to use the debugger in there
        
        **Note : if supervisorctl does not work, look at the docker compose changes in the docker tab on notion.**
        
    - Queries in Rails
        - Find a single element
            
            This method lets you supply the primary id of an object and retrieves that single object for you. If you provide an array of ids, you can also retrieve multiple objects.
            
            ### **Rails**
            
            ```Ruby
            bond = Agent.find(7)
            ```
            
            ### **SQL**
            
            ```SQL
            SELECT "agents".* FROM "agents" WHERE "agents"."id" = ? LIMIT 1 [["id", 7]]
            ```
            
              
            
        - Finding the first and last element
            
            ```Ruby
            enemy_agents = SpectreAgent.first(10)
            enemy_agents = SpectreAgent.last(10)
            ```
            
        - Find by some condition on a property
    - Opening IRB
        
        Run the following commands and enter the IRB
        
        `docker-compose exec app bash`
        
        `rails c`
        
    - Rescue and Retry
        
        Retry is built in to Ruby's exception rescuing system. It's quite simple. If you use "retry" in your rescue block it causes the section of code that was rescued to be run again. Let's look at an example.
        
        `**begin**``retries ||=` `**0**``puts "try ##{ retries }"` `**raise**` `"the roof"` `**rescue**``**retry**` `**if**` `(retries +=` `**1**``) <` `**3**``**end**``# ... outputs the following: # try \#0 # try \#1 # try \#2`
        
        There are a few things to note here:
        
        - When retry is called, **all** of the code in between begin and rescue is run again. It most definitely **does** **not** "pick up where it left off" or anything like that.
        - If you don't provide some mechanism to limit retries, you will wind up with an infinite loop.
        - Code in both the begin and rescue blocks are able to access the same retries variable in the parent scope.
        
          
        
        Read more ⇒
        
        > [!info] How to "try again" when exceptions happen in Ruby  
        > Not all errors are fatal.  
        > [https://www.honeybadger.io/blog/how-to-try-again-when-exceptions-happen-in-ruby/](https://www.honeybadger.io/blog/how-to-try-again-when-exceptions-happen-in-ruby/)  
        
    - Try
        
        The idea behind `Object\#try` is simple: instead of raising `NoMethodError` exception when calling some method on `nil` or calling a method on non-`nil` object that is not implemented by this object, it just returns `nil`.
        
        Imagine that you want to grab the email of the first user. To make sure it won’t blow up when there are no users, you could write it the following way:
        
        ```Ruby
        user.first.try(:email)
        ```
        
        Also accepts functions and their arguments
        
        ```Ruby
        host_email_before_handover = event['Extensions'].try(:first)
        																.try(:[], "HostHandoverEmail")
        ```
        
          
        
    - Safe navigation operator
        
        This means as of 2.3 below code
        
        ```Ruby
        account.try(:owner).try(:address)
        ```
        
        can be rewritten to
        
        ```Ruby
        account&.owner&.address
        ```
        
        It is also including a similar sort of way: `Array\#dig` and `Hash#dig`. So now this
        
        ```Ruby
        city = params.fetch(:[], :country).try(:[], :state).try(:[], :city)
        ```
        
        can be rewritten to
        
        ```Ruby
        city = params.dig(:country, :state, :city)
        ```
        
        Again, `\#dig` is not replicating `\#try`'s behaviour. So be careful with returning values. If `params[:country]` returns, for example, an Integer, `TypeError: Integer does not have #dig method` will be raised.
        
    - Performance Tips and Tricks
        
        Typhoeus - runs HTTP requests in parallel
        
        Bulk insert data using ActiveRecord
        
          
        
    - Explain
        
        You can now run EXPLAIN on the SQL generated by a relation this way:
        
        `User.where(:id => 1).joins(:posts).explain`
        
        The result of that method call is a string that carefully imitates the output of database shells. For example, under MySQL you get something similar to
        
        ```SQL
        EXPLAIN for: SELECT `users`.* FROM `users` INNER JOIN `posts` ON `posts`.`user_id` = `users`.`id` WHERE `users`.`id` = 1
        +----+-------------+-------+-------+---------------+---------+---------+-------+------+-------------+
        | id | select_type | table | type | possible_keys | key | key_len | ref | rows | Extra |
        +----+-------------+-------+-------+---------------+---------+---------+-------+------+-------------+
        | 1 | SIMPLE | users | const | PRIMARY | PRIMARY | 4 | const | 1 | |
        | 1 | SIMPLE | posts | ALL | NULL | NULL | NULL | NULL | 1 | Using where |
        +----+-------------+-------+-------+---------------+---------+---------+-------+------+-------------+
        2 rows in set (0.00 sec)
        ```
        
        and under PostgreSQL the same call yields something like
        
        ```SQL
        EXPLAIN for: SELECT "users".* FROM "users" INNER JOIN "posts" 
        ON "posts"."user_id" = "users"."id" WHERE "users"."id" = 1 QUERY PLAN
        ------------------------------------------------------------------------------ Nested Loop Left Join (cost=0.00..37.24 rows=8 width=0) Join Filter: (posts.user_id = users.id) -> Index Scan using users_pkey on users (cost=0.00..8.27 rows=1 width=4) Index Cond: (id = 1) -> Seq Scan on posts (cost=0.00..28.88 rows=8 width=4) Filter: (posts.user_id = 1)
        (6 rows)
        ```
        
    - How to use .nil? .empty? .blank? .present? in Rails
        
        [https://medium.com/le-wagon/how-to-use-nil-blank-present-exists-in-rails-5-fe03e78ab979](https://medium.com/le-wagon/how-to-use-nil-blank-present-exists-in-rails-5-fe03e78ab979)
        
        ![[Untitled 12.png]]
        
    - Dig
        
        **dig(key, ...) → objectclick to toggle source**
        
        Retrieves the value object corresponding to the each _key_ objects repeatedly.
        
        ```Ruby
        h = { foo: {bar: {baz: 1}}}
        
        h.dig(:foo, :bar, :baz)           
        #=> 1, h.dig(:foo, :zot)                 
        #=> nil
        ```
        
    - N+1 problem
        
        Your current solution is very heavy on the database. It not only fetches all users for a company from the database but also all identities of all those users. If a company has 100 users and each user has 5 identities, we just send 1 (for the users) + 100 queries (for the identities of each user) to the database, see also [N+1 problem](https://secure.phabricator.com/book/phabcontrib/article/n_plus_one/). We fetched 100 + 100 * 5 = 600 full rows from the database.
        
        And this happens just to serialize a single user. If we want to serialize a the full list of users, we would send 100 * (1 + 100) = 10,100 database queries and fetch 100 * 600 = 60,000 rows from the database.
        
        A good guideline is to only fetch the information we actually need here from the database. In this case, we only need a boolean answer to whether any of the employees of the user's company has a broken identity. We can use an EXIST query for that with `.exists?` from active record. Since we want to know if an identity matching certain criteria exists, we run it on `Identity`.
        
        ```Plain
        Identity.exists?(...)
        ```
        
        Now we have two matching criteria:
        
        1. The identity must belong to a user of the same company as our user
        2. The identity must be broken
        
        For the first part, we can do `user: User.where(company: object.company)`. Rails is smart enough to make a subquery out of this so that this does not result in an extra query but is combined with the main query on the Identity table. It's also smart enough to not fetch the company of object since it knows that we only need the id and it also doesn't fetch the complete users but only their ids.
        
        For the second part, we just do `broken: true`. Ideally, we would have a db index on the broken column to be able to filter these rows out very efficiently but that is not so important in this case since the user filter already brings down the result set size significantly.
        
        We end up with:
        
        ```Plain
        Identity.exists?(user: User.where(company: object.company), broken: true)
        ```
        
        The generated SQL query looks like this:
        
        ```Plain
        SELECT 1 AS one FROM "identities" WHERE "identities"."user_id" IN (SELECT "users"."id" FROM "users" WHERE "users"."deleted_at" IS NULL AND "users"."company_id" = $1) AND "identities"."broken" = $2 LIMIT $3
        ```
        
        Only a single boolean is fetched from the database and we only had to look at the user id (and deleted) column and the identity user_id and broken columns.
        
    - ! in ruby on rails
        
        The important thing to remember is that in Ruby a trailing `!` or `?` are allowed on method names and become _part of the method name_, not a modifier added on. `x` and `x!` and `x?` are three completely different methods.
        
        In Ruby the convention is to add `!` to methods that make _in-place modifications_, that is they modify the object in fundamental ways. An example of this is `String\#gsub` which returns a copy, and `String#gsub!` which modifies the string in-place.
        
        In Rails this has been ported over to mean that as well as situations where the method _will raise an exception on failure_ instead of returning `nil`. This is best illustrated here:
        
        ```Ruby
        Record.find_by(id: 10) # => Can return nil if not found
        Record.find_by!(id: 10) # => Can raise ActiveRecord::RecordNotFound
        ```
        
        Note that this is not always the case, as methods like `find` will raise exceptions even without the `!`. It's purely an informational component built into the method name and does not guarantee that it will or won't raise exceptions.
        
        _Update_:
        
        The reason for using exceptions is to make flow-control easier. If you're constantly testing for `nil`, you end up with highly paranoid code that looks like this:
        
        ```Ruby
        def updateif (user.save)
            if (purchase.save)
              if (email.sent?)
                redirect_to(success_path)
              else
                render(template: 'invalid_email')
              end
            else
              render(template: 'edit')
            end
          else
            render(template: 'edit')
          end
        end
        ```
        
        In other words, you always need to be looking over your shoulder to be sure nothing bad is happening. With exceptions it looks like this:
        
        ```Ruby
        def update
          user.save!
          purchase.save!
          email.send!
        
          redirect_to(success_path)
        
        rescue ActiveRecord::RecordNotFound
          render(template: 'edit')
        rescue SomeMailer::EmailNotSent
          render(template: 'invalid_email')
        end
        ```
        
        Where you can see the flow is a lot easier to understand. It describes "exceptional situations" as being less likely to occur so they don't clutter up the main code.
        
    - DB rollback
        
        ```Ruby
        rake db:rollback STEP=1
        ```
        
        Is a way to do this, if the migration you want to rollback is the last one applied. You can substitute 1 for however many migrations you want to go back.
        
        For example:
        
        ```Plain
        rake db:rollback STEP=5
        ```
        
        Will also rollback all the migration that happened later (4, 3, 2 and also 1).
        
        To roll back all migrations back to (and including) a target migration, use: (This corrected command was added AFTER all the comments pointing out the error in the original post)
        
        ```Plain
        rake db:migrate VERSION=20100905201547
        ```
        
        In order to rollback ONLY ONE specific migration (OUT OF ORDER) use:
        
        ```Plain
        rake db:migrate:down VERSION=20100905201547
        ```
        
        Note that this will NOT rollback any interceding migrations -- only the one listed. If that is not what you intended, you can safely run `rake db:migrate` and it will re-run only that one, skipping any others that were not previously rolled back.
        
        And if you ever want to migrate a single migration out of order, there is also its inverse `db:migrate:up`:
        
        ```Plain
        rake db:migrate:up VERSION=20100905201547
        ```
        
    - Option Pattern
        
        > [!info] Option pattern in Ruby  
        > Have you ever seen this when coding Ruby?  
        > [https://medium.com/bettersharing/option-pattern-in-ruby-7b0f7c5abdb6](https://medium.com/bettersharing/option-pattern-in-ruby-7b0f7c5abdb6)  
        
    - Inverse of
        
        [https://www.viget.com/articles/exploring-the-inverse-of-option-on-rails-model-associations/](https://www.viget.com/articles/exploring-the-inverse-of-option-on-rails-model-associations/)
        
- Database
    - SQL
        - Multi-Insert
            
            Wrap each row of values to be inserted in brackets/parenthesis `(value1, value2, value3)` and separate the brackets/parenthesis by comma for as many as you wish to insert into the table.
            
            ```SQL
            INSERT INTO example
            VALUES
              (100, 'Name 1', 'Value 1', 'Other 1'),
              (101, 'Name 2', 'Value 2', 'Other 2'),
              (102, 'Name 3', 'Value 3', 'Other 3'),
              (103, 'Name 4', 'Value 4', 'Other 4');
            ```
            
        - Coalesce
            
            Usage can be kind of similar to hashmap.get(value, default) ⇒ Used if value is null.
            
            ![[Untitled 13.png]]
            
        - JOIN
            
            ![[Untitled 14.png]]
            
    - Optimize database indexes
        
        Dont want to have a lot of indices on a database.
        
        Types of indices:
        
        1. multi-column
        2. partial
        3. ordered
        
        Default index is a B-tree which can be traversed in any way.
        
          
        
    - B-Trees
        - Disk Structure
            
              
            
            ![[Untitled 15.png]]
            
            1. The concentric circles are called tracks
            2. The triangular arcs are called sectors.
            3. Intersection between the sectors and tracks are called blocks.
            4. Block Address = (Track Number, Sector Number)
            5. Typical block size = 512 bytes
            6. Any one byte can be referenced by the offset inside the block.
            7. By the spinning of the spindle, sectors are changed.
            8. By the moving of the head, tracks are changed.
        - How data is stored on disk
            
              
            
        - What is indexing
        - What is multi-level indexing
        - M-way search trees
        - B-Trees
        - Insertion and deletion on B-trees
        - B+-trees
- Random
    
    - Demodesk docs
        
        [https://gitlab.com/demodesk/demodesk/-/wikis](https://gitlab.com/demodesk/demodesk/-/wikis/Analytics)
        
    - Linux
        - Find all processes running on a port
            
            `ss -lptn 'sport = :$PORT'`
            
        - Add ssh key to server
            
            **Step 1: Check for SSH Keys**
            
            First, check for existing SSH keys on your computer. Open Git Bash, Cygwin, or Terminal, etc. and enter:
            
            ```Plain
            $ ls -al ~/.ssh
            # Lists all the files in your .ssh directory, if they exist
            Generating public/private rsa key pair
            ```
            
            Check the directory listing to see if you already have a public SSH key. By default, the filenames of the public keys are one of the following:
            
            - id_dsa.pub
            - is_ecdsa.pub
            - id_ed25519.pub
            - id_rsa.pub
            
            If you see an existing public and private key pair listed (for example id_rsa.pub and id_rsa) that you’d like to use, you can skip Step 2 and go straight to [Step 3](https://adamdehaven.com/blog/how-to-generate-an-ssh-key-and-add-your-public-key-to-the-server-for-authentication/#step-3).
            
            # **Step 2: Generate a new SSH key**
            
            With your command line tool still open, enter the text shown below. Make sure you substitute in your email address:
            
            ```Plain
            $ ssh-keygen -t rsa -b 4096 -C "mcflym@N123456"
            # Creates a new ssh key, using the provided domain username and computer name as a label
            Generating public/private rsa key pair.
            ```
            
            You’ll be asked to enter a passphrase, or simply press Enter to not enter a passphrase:
            
            ```Plain
            Enter passphrase (empty for no passphrase):
            Enter same passphrase again:
            ```
            
            After you enter a passphrase (or just press Enter twice), review the fingerprint, or ‘id’ of your SSH key:
            
            ```Plain
            Your identification has been saved in /Users/username/.ssh/id_rsa.
            Your public key has been saved in /Users/username/.ssh/id_rsa.pub.
            The key fingerprint is:
            nss2VhNB0Y62VIToM+/qYe3HS4TPXmrhuBxjUz4l/I8= your@email.com
            ```
            
            # **Step 3: Add your key to the ssh-agent**
            
            To configure the ssh-agent program to use your SSH key, first ensure ssh-agent is enabled.
            
            ```Plain
            # start the ssh-agent in the background
            $ eval $(ssh-agent -s)
            Agent pid 59566
            ```
            
            If you are using Git Bash, turn on the ssh-agent with command shown below instead:
            
            ```Plain
            # start the ssh-agent in the background
            $ eval `ssh-agent`
            Agent pid 59566
            ```
            
            Then, add your SSH key to the ssh-agent:
            
            ```Plain
            $ ssh-add ~/.ssh/id_rsa
            ```
            
            # **Step 4: Add your SSH key to the server**
            
            To add your public SSH key to the server, you’ll copy the public SSH key you just created to the server. Substitute “username” with your username on the server, and “server.address.com” with the domain address or IP address of your server:
            
            ```Plain
            $ cat ~/.ssh/id_rsa.pub | ssh username@server.address.com 'cat >> ~/.ssh/authorized_keys'
            ```
            
            The server will then prompt you for your password:
            
            ```Plain
            username@server.address.com's password:
            ```
            
        - Find IP address for a website
            
            `nslookup websitename`
            
        - Changing DNS for a website
            
            `sudo nano /etc/hosts`  
            Use the IP address from above and the new domain name that you want to use  
            
    - Multiprocessing vs Multithreading
        
        you can use threading if your program is network bound or multiprocessing if it's CPU bound.
        
          
        
        **Multithreading**
        
        By nature, Python is a linear language, but the threading module comes in handy when you want a little more processing power. While threading in Python cannot be used for parallel CPU computation, it's perfect for I/O operations such as web scraping because the processor is sitting idle waiting for data.
        
        Threading is game-changing because many scripts related to network/data I/O spend the majority of their time waiting for data from a remote source. Because downloads might not be linked (i.e., scraping separate websites), the processor can download from different data sources in parallel and combine the result at the end. For CPU intensive processes, there is little benefit to using the threading module.
        
          
        
        **Multiprocessing**
        
        Without multiprocessing, Python programs have trouble maxing out your system's specs because of the `GIL` (Global Interpreter Lock). Python wasn't designed considering that personal computers might have more than one core (shows you how old the language is), so the GIL is necessary because Python is not thread-safe and there is a globally enforced lock when accessing a Python object. Though not perfect, it's a pretty effective mechanism for memory management. _What can we do?_
        
        Multiprocessing allows you to create programs that can run concurrently (bypassing the GIL) and use the entirety of your CPU core. Though it is fundamentally different from the threading library, the syntax is quite similar. The multiprocessing library gives each process its own Python interpreter and each their own GIL.
        
        Because of this, the usual problems associated with threading (such as data corruption and deadlocks) are no longer an issue. Since the processes don't share memory, they can't modify the same memory concurrently.
        
    - Git
        - Blame
            
            git blame is a very good tracking command for Git. git blame shows the author information of each line of the project’s last modified source file. You can find the author name, author email, the commit hash etc of the last modified source file line by line. You will see shortly what I mean practically.
            
            ### **Basic Usage:**
            
            To find the author and commit information of each line of the last modified version of the file (let’s say **myfile**) in your Git repository, you run **git blame** as follows:
            
            ```Shell
            $ git blame myfile
            ```
            
            In my Git repository, I have a file **gulpfile.babel.js**. Let’s say, I want to check the author and commit information of this file line by line. To do that, I would run git blame as follows:
            
            ```Shell
            $ git blame gulpfile.babel.js
            ```
            
            [![](https://linuxhint.com/wp-content/uploads/2019/06/4-35-1024x75.png)](https://linuxhint.com/wp-content/uploads/2019/06/4-35-1024x75.png)
            
        - Delete Branch
            
            ### Deleting _local_ branches in Git
            
            ```Shell
            $ git branch -d feature/login
            ```
            
            Using the "-d" flag, you tell "git branch" which item you want to delete.
            
            Note that you might also need the "-f" flag if you're trying to delete a branch that contains _unmerged changes_. Use this option with care because it makes losing data very easy.
            
            ### Deleting _remote_ branches in Git
            
            To delete a remote branch, we do **not** use the "git branch" command - but instead "git push" with the "--delete" flag:
            
            ```Shell
            $ git push origin --delete feature/login
            ```
            
        - Update Submodule
            
            Use the **git submodule update** command to set the submodules to the commit specified by the main repository. This means that if you pull in new changes into the submodules, you need to create a new commit in your main repository in order to track the **updates** of the nested submodules.  
              
            [https://www.vogella.com/tutorials/GitSubmodules/article.html](https://www.vogella.com/tutorials/GitSubmodules/article.html)
            
        - Cherry Pick
            
            git cherry-pick is a powerful command that enables arbitrary Git commits to be picked by reference and appended to the current working HEAD. Cherry picking is the act of picking a commit from a branch and applying it to another. git cherry-pick can be useful for undoing changes. For example, say a commit is accidently made to the wrong branch. You can switch to the correct branch and cherry-pick the commit to where it should belong.
            
            To demonstrate how to use `git cherry-pick` let us assume we have a repository with the following branch state:
            
            `a - b - c - d Master` `\` `e - f - g Feature`
            
            `git cherry-pick` usage is straight forward and can be executed like:
            
            `git cherry-pick commitSha`
            
            In this example `commit`Sha is a commit reference. You can find a commit reference by using `git log`. In this example we have constructed lets say we wanted to use commit `f` in `master`. First we ensure that we are working on the `master` branch.
            
            `git checkout master`
            
            Then we execute the cherry-pick with the following command:
            
            `git cherry-pick f`
            
            Once executed our Git history will look like:
            
            `a - b - c - d - f Master` `\` `e - f - g Feature`
            
            The f commit has been successfully picked into the feature branch
            
        - Change Commit Message
            
            ```Shell
            // For interactive mode
            git rebase master -i
            
            // This opens a window of vi (or nano) and then edit your message using 
            // r and squash other commit messages if you want.
            
            // If everything is fine
            git push -f
            
            // else
            git rebase --abort
            ```
            
        - Change commit author
            
            > [!info] Squash commits into one with Git  
            > Written by Triangles on November 17, 2017 * updated on May 31, 2020 * ID 60 - A nice way to group some changes together, especially before sharing them with others.  
            > [https://www.internalpointers.com/post/squash-commits-into-one-git](https://www.internalpointers.com/post/squash-commits-into-one-git)  
            
            `git commit --amend --reset-author`
            
            `git push --force-with-lease`  
            - Does not override other people's changes  
            
        - Remove changes for a file in MR
            
            Switch to the branch from which you created the pull request:
            
            ```Plain
            $ git checkout pull-request-branch
            ```
            
            Overwrite the modified file(s) with the file in another branch, let's consider it's **master**:
            
            ```Plain
            git checkout origin/master -- src/main/java/HelloWorld.java
            ```
            
            Commit and push it to the remote:
            
            ```Plain
            git commit -m "Removed a modified file from pull request"
            git push origin pull-request-branch
            ```
            
    - Regex
        
        Regex Cheat sheet
        
        ![[Untitled 16.png]]
        
    - Websites
        1. [https://justforfun.io/](https://justforfun.io/)
        2. [https://www.supremo.co.uk/typeterms/](https://www.supremo.co.uk/typeterms/)
        3. [https://flexboxfroggy.com/](https://flexboxfroggy.com/)
        4. [https://codepip.com/games/grid-garden/](https://codepip.com/games/grid-garden/)
        5. [https://css-transform.moro.es/](https://css-transform.moro.es/)
        6. [https://zzz.dog/modeling](https://zzz.dog/modeling)
        7. [https://colors.lol/](https://colors.lol/)
        8. [https://mixandgo.com/learn/how-to-use-ruby-each](https://mixandgo.com/learn/how-to-use-ruby-each)
    - Certificate challenges
        - HTTP-01 challenge
        - DNS-01 challenge
        - TLS-ALPN-01 challenge
    - DNS
    - Nginx
    - Cloudflare
    - Proxy servers
    - Kubernetes
        - Ingress
    - DNS
        - What is a DNS ?
            1. The Domain Name System (DNS) is the phonebook of the Internet. Humans access information online through domain names, like [nytimes.com](http://nytimes.com/) or [espn.com](http://espn.com/). Web browsers interact through Internet Protocol (IP) addresses. DNS translates domain names to IP addresses so browsers can load Internet resources.
            2. Each device connected to the Internet has a unique IP address which other machines use to find the device. DNS servers eliminate the need for humans to memorize IP addresses such as 192.168.1.1 (in IPv4), or more complex newer alphanumeric IP addresses such as 2400:cb00:2048:1::c629:d7a2 (in IPv6).
        - How does DNS work ?
            1. The process of DNS resolution involves converting a hostname (such as [www.example.com](http://www.example.com/)) into a computer-friendly IP address (such as 192.168.1.1). An IP address is given to each device on the Internet, and that address is necessary to find the appropriate Internet device - like a street address is used to find a particular home.
            2. When a user wants to load a webpage, a translation must occur between what a user types into their web browser ([example.com](http://example.com/)) and the machine-friendly address necessary to locate the [example.com](http://example.com/) webpage.
            3. In order to understand the process behind the DNS resolution, it’s important to learn about the different hardware components a DNS query must pass between. For the web browser, the DNS lookup occurs “ behind the scenes” and requires no interaction from the user’s computer apart from the initial request.
        - What are the 4 DNS servers involved in loading a webpage ?
            1. DNS recursor - The recursor can be thought of as a librarian who is asked to go find a particular book somewhere in a library. The DNS recursor is a server designed to receive queries from client machines through applications such as web browsers. Typically the recursor is then responsible for making additional requests in order to satisfy the client’s DNS query.
            2. Root server - The root server is the first step in translating (resolving) human readable host names into IP addresses. It can be thought of like an index in a library that points to different racks of books - typically it serves as a reference to other more specific locations.
            3. TLD nameserver - The top level domain server (TLD) can be thought of as a specific rack of books in a library. This nameserver is the next step in the search for a specific IP address, and it hosts the last portion of a hostname (In [example.com](http://example.com/), the TLD server is “com”).
            4. Authoritative nameserver - This final nameserver can be thought of as a dictionary on a rack of books, in which a specific name can be translated into its definition. The authoritative nameserver is the last stop in the nameserver query. If the authoritative name server has access to the requested record, it will return the IP address for the requested hostname back to the DNS Recursor (the librarian) that made the initial request.
        - What is the difference between an authoritative DNS server and a recursive DNS resolver?> [!important]  
            > the recursive resolver is at the beginning of the DNS query and the authoritative nameserver is at the end.  
            
              
            
        - What is the recursive DNS resolver ?
            
            The recursive resolver is the computer that responds to a recursive request from a client and takes the time to track down the DNS record. It does this by making a series of requests until it reaches the authoritative DNS nameserver for the requested record (or times out or returns an error if no record is found).  
              
            
            ![[Untitled 17.png]]
            
        - What is an authoritative DNS server ?
            
            1. server that actually holds, and is responsible for, DNS resource records. This is the server at the bottom of the DNS lookup chain that will respond with the queried resource record, ultimately allowing the web browser making the request to reach the IP address needed to access a website or other web resources.
            2. can satisfy queries from its own data without needing to query another source, as it is the final source of truth for certain DNS records.
            
            ![[Untitled 18.png]]
            
            > in instances where the query is for a subdomain such as [foo.example.com](http://foo.example.com/) or [blog.cloudflare.com](http://blog.cloudflare.com/), an additional nameserver will be added to the sequence after the authoritative nameserver, which is responsible for storing the subdomain’s CNAME record.
            
        - What are the steps in a DNS lookup?
            
            ### **The 8 steps in a DNS lookup:**
            
            1. A user types ‘example.com’ into a web browser and the query travels into the Internet and is received by a DNS recursive resolver.
            2. The resolver then queries a DNS root nameserver (.).
            3. The root server then responds to the resolver with the address of a Top Level Domain (TLD) DNS server (such as .com or .net), which stores the information for its domains. When searching for example.com, our request is pointed toward the .com TLD.
            4. The resolver then makes a request to the .com TLD.
            5. The TLD server then responds with the IP address of the domain’s nameserver, example.com.
            6. Lastly, the recursive resolver sends a query to the domain’s nameserver.
            7. The IP address for example.com is then returned to the resolver from the nameserver.
            8. The DNS resolver then responds to the web browser with the IP address of the domain requested initially.
            
            Once the 8 steps of the DNS lookup have returned the IP address for example.com, the browser is able to make the request for the web page:
            
            1. The browser makes a [**HTTP**](https://www.cloudflare.com/learning/ddos/glossary/hypertext-transfer-protocol-http/) request to the IP address.
            2. The server at that IP returns the webpage to be rendered in the browser (step 10).
            
              
            
            ![[Untitled 19.png]]
            
        - What is a DNS resolver ?
            
            ![[Untitled 20.png]]
            
        - What are the types of DNS queries ?
            
            ### **3 types of DNS queries:**
            
            1. **Recursive query** - In a recursive query, a DNS client requires that a DNS server (typically a DNS recursive resolver) will respond to the client with either the requested resource record or an error message if the resolver can't find the record.
            2. **Iterative query** - in this situation the DNS client will allow a DNS server to return the best answer it can. If the queried DNS server does not have a match for the query name, it will return a referral to a DNS server authoritative for a lower level of the domain namespace. The DNS client will then make a query to the referral address. This process continues with additional DNS servers down the query chain until either an error or timeout occurs.
            3. **Non-recursive query** - typically this will occur when a DNS resolver client queries a DNS server for a record that it has access to either because it's authoritative for the record or the record exists inside of its cache. Typically, a DNS server will cache DNS records to prevent additional bandwidth consumption and load on upstream servers.
        - What is DNS caching?
            
            The purpose of caching is to temporarily stored data in a location that results in improvements in performance and reliability for data requests. DNS caching involves storing data closer to the requesting client so that the DNS query can be resolved earlier and additional queries further down the DNS lookup chain can be avoided, thereby improving load times and reducing bandwidth/CPU consumption.
            
            DNS data can be cached in a variety of locations, each of which will store DNS records for a set amount of time determined by a [**time-to-live (TTL)**](https://www.cloudflare.com/learning/cdn/glossary/time-to-live-ttl/).
            
        - Where does DNS caching occur ?
            
            ### **Browser DNS caching**
            
            Modern web browsers are designed by default to cache DNS records for a set amount of time.  
            In chrome, you can see the status of your DNS cache by going to chrome://net-internals/\#dns.  
            
            ### **Operating system (OS) level DNS caching**
            
            The operating system level DNS resolver is the second and last local stop before a DNS query leaves your machine. The process inside your operating system that is designed to handle this query is commonly called a “stub resolver” or DNS client.
            
            hen a stub resolver gets a request from an application, it first checks its own cache to see if it has the record. If it does not, it then sends a DNS query (with a recursive flag set), outside the local network to a DNS recursive resolver inside the Internet service provider (ISP).
            
            ### **Recursive resolver DNS caching**
            
            When the recursive resolver inside the ISP receives a DNS query, like all previous steps, it will also check to see if the requested host-to-IP-address translation is already stored inside its local persistence layer.
            
            The recursive resolver also has additional functionality depending on the types of records it has in its cache:
            
            1. If the resolver does not have the [**A records**](https://www.cloudflare.com/learning/dns/dns-records/dns-a-record/), but does have the [**NS records**](https://www.cloudflare.com/learning/dns/dns-records/dns-ns-record/) for the authoritative nameservers, it will query those name servers directly, bypassing several steps in the DNS query. This shortcut prevents lookups from the root and .com nameservers (in our search for example.com) and helps the resolution of the DNS query occur more quickly.
            2. If the resolver does not have the NS records, it will send a query to the TLD servers (.com in our case), skipping the root server.
            3. In the unlikely event that the resolver does not have records pointing to the TLD servers, it will then query the root servers. This event typically occurs after a DNS cache has been purged.
    - Content delivery network
        - What is caching ?
            
            Caching is the process of storing copies of files in a cache, or temporary storage location, so that they can be accessed more quickly. Technically, a cache is any temporary storage location for copies of files or data, but usually the term is used in reference to Internet technologies. DNS servers cache DNS records for faster lookups, CDN servers cache content to reduce latency, and web browsers cache HTML files, JavaScript, and images in order to load websites more quickly.  
              
            
        - CDN Caching> [!important]  
            > Think of a CDN as being like a chain of grocery stores: Instead of going all the way to the farms where food is grown, which could be hundreds of miles away, shoppers go to their local grocery store, which still requires some travel but is much closer. Because grocery stores stock food from faraway farms, grocery shopping takes minutes instead of days. Similarly, CDN caches 'stock' the content that appears on the Internet so that webpages load much more quickly.  
            
            A CDN, or content delivery network, caches content (such as images, videos, or webpages) in proxy servers that are located closer to end-users than origin servers. (A proxy server is a server that receives requests from clients and passes them along to other servers.) Because the servers are closer to the user making the request, a CDN is able to deliver content more quickly.
            
            ![[Untitled 21.png]]
            
        - How long does cached data remain in a CDN server?
            
            When websites respond to CDN servers with the requested content, they attach information to the content that will let the servers know how long to store it.  
            This information is stored in a part of the response called the HTTP header, and it specifies for how many seconds, minutes, or hours content will be cached. This is known as the Time-To-Live (TTL). When the TTL expires, the cache removes the content.  
            
        - Web browser caching
            
            Web browser caching takes place when a browser saves a copy of files from a website on the user device's hard drive.  
            When a webpage is cached, the browser only needs to load new or updated pieces of a page, which enables browsers to deliver pages quickly even if an Internet connection is slow.  
            Browsers store these files until their TTL expires or until the hard drive cache is full. Users can also clear their browser cache if desired.  
            
        - DNS Caching
            
            DNS caching takes place on DNS servers. The servers store recent DNS lookups in their cache so that they don't have to query nameservers and can instantly reply with the IP address of a domain.
            
        - Anycast
            
              
            
        - Data center
        - Origin server
        - Edge server
        - Internet exchange server
        - Round trip time
        - Time to live
        - Cache control
        - GSLB
        - Cache hit ratio
        - Static vs dynamic caching
        - Reverse proxy servers
    
      
    
- HTML
    - <pre> Tag
        
        The `<pre>` tag defines preformatted text.
        
        Text in a `<pre>` element is displayed in a fixed-width font, and the text preserves both spaces and line breaks. The text will be displayed exactly as written in the HTML source code.
        
    - <code> Tag
        
        The <code> tag is used to define a piece of computer code. The content inside is displayed in the browser's default monospace font.
        
    - Inline vs Block
        
        ![[Untitled 22.png]]
        
        - at least in firefox, <input> & <button> are inline-block by default, not inline  
        - the width on a block element defaults to auto, not 100% (though it will often look like it's 100%)  
        - you can set the width on an inline element if it's a "replaced" element  
        
    - rel='preload' (Caching)
        
        The `preload` value of the [`<link>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link) element's [`rel`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link#attr-rel) attribute lets you declare fetch requests in the HTML's [`<head>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/head), specifying resources that your page will need very soon, which you want to start loading early in the page lifecycle, before browsers' main rendering machinery kicks in. This ensures they are available earlier and are less likely to block the page's render, improving performance.
        
        This article provides a basic guide to how `<link rel="preload">` works.
        
        > [!important]  
        > Used this in the carousel for demodesk inorder to load the images and cache them and not re fetch them on each carousel page switch.  
        
        For images,
        
        first have the standard img tag and then also include the link tag with rel="preload".  
        This way the images will be preloaded and cached.  
        
          
        
        Most commonly use `<link>` to load a CSS file to style your page with:
        
        ```HTML
        <link rel="stylesheet" href="styles/main.css">
        ```
        
        Here however, we will use a `rel` value of `preload`, which turns `<link>` into a preloader for any resource we want. You will also need to specify:
        
        - The path to the resource in the [`href`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link#attr-href) attribute.
        - The type of resource in the [`as`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link#attr-as) attribute.
        
          
        
        `preload` has other advantages too. Using `as` to specify the type of content to be preloaded allows the browser to:
        
        - Prioritize resource loading more accurately.
        - Store in the cache for future requests, reusing the resource if appropriate.
        - Apply the correct [content security policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) to the resource.
        - Set the correct [`Accept`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept) request headers for it.
        
          
        For more, read here:  
        [https://developer.mozilla.org/en-US/docs/Web/HTML/Preloading_content](https://developer.mozilla.org/en-US/docs/Web/HTML/Preloading_content)
        
    - Mobile input mode
        
        We can "hint" the browser what virtual keyboard we want to show on mobile devices without forcing the browser to change the visuals of the input. (unlike with input="number" for example)
        
        ![[Untitled 23.png]]
        
    - Random elements
        
        ![[Untitled 24.png]]
        
- Data Structure
    - Bloom filter
        
        Roughly speaking, a [Bloom filter](https://en.wikipedia.org/wiki/Bloom_filter) is a space-efficient way to check if an element is in a set.
        
        The trick is that it doesn't store the elements themselves; it just knows with some confidence that they were stored before. In our case, it can say with a certain _error rate_ that a word is in an article.
        
        ![[Untitled 25.png]]
        
        A Bloom filter stores a 'fingerprint' (a number of hash values) of all input values instead of the raw input. The result is a low-memory-footprint data structure. This is an example of 'hello' as an input.
        
          
        
        Python Code -
        
        ```Python
        filters = {}
        for name, words in split_posts.items(): 
        		filters[name] = BloomFilter(capacity=len(words), error_rate=0.1) 
        	for word in words: 
        			filters[name].add(word)
        ```
        
          
        
          
        
- Rust
    - println!
        
        The one character that might look out of place is an exclamation mark after println. If you have programmed in Ruby, you may be used to thinking that it is used to signal a destructive operation. In Rust, it signals the use of a macro. Macros can be thought of as sort of fancy functions for now. They offer the ability to avoid boilerplate code. In the case of println!, there is a bunch of type detection going on under the hood so that arbitrary data types can be printed to the screen.
        
    - Strings
        - UTF-8
            
            One of the first things that you are likely to notice is that strings in Rust are able to include a wide range of characters. Strings are UTF-8. This means that you are able to use non-English languages with relative ease.
            
        - Strings vs &str
            1. Strings are stored on the heap and are mutable (that is are modifiable)
            2. &str are immutable and are allocated on the stack (sometimes a heap reference or sometimes embedded in the code).
            3. Hardcoded strings (or string literals) are similar to &str
            4. Very easy to translate between the two
                
                ```Rust
                // &str ->> String
                let example_str: &str = "Hello";
                let example_string: String = String::from("Partner"); 
                
                let string_from_str: String = example_str.to_string();
                
                // String ->> &str
                // Does not simply copy the characters, it just points to
                // the original memory of the string
                let str_from_string: &str = &example_string;
                ```
                
            
        - Concatenate two strings
            1. We cannot just use a + to concatenate two strings (Throws a compile error)
            2. One of the ways to achieve this is to use an array and call concat (Result is a String and not a &str)
                
                ```Rust
                let combine_str: String = ["first",  "second"].concat();
                ```
                
                Another way is to use a format macro
                
                ```Rust
                let combine_with_format_macro = format!("{} {}", "first", "second");
                ```
                
            3. We can use + for concatenating a string and a string slice but this is very peculiar and relates to the concept of borrowing in rust.
        - Creating a new string
            
            Has to be declared as a mut to be mutable
            
            ```Rust
            let mut mut_string = String::new();
            mut_string.push_str(example_str);
            mut_string.push('m');
            ```
            
        - String from substring
            
            .. means upto but not including
            
            ```Rust
            let str_from_substring: &str = &example_str[0..2];
            ```
            
        - Get a particular char
            1. You can't just index into a string to get a char
            2. You need to do the following -  
                Since the nth char may or may not exist so it returns an option. (SAFETY)  
                
                ```Rust
                let char_by_index = &example_string.chars().nth(1);
                ```
                
            3. Handling overshooting of indices
                
                ```Rust
                match char_by_index {
                      Some(c) => println!("Found a char {}", c),
                      None => {}
                    }
                
                if let Some(c) = example_str.chars().nth(2) {
                      println!("Found a char {}", c);
                    }
                ```
                
        
    - Functions and procedures
        
        1. Functions return a value and procedures dont
        2. Function signature
            
            ```Rust
            // No semicolon line is understood as the return in rust
            
            fn some_function(param_a: f32, param_b: f32) -> f32 {
              param_a + param_b
            }
            ```
            
        3. Procedure signature
            
            ```Rust
            fn some_procedure(param_a: f32, param_b: f32) {
              println!("Hello world");
            }
            ```
            
              
            
        
        - Passing string and string slices to functions and procedures
            
            ```Rust
            fn some_str_procedure(param: &str) {
              println!("This is the param,  {}", param);
            }
            
            // Coercion
            let string_var = String::from("Hello");
            some_str_procedure(&string_var);
            ```
            
            Its usually preferable to use string slices as parameters in your functions and procedures.
            
              
            
    - Inline if statements
        
        Similar to the ternary operator usage in Javascript
        
        ```Rust
        // No semi-colon is used inside the branches (kind of similar to the return in a function)
        let var_from_inline = if some_bool { 300 } else { 400 };
        ```
        
    - Match conditional statement
        
        Kind of similar to a switch case
        
        ```Rust
        match some_bool {
              true => {
                println!("True");
              }
        
              false => {
                println!("false");
              }
            }
        
        
        let some_int = 100;
        
            match some_int {
              0 => println!("Hit 0 branch"),
              1..=100 => println!("Hit 1 and 100 branch"),
              _=> println!("Hit else branch"),
            }
        ```
        
    - Tuples
        
        Similar to tuples in python
        
        ```Rust
        let some_tuple = (2, 3.4);
            println!("My data is {}, {}", some_tuple.0, some_tuple.1);
            println!("Data is {:?}", some_tuple);
        ```
        
        Empty tuple (Sometimes seen at the end of match statements when you don't want to do anything for that branch)
        
        ```Rust
        let empty_tuple = ();
        ```
        
    - Structs, Traits and Impl
        1. Structs are similar to classes or objects in other languages with some notable differences
        2. Structs represent complex data types
        3. No inheritance
        4. Rust has traits - similar to polymorphism for object oriented programming
            
            ```Rust
            struct NavneetsData {
              some_int: i32,
              some_float: f64,
              some_bool: bool,
            }
            
            let navneet_var = NavneetsData {
                  some_bool: true,
                  some_int: 80,
                  some_float: 4.,
                };
            
            let mut navneet_var = NavneetsData {
                  some_bool: true,
                  some_int: 80,
                  some_float: 4.,
                };
                
                navneet_var.some_bool = false;
            ```
            
        5. Field declarations end with a comma (other languages use a semi-colon)
        6. We cannot just change values in a struct (it needs to be defined as mutable for that).
        7. Load data from another var (Similar to how it works in Javascript)
            
            ```Rust
            let navneet_var_2 = NavneetsData {
                  some_bool: true,
                  ..navneet_var
                };
            ```
            
        8. Importing other files in your main file - 
            
            ```Rust
            mod random_info;
            use random_info::*;
            ```
            
        9. Structs in declared in other files need to be declared as public to be visible in your main file. (Visibility of the fields also need to set explicitly to be visible)
            
            ```Rust
            pub struct RandomInfo {
            	pub some_bool: bool,
            }
            ```
            
        10. Associated functions (Functions that are associated with a struct type)
            
            ```Rust
            pub struct RandomInfo {
                pub some_bool: bool,
                pub some_float: f64,
            }
            
            impl RandomInfo {
                pub fn new(some_bool: bool, some_float: f64) -> Self {
                    RandomInfo { some_bool: some_bool, some_float: some_float }
                }
            }
            ```
            
        11. Self vs &self  
              
            **Self** - means the type  
              
            **&self** - means the actual data
        12. Use of &self
            
            ```Rust
            impl RandomInfo {
                pub fn new(some_bool: bool, some_float: f64) -> Self {
                    RandomInfo { some_bool: some_bool, some_float: some_float, some_int: 8 }
                }
            
                pub fn is_smaller(&self, compare_to: i32) -> bool {
                    self.some_int < compare_to
                }
            }
            ```
            
        13. Mutable implementations
            
            ```Rust
            pub struct RandomInfo {
                pub call_count: i64,
                pub some_bool: bool,
                pub some_float: f64,
                pub some_int: i32,
            }
            
            impl RandomInfo {
                pub fn new(some_bool: bool, some_float: f64) -> Self {
                    RandomInfo { 
                        call_count: 0,
                        some_bool: some_bool, 
                        some_float: some_float, 
                        some_int: 8 }
                }
            
                pub fn is_smaller(&mut self, compare_to: i32) -> bool {
                    self.call_count += 1;
                    self.some_int < compare_to
                }
            }
            ```
            
        14. Polymorphism with traits  
              
              
            **What is polymorphism ? -**  
            Polymorphism is the ability of an object to take on many forms. The most common use of polymorphism in OOP occurs when a parent class reference is used to refer to a child class object. Any Java object that can pass more than one IS-A test is considered to be polymorphic.  
            
            ```Rust
            pub trait SomeTrait {
                fn is_valid(&self) -> bool;
            }
            
            pub struct RandomInfo {
                pub call_count: i64,
                pub some_bool: bool,
                pub some_float: f64,
                pub some_int: i32,
            }
            
            impl SomeTrait for RandomInfo {
                fn is_valid(&self) -> bool {
                    self.some_bool
                }
            }
            
            // Usage
            fn print_if_is_valid(check_me: &dyn SomeTrait) {
              if check_me.is_valid() {
                println!("Valid!");
              }
            }
            ```
            
    - enums
        
        ```Rust
        enum Payment {
          Cash,
          CreditCard,
        }
        
        let some_payment = Payment::Cash;
        
          match some_payment {
            Payment::Cash => {
              println!("paying with cash", );
            }
        
            Payment::CreditCard => {
              println!("paying with card")
            }
          }
        
        
        enum Payment {
          Cash(f32),
          CreditCard,
        }
        
        fn main() {
        
          let some_payment = Payment::Cash(100.);
        
          match some_payment {
            Payment::Cash(amt) => {
              println!("paying with cash", );
            }
        
            Payment::CreditCard => {
              println!("paying with card")
            }
          }
        ```
        
    - Generics
        1. Used for situations where a generic type is required
            
            ```Rust
            // The T here just acts as a placeholder until the compiler
            // sees how you want to define the Point struct
            
            struct Point<T> {
              x: T,
              y: T,
            }
            
            let a = Point { x: 100, y: -1 };
            
            let b = Point { x:10.1, y: 10.2 };
            ```
            
        2. Generics have no runtime cost. All done at compile time
        3. Enumerations can also have generics
            
            ```Rust
            enum SomeEnum<T> {
              OptionA(T),
              OptionB(T),
              OptionC,
            }
            
            let some_enum_data = SomeEnum::OptionA(32.1);
            match some_enum_data {
                SomeEnum::OptionA(a) => println!("OptionA contains {}", a),
                SomeEnum::OptionB(b) => println!("OptionA contains {}", b),
                SomeEnum::OptionC => println!("OptionA contains c"),
              }
            ```
            
        4. Vectors are also generic
        5. Functions can also be generic
            
            ```Rust
            fn random_generic_func<T>(input_a: T, input_b: T) -> T {
            	input_a
            }
            
            // If you want to return an operation over inputs
            fn random_generic_func<T: std::ops::Add<Output=T>>(input_a: T, input_b: T) -> T {
              input_a + input_b
            }
            
            // Another way to write this - 
            fn random_generic_func<T>(input_a: T, input_b: T) -> T 
            where T: std::ops::Add<Output=T> {
              input_a + input_b
            }
            ```
            
        6. Using traits with generics
            
            ```Rust
            trait SomeCustomTrait {
              fn blah_blah(&self, a: &str, b: &str) -> String;
            }
            
            fn do_this<T>(some_var: &T) -> String 
            where T: SomeCustomTrait + std::fmt::Debug {
              println!("{:?}", some_var);
              some_var.blah_blah("first", "second");
            }
            ```
            
        7. Using structs with generics
            
            ```Rust
            struct DougStruct<T, U> {
            	dougs_t: T,
            	dougs_u: U,
            }
            
            impl<T, U> DougsStruct<T, U> 
            where T: std::fmt::Debug,
            			U: std::fmt::Debug {
            	fn log_something(&self) {
            		println!("{:?}, {:?}", self.dougs_t, self.dougs_u);
            	}
            }
            ```
            
    - **Ownership**
        
        There is no memory management in Rust.
        
        - What is ownership ?
            1. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. **Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time**. None of the ownership features slow down your program while it’s running.
            
        - The stack and the heap
            1. Both the stack and the heap are parts of memory that are available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as _last in, first out_. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called _pushing onto the stack_, and removing data is called _popping off the stack_.
            2. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead. The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a _pointer_, which is the address of that location. This process is called _allocating on the heap_ and is sometimes abbreviated as just _allocating_. Pushing values onto the stack is not considered allocating. Because the pointer is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
            3. **Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.**
            4. Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
            5. **Accessing data in the heap is slower** than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant taking orders from many tables. It’s most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap). Allocating a large amount of space on the heap can also take time.
            6. When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
            7. Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. (In the context of rust).
        - Ownership rules
            1. **Each value in Rust has a variable that’s called its owner.**
            2. **There can only be one owner at a time.**
            3. **When the owner goes out of scope, the value will be dropped.**
        - Most common Rust compile error
            
            ```Rust
            let var_a = String::from("Howdy");
            let var_b = var_a;
            println!("{}", var_a);
            ```
            
            This throws the following error
            
            ![[Screenshot_2021-01-20_at_23.12.32.png]]
            
        - Stack variables
            
            ```Rust
            // Stack is used by default by rust for its memory needs
            let stack_i8: i8 = 10;
            
            // For all these vars, Rust knows the space required
            let stack_i8: i8 = 10;
            let stack_bool: bool = true;
            ```
            
            1. Stack variables are fixed in size
            2. Collections and vectors cannot be stack variables because they can change in size. (**Exception is a fixed size array since the compiler can know the exact size and memory needed for such an array**)
            3. Strings are collections of u8 that can grow so it cannot be on the stack
            4. Rust cleans all memory associated with a scope once the scope exits.
            5. Anything with a =={}== creates a new scope. (This is very similar to other languages handle scoped variables)
                
                ```Rust
                if stack_i8 == 3 {
                    let inside_scope = 9;
                    print!("{}", inside_scope);
                  }
                
                // Compile error if inside_scope is used now
                ```
                
        - **Heap variables**
            1. Heap variables are flexible and can grow in size (For eg- vectors, collections and strings)
            2. Has runtime performance cost
            3. Memory can live beyond the scope that created it
            4. Memory is automatically recaptured when the **==LAST OWNER==** goes out of scope.
                
                ```Rust
                let heap_vector :Vec<i8>= Vec::new();
                let heap_string :String = String::from("Howdy");
                let heap_i8: Box<i8> = Box::new(30);
                
                
                // The following piece of code works without any compile error
                // as expected.
                
                let stack_i8_2 = stack_i8;
                  println!("{}", stack_i8);
                  println!("{}", stack_i8_2);
                
                // However, when doing the same with the aforementioned heap i8
                // the compiler throws a borrow error.
                
                let heap_i8_2 = heap_i8;
                  print!("{}", heap_i8);
                  print!("{}", heap_i8_2);
                ```
                
                ![[Screenshot_2021-01-24_at_10.42.02.png]]
                
                  
                
            5. In Rust, every piece of data in memory has an owner and there can only be a single owner at a time.
            6. When we create ==heap_i8==, enough memory was allocated on the heap to represent the data and then the ownership of that memory is assigned to ==heap_i8==. (==heap_i8== is the one and only one official owner of that memory).
            7. When we created ==heap_i8_2== from ==heap_i8==, what we were really doing was keeping the original memory but transferring the ownership to ==heap_i8_2==.
            8. The variable ==heap_i8== is no longer pointing to any memory.
            9. This is in contrast to most other programming languages. In most other languages, ==heap_i8_2== and ==heap_i8== would point to the same location in memory.
            10. This does not happen for stack data since it is extremely cheap to create a copy for stack data. ==stack_i8== and ==stack_i8_2== "own" different memory.
            11. **How do we keep** ==**heap_i8**== **intact after the assigning of** ==**heap_i8_2**== **?**a. One way is to clone the heap_i8 memory
                
                ```Rust
                let heap_i8_2 = heap_i8.clone();
                  print!("{}", heap_i8);
                  print!("{}", heap_i8_2);
                ```
                
                This creates a completely new variable. So changing the value of one does not affect the other.  
                  
                _==Cloning is relatively expensive and not ideal for performance critical applications==__._  
                  
                
                b. Another way is to "borrow" the ownership
                
                ```Rust
                let heap_i8_2 = &heap_i8;
                  print!("{}", heap_i8);
                  print!("{}", heap_i8_2);
                ```
                
                  
                
            12. Usage in procedures and functions -
                
                When stack_f64 gets passed to the procedure, it will create a copy and place it on the stack.
                
                ```Rust
                let stack_f64: f64 = 1.;
                let heap_f64: Box<f64> = Box::new(2.);
                
                // If we call stack_procedure, we can still use the variable
                // afterwards.
                stack_procedure(stack_f64);
                println!("In main stack,  {}", stack_f64);
                
                
                fn stack_procedure(param_a: f64) {
                  println!("In stack_procedure: {}", param_a);
                }
                ```
                
                ==_Rust assumes immutability by default.  
                  
                _==
                
            13. Heap procedures
                
                Throws an ownership and borrow error.
                
                Ownership of the ==heap_f64== gets transfered to ==param== when it is passed to ==heap_procedure==.  
                  
                As soon as the scope of the procedure ends, the memory is cleaned up.  
                
                ![[Screenshot_2021-01-24_at_11.19.03.png]]
                
                ```Rust
                fn heap_procedure(param: Box<f64>) {
                  println!("In heap_procedure: {}", param);
                }
                ```
                
                  
                
            14. **Solution for the heap procedure problem** -  
                Instead of giving away the ownership of the memory, we can instead borrow the ownership and then return it once the procedure ends.  
                > [!important]  
                > Remember, allocated memory needs to one and only one owner.  
                
                ```Rust
                heap_procedure(&heap_f64);
                println!("In main heap, {}", heap_f64);
                
                fn heap_procedure(param: &Box<f64>) {
                  println!("In heap_procedure: {}", param);
                }
                ```
                
                & tells us that the memory's
                
            
    - Cargo vs rustc
        
        In a strict sense, `rustc` is the Rust compiler. The preceding examples don’t use it though. Instead, they interact with `cargo`. `cargo` provides a simple interface, invoking `rustc` on our behalf.
        
        Add the `--verbose` flag to cargo to see what `cargo` passes to `rustc`:
        
        ```Shell
        $ cd code/ch1/ch1-penguins
        $ cargo run --verbose
           Compiling ch1-penguins v0.1.0 (rust-in-action/code/ch1/ch1-penguins)
             Running `rustc --crate-name ch1_penguins --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -Cbitcode-in-rlib=no -C debuginfo=2 -C metadata=390c3b75f851c687 -C extra-filename=-390c3b75f851c687 --out-dir rust-in-action/code/ch1/ch1-penguins/target/debug/deps -C incremental=rust-in-action/code/ch1/ch1-penguins/target/debug/incremental -L dependency=rust-in-action/code/ch1/ch1-penguins/target/debug/deps`
            Finished dev [unoptimized + debuginfo] target(s) in 0.43s
             Running `target/debug/ch1-penguins`
        debug: "  Little penguin,33" -> ["Little penguin", "33"]
        Little penguin, 33cm
        debug: "  Yellow-eyed penguin,65" -> ["Yellow-eyed penguin", "65"]
        Yellow-eyed penguin, 65cm
        debug: "  Fiordland penguin,60" -> ["Fiordland penguin", "60"]
        Fiordland penguin, 60cm
        debug: "  Invalid,data" -> ["Invalid", "data"]
        ```
        
          
        
    - Why rust
        
        ![[Untitled 26.png]]
        
    - CSV Parsing in rust
        
        ```Rust
        fn main() {
        let penguin_data ="\
        
        common name,length (cm)
        
        Little penguin,33
        
        Yellow-eyed penguin,65
        
        Fiordland penguin,60
        
        Invalid,data
        
        ";
        
        let records = penguin_data.lines();
        
        for (i, record) in records.enumerate() {
        
        if i == 0 || record.trim().len() == 0 {
        
        continue;
        
        }
        
        let fields: Vec<_> = record
        
        .split(',')
        
        .map(|field| field.trim())
        
        .collect();
        
        if cfg!(debug_assertions) {
        
        eprintln!("debug: {:?} -> {:?}", record, fields);
        
        }
        
        let name = fields[0];
        
        let maybe_length: Result<f32, _> = fields[1].parse();
        
        if maybe_length.is_err() {
        
        continue;
        
        }
        
        let length = maybe_length.unwrap();
        
        println!("{}, {}cm", name, length);
        
        }
        
        }
        ```
        
    - Rust vs C
        
        Rust programs are free from:
        
        - “dangling pointers” - live references to data that has become invalid over the course of the program (see [Listing 1. 6](https://livebook.manning.com/book/rust-in-action/chapter-1/v-13/dangling))
        - “data races” - unable to determine how a program will behave from run to run because external factors are changing (see [Listing 1. 7](https://livebook.manning.com/book/rust-in-action/chapter-1/v-13/race))
        - “buffer overflow” - attempting to access the 12th element of an array of only 6 elements (see [Listing 1. 8](https://livebook.manning.com/book/rust-in-action/chapter-1/v-13/buff-overflow))
        - “iterator invalidation” - an issue caused by something that is being iterated over being altered mid-way through
        
        When programs are compiled in debug mode, Rust also protects against integer overflow. What is integer overflow? Well, integers can only represent a finite number of numbers, they have a fixed-width in memory, Integer overflow is what happens when they hit their limit and flow over to the beginning again.
        
    - Return type for an assignment
        
        It’s receiving the result of an assignment. In Rust, this is the blank type: (). () is pronounced “unit”[16]. () is returned by expressions when there is no other meaningful return value.
        
    - Rust Features
        - Data within Rust is immutable by default
        - The language’s first priority is safety
        - Compile-time checks are strongly preferred, safety should be a “zero-cost abstraction”
        - Cache-friendly data structures are provided by default. Arrays usually hold data within Rust programs, rather than deeply nested tree structures that are created by pointers. This is referred to as data-oriented programming.
        - Having a modern package manager available (cargo) makes it very easy to benefit from the world’s smartest programmers. C and C++ have much less consistency here, and building large projects with many dependencies is typically very difficult.
        - Methods are always dispatched statically, unless dynamic dispatch is explicitly requested. This enables the compiler to heavily optimize code, sometimes to the point of eliminating the cost of the function call entirely.
        - Rust includes a large number of numeric types. You will become used to declaring the size in bytes, which affects how many numbers the type can represent, and whether your type is able to represent negative numbers.
        - Conversions between types are always explicit. Rust will not automatically convert your 16-bit integer into a 32-bit integer.
        - Rust’s numbers can have methods. For example, to round 24.5 to the nearest integer, Rust programmers use `24.5_f32.round()` rather than (`round(24.5_f32)`). The type suffix is required because a concrete type is required.
    - Immutability
        
        unchanging over time or unable to be changed.
        
    - Heartbleed
        
        Heartbleed was caused by re-using a _buffer_ incorrectly. A buffer is a space for set aside in memory for receiving input. Data can leak from one read to the next if the buffer’s contents are not cleared between writes.
        
        Why does this situation occur? Programmers hunt for performance. Buffers are re-used to minimize how often memory applications ask for memory from the operating system.
        
        Imagine that we wished to process some secret information from multiple users. We decide, for whatever reason, it re-use a single buffer through the course of the program. If we don’t reset this buffer once we have made use of it, information from earlier calls leaks to the latter ones.
        
        ```Rust
        let buffer = &mut[0u8; 1024];
        read_secrets(&user1, buffer);
        store_secrets(buffer);
        read_secrets(&user2, buffer);
        store_secrets(buffer);
        ```
        
          
        
    - Zero sided types
        
        many types are zero-sized. That is, they only exist as hints to the compiler and take up no memory at all in the running program.
        
    - Rust file components
        
        To compile a single file of Rust into a working program:
        
        - Make sure that file includes a `main()` function
        - Open a shell such as `cmd.exe`, Powershell, `bash` or Terminal
        - Execute the command `rustc <file>`, where `<file>` is the file to be compiled
    - Compiling larger projects
        
        Projects larger than a single file tend to be compiled with a higher-level tool called `cargo`. `cargo` understands how to compile multi-file projects (and much more). It executes `rustc` on your behalf.
        
        If you’re ever curious about what `cargo` is doing under the hood, add the verbose flag (`-v`). This will instruct `cargo` to report the commands that it is executing on your behalf.
        
    - Numeric Types
        
        Rust contains a full complement of numeric types. The types are grouped into a few families:
        
        - signed integers (`i`) can represent negative as well as positive integers
        - unsigned integer (`u`) can only represent positive integers but can count twice as high than their signed counterparts
        - floating point (`f`) is able to represent real numbers and has special values for infinity, negative infinity and "not a number"
        
        The widths are the number of bits that the type uses in RAM and in the CPU. Types that take up more space, such as `u32` vs `i8`, can represent more numbers at the expense of needing to store extra zeros for smaller numbers.
        
    - Impossible to compare different types
        
        Rust’s type safety requirements prevent comparisons between types. This example will not compile:
        
        The following piece of code results in an error ⇒
        
        ```Rust
        fn main() {
        let a: i32 = 10;
        let b: u16 = 100;
        if a < b {
        println!("Ten is less than one hundred.");
        }
        }
        ```
        
        To appease the compiler, use an as to cast one of the operands to the other’s type, e.g. b as i32:
        
        ```Rust
        fn main() {
        let a: i32 = 10;
        let b: u16 = 100;
        if a < (b as i32) {
        println!("Ten is less than one hundred.");
        }
        }
        ```
        
        **Warning** ⚠️ ⇒ Using type casts carelessly will cause your program to behave unexpectedly. The expression 300_i32 as i8 returns 44.
        
    - Floating point hazards
        
        Floating point types (`f32` and `f64`) are a special case of comparisons for (at least) two reasons:
        
        1. They often approximate the numbers that they’re representing. They’re implemented in base 2, but we often want to calculate numbers in base 10. This mismatch creates ambiguity.
        2. They can represent values that have unintuitive semantics. Unlike integers, floating point types have some values that do not play well with others. Formally, they only have a _partial equivalence relation_.
        
        Numbers that are impossible to represent in binary
        
        Floating point numbers are implemented inside computing systems that use binary mathematics, but often are used by people to represent values in decimal.
        
        Rust includes some tolerances to allow comparisons between floating-point values. These tolerances are defined as `f32::EPSILON` and `f64::EPSILON`.
        
        - NaN
            
            Floating point types include “Not a Number” (typically represented as NaN, but within Rust as NAN) values. These values represent the result of numerical operations that are undefined, such as taking the square root of a negative number. By definition, one NaN value is always unequal to other NaN values.
            
    - Syntax for declaring variable
        
        One is a literal syntax provided by the Rust language (line 4). The other is the new() static method that many types implement for added convenience (line 5). A static method is a function that’s available from a type, not an instance of that type
        
        ```Rust
        use num::complex::Complex;
        
        fn main() {
        let a = Complex { re: 2.1, im: -1.2 };
        let b = Complex::new(11.1, 22.2);
        let result = a + b
        println!("{} + {}i", result.re, result.im)
        }
        ```
        
    - Ampersand prefix
        
        The ampersand (&) prefix to haystack on line 5, e.g. &haystack, may look unfamiliar. The ampersand is acting as a unary operator that returns a reference to haystack. For any type T, the syntax &T returns a read-only reference to T. A useful quirk of references to arrays is that they enable you to iterate over references to the elements of the array with for loops. In Rust terminology, when we use &T we borrow T. A “borrowed” value outlives the scope that it is being used in.
        
          
        
        ```Rust
        fn main() {
        let needle = 0o52;
        let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
        for item in &haystack {
        if *item == needle {
        println!("{}", item);
        }
        }
        }
        ```
        
    - Iterators
        
        Concretely, &haystack enabled us to iterate over items within haystack. Unfortunately, not all types support this syntax.
        
        Calling haystack.iter() returns an iterator.
        
        ```Rust
        fn main() {
        let needle = 0xCB;
        let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];
        for item in haystack.iter() {
        if *item == needle {
        println!("{}", item);
        }
        }
        }
        ```
        
        Some types also provide iter_mut() and into_iter() methods. They are more complicated and most readers can probably be skip worrying about them for now. The iter_mut() method allows you to mutate (to change) values as you are iterating over them. The into_iter() method is closer to iter(), but rather than using references, it returns elements by value. As a consequence, this prevents other parts of the code from accessing those values.
        
    - Anonymous Loops
        
        When the local variable is not being used within the block, it’s conventional to use an underscore (_). Using this pattern in conjunction with the _exclusive range syntax_ (n..m) and the inclusive range syntax (n..=m) makes it clear that the intent is to perform a loop for a fixed number of times.
        
        ```Rust
        for _ in 0..10 {
        // ...
        }
        ```
        
    - Avoid managing an index variable
        
        It’s common many languages to loop through things by using a temporary variable that’s incremented at the end of each iteration. Conventionally, this variable is named `i` (for index).
        
        A Rust version of that pattern would be this snippet:
        
        ```Rust
        let collection = [1, 2, 3, 4, 5];
        for i in 0..collection.len() {
        let item = collection[i];
        // ...
        }
        ```
        
        This is legal Rust. It’s also essential in cases where iterating directly over `collection` (`for item in collection`) is impossible. However, it is generally discouraged. The manual approach introduces two problems:
        
        1. **Performance:** Indexing values with the `collection[index]` syntax incurs runtime costs for _bounds checking_. That is, Rust will check that `index` currently exists within `collection` as valid data. Those checks are not necessary when iterating directly over `collection`. The compiler can use compile-time analysis to prove that illegal access is impossible.
        2. **Safety:** Periodically accessing `collection` over time introduces the possibility that it has changed. Using a `for` loop over `collection` directly allows Rust to guarantee that the `collection` remains untouched by other parts of the program.
    - Infinite Loops
        
        **Avoid** `**while**` **to loop endlessly**
        
        Most Rust programmers avoid the following idiom to express looping “forever”
        
        ```Rust
        while true {
        println!("Are we there yet?");
        }
        ```
        
        Rust contains a loop keyword for providing more control than for and while. loop will execute a code block again and again, never stopping for a tea break. loop continues to execute until a break keyword is encountered or the program is terminated from the outside.
        
        ```Rust
        loop {
        let requester, request = accept_request();
        let result = process_request(request);
        send_response(requester, result);
        }
        ```
        
    - break
        
        The break keyword breaks out of a loop. Rust’s generally operates as you are used to.
        
        ```Rust
        for (x, y) in (0..).zip(0..) {
        if x + y > 100 {
        break;
        }
        // ...
        }
        ```
        
    - Loop labels
        
        A loop label is an identifier prefixed with an apostrophe (').
        
        ```Rust
        'outer: for x in 0.. {
        for y in 0.. {
        for z in 0.. {
        if x + y + z > 1000 {
        break 'outer;
        }
          // ...
        }
        }
        }
        ```
        
          
        
    - match
        
        While it’s possible to use if/else blocks in Rust, there is a safer alternative: match. match is safer because it will warn you if you haven’t considered a relevant alternative. It is also elegant and concise:
        
        Rust’s match keyword is analogous to other languages' switch keyword. Unlike C’s switch however, match guarantees that all possible options for a type are explicitly handled. Additionally, a match does not “fall through” to the next option by default. Instead, match returns immediately when a match is found.
        
          
        
        ```Rust
        fn main() {
        let needle = 42;
        let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862]
        for item in &haystack {
        let result = match item {
        42 | 132 => "hit!",
        _ => "miss",
        };
        if result == "hit!" {
          println!("{}: {}", item, result);
        }
        }
        }
        ```
        
    - Function signature
        
        ![[Untitled 27.png]]
        
    
- CSS
    - Z-index
        
        ![[Untitled 28.png]]
        
    - Flexbox
    - Grid
    - Pseudo Elements
        
        The `::before` and `::after` pseudo-elements in CSS allows you to insert content onto a page without it needing to be in the HTML. While the end result is not actually in the DOM, it appears on the page as if it is, and would essentially be like this:
        
        ```CSS
        div::before {
          content: "before";
        }
        div::after {
          content: "after";
        }
        ```
        
        ```CSS
        <div>
          before
          <!-- Rest of stuff inside the div -->
          after
        </div>
        ```
        
        The only reasons to use one over the other are:
        
        - You want the generated content to come before the element content, positionally.
        - The `::after` content is also “after” in source-order, so it will position on top of ::before if stacked on top of each other naturally.
        
        Note that the content is still _inside_ the element they are applied to. The naming sort of feels like they might come, ya know, before or after the element, but it’s really before or after _the other content inside_.
        
        The value for content can be:
        
        - **A string:** `content: "a string";` – special characters need to be specially encoded as a unicode entity. See the [glyphs page](https://css-tricks.com/snippets/html/glyphs/).
        - **An image:** content: url(/path/to/image.jpg); – The image is inserted at it’s exact dimensions and [cannot be resized.](https://codepen.io/team/css-tricks/pen/gBaaGo) Since things like [gradients](https://css-tricks.com/css3-gradients/) are actually images, a pseudo element can be a gradient.
        - **Nothing:** content: “”; – Useful for clearfix and inserting images as background-images (set width and height, and can even resize with background-size).
        - **A counter:** `content: counter(li);` – Really useful for [styling lists](http://www.456bereastreet.com/archive/201105/styling_ordered_list_numbers/) until :marker comes along.
        
        Note that you cannot insert HTML (at least, that will be rendered as HTML). `content: "<h1>nope</h1>";`
        
    - Flex Direction
        
        the [`flex-direction`](https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction) property can take one of four values:
        
        - `row`
        - `column`
        - `row-reverse`
        - `column-reverse`
        
        The first two values ​​keep the items in the same order that they appear in the document source order and display them sequentially from the start line.
        
        [![](https://mdn.mozillademos.org/files/15649/Basics1.png)](https://mdn.mozillademos.org/files/15649/Basics1.png)
        
        [![](https://mdn.mozillademos.org/files/15650/align10.png)](https://mdn.mozillademos.org/files/15650/align10.png)
        
        The second two values ​​reverse the items by switching the start and end lines.
        
        [![](https://mdn.mozillademos.org/files/15651/align9.png)](https://mdn.mozillademos.org/files/15651/align9.png)
        
        [![](https://mdn.mozillademos.org/files/15652/align11.png)](https://mdn.mozillademos.org/files/15652/align11.png)
        
        Remember that the start line relates to writing modes. The row-related examples above demonstrate how `row` and `row-reverse` work in a left-to-right language such as English. If you are working in a right-to-left language like Arabic then `row` would start on the right, `row-reverse` on the left.
        
        [![](https://mdn.mozillademos.org/files/15647/order-rtl.png)](https://mdn.mozillademos.org/files/15647/order-rtl.png)
        
    - Position absolute vs Position Relative
    - CSS Box Model
        
        ![[Untitled 29.png]]
        
    - Random Links
        1. [https://css-tricks.com/centering-css-complete-guide/](https://css-tricks.com/centering-css-complete-guide/)
    - outline-offset
        
        The `outline-offset` property accepts negative values, which can be useful for when you want to show an outline & avoid it cutting off or overlapping adjacent elements, such as focusable table (or grid) cells and rows.  
          
        
        ![[Untitled 30.png]]
        
          
        
        ![[Untitled 31.png]]
        
        Read more at -
        
        [https://tympanus.net/codrops/css_reference/outline-offset/](https://tympanus.net/codrops/css_reference/outline-offset/)
        
    - 1-line layouts
        
        [https://1linelayouts.glitch.me/](https://1linelayouts.glitch.me/)
        
          
        
    - Specificity and !important
        
        [https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity](https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity)
        
- Vue Unit Testing
    - Types of unit testing
        1. Business logic testing : Application logic
        2. Public interface testing : Props
        3. Side effects : "if this, then that"
    - Libraries
        - Vue Test Utils
            
            Official unit testing library for Vue
            
        - Jest
            
            Test Runner
            
    - Overview
        1. Naming Convention <Component Name>.spec.js
        2. Typically have one test per file
    - Running Test
        
        `npm run test`
        
    - Mounting
        1. shallowMount creates a wrapper for the Vue component but with stubbed child components.
        2. mount creates a wrapper for the Vue component including mounting any child components.
    - Describe Block
        
        defines the unit test suite. Witihin a unit test file, there can be multiple describe blocks that define different unit test suites.
        
        Each describe block can contain multiple unit tests.
        
        1. describe block - unit test suite
        2. it block - individual unit test function
    - Steps
        1. Mount the vue component for ex using a `shallowMount`.
        2. Actual testing happens using expect.
        3. check name of the component
        4. check that title is rendered with for example using `toMatch`.
    - Jest Helpers
        
        - Booleans
            1. toBeTruthy() - checks that variable/statement is true
            2. toBeFalsy() - checks that a variable/statement is false
        - Documentation
            
            [https://jestjs.io/](https://jestjs.io/)
            
        - beforeEach
            
            called before execution of each unit test.
            
            to normalize the code to bring to a standard state.
            
        - afterEach
            
            called after execution of each unit test
            
            remove unwanted items that may fail the upcoming tests.
            
        
          
        
- Unix
    - $
        1. $$ means the process id
        2. $? Refers to the return statement of the previous command
        3. $_ refers to the command history
        
    - Set the default as vi
    - Creating pipes
        
        1. mkfifo mycommspipe
        2. ls -l mycommspipe
        3. open another terminal
        4. In the first terminal -  
            cat /etc/passwd > mycommspipe  
            
        5. In the second terminal run,  
            cat < mycommspipe  
            
        
        You are communicating through a pipe!
        
- Git
    - Basic Terminology
        1. Git refers to the folders (or top level directories) as trees
        2. And the files are referred to as blobs.
        3. These represent a recursive data structure.
        4. The top level directory is the directory being tracked.
    - How is the tracking done?
        1. One way is to take the snapshot of the entire directory and then history is basically a linear timeline of snapshots.
        2. Git on the other hand uses a directed acyclic graph to represent the history.
        3. In git, each snapshot has some number of parents.
        4. This enables features like branching and forking
        5. Each snapshot also contains some metadata such as author name etc.
    - Data Structure representation of git
        
        ```Python
        type blob = array<byte>
        type tree = map<string, tree | blob>
        
        type commit = struct {
        	parents array<commits> # Just stores the pointers or ids (not the actual commits)
        	author: string,
        	mssg: string,
        	snapshot: tree,
        }
        ```
        
    - How does git store and addresses the data ?
        
        Gits on data store is a content addressed store where contents are addressed by their hash.
        
        ```Python
        type object = blob | tree | commit
        objects = map <string, object>
        
        
        def store(o):
        	key = sha1(o)
        	objects[key] = o
        
        def load(id):
        	return objects[id]
        ```
        
    - Git referencing system
        1. The ids for each of the git objects (basically the commits tree or blobs) are very long and inconvenient.
        2. So git maintains and referencing system.
        3. This basically a way to map human readable names to the sha1 keys of the objects.
            
            ```Python
            references = map<string, string>
            ```
            
        4. The entire acyclic graph is immutable however references are mutable.
        5. On a high level, git commands are just a manipulation of either the references data or the objects data.
    - Git commands
        
        1. git init initialize a git repository. This creates a hidden .git folder inside a repository which in itself contains the objects and references that is used by git for creating and maintaining the directed acyclic graph.
            
            ```Shell
            // Initialize a git repository
            git init
            ```
            
        2. git status
            
            Shows the current status  
              
            Git staging area  
            We can tell git what changes to include in the next snapshot you take.  
            
            ```Shell
            git status 
            
            // Add
            git add hello.txt
            
            // Take a snapshot 
            git commit -m "Add hello.txt"
            // This prints out the hash of the commit
            
            
            // Helps visualize the history (git commit log)
            git log
            
            // A better way to visualize the history (in the form of a graph)
            git log --all --graph --decorate
            
            // To view the internal contents of a hash commit 
            // (or the hash of tree or the blob)
            git cat-file -p HASH_OF_THE_COMMIT
            
            
            // Checkout helps to move around in your commit history
            git checkout HASH_OF_THE_COMMIT
            ```
            
        
          
        
        3. HEAD basically refers to where you are currently looking at.
        
        4. After git add, the files are added to the index (also known as the staging area), which lists the files that git is monitoring in the working directory. It does the same job as a directory in linux and holds details of which files and their versions are being worked on.  
        You can see what is in the index using :  
          
        
        ```Shell
        git ls-files --stage
        ```
        
          
        
        5. The compressed files are stored under objects with the first two characters of the hash used to create a directory.
        
        6. This is done to speed up storing and retrieving objects.
        
        7. Setting alias for long git commands is a useful tool  
        Like Linux aliases, git aliases are stored in a config file on your machine.  
        
        ```Shell
        git config --global alias.logdog 'log --decorate --oneline --graph'
        ```
        
        8. .gitignore  
        Add files to the .gitignore if you don't want some files to not be tracked by git. If git is tracking a file, then adding to .gitignore will have no effect. If you want to untrack a file, use a git rm —cached which removes the git object but leaves the file in the working directory.  
        
    - Undoing changes and commits
        1. You can use git reset to tell git to replace the information in the index (staging area) with the version associated with the current head (or latest) commit.
        2. Git rolls back the add which leaves you where you were before the add.
        3. If you completely want to forget the changes, use 
            
            ```Shell
            git checkout -- <filename>
            ```
            
        4. However, how do we undo a commit ?  
            You can surgically remove a commit from history, but we prefer not to rewrite history (to avoid modifying the history of the codebase).  
            Instead, you undo a commit by applying the reverse of what the commit was doing.  
              
            
            ```Shell
            git revert HASH_OF_THE_COMMIT
            
            // Use git log to show all the changes made to a file|
            git log --follow -p -- file_name
            ```
            
              
            
        5. See the changes from your previous commits  
            diff computes the difference with respect to the HEAD  
            
            ```Shell
            git diff file_name
            ```
            
              
            
        
- Vue mixins
    
    [https://medium.com/javascript-in-plain-english/how-to-work-with-mixins-on-vue-js-a61f72737f25](https://medium.com/javascript-in-plain-english/how-to-work-with-mixins-on-vue-js-a61f72737f25)
    
- Vue.set
    
    [telerik.com/blogs/so-what-actually-is-vue-set](http://telerik.com/blogs/so-what-actually-is-vue-set)
    
- Vim
    - Find and replace
        
        :`%s/<search_string>/<replace_string>/g`
        
- SQL
    - What happens if we try to create a table with an existing name?
        
        When you try to create a table with an already existing table name, you will receive an error message, and no table will be modified or created.
        
        Because SQLite (used in the exercises) is case insensitive for most syntax including names, this will apply to any casing of the table name. For instance, given the `celebs` table from this exercise, if you tried to run the following, it will throw an error, because the table name already exists.
        
        ```SQL
        CREATE TABLE Celebs (
           id INTEGER, 
           name TEXT, 
           age INTEGER
        );
        ```
        
    - Is there a shorter way to insert multiple rows in a table?
        
        Yes, instead of inserting each row in a separate `INSERT` statement, you can actually insert multiple rows in a single statement.
        
        To do this, you can list the values for each row separated by commas, following the `VALUES` clause of the statement.
        
        Here is how it would look,
        
        ```SQL
        INSERT INTO table (col1, col2, col3)
        VALUES
        (row1_val1, row1_val2, row1_val3),
        (row2_val1, row2_val2, row2_val3),
        (row3_val1, row3_val2, row3_val3);
        ```
        
    - What order are rows selected from a table?
        
        In most SQL databases, by default, the rows will be selected in the order that they appear in the table, from top to bottom. For instance, if you have a statement like
        
        ```Plain
        SELECT * FROM table
        ```
        
        this will select all rows from the table from the first row that appears down to the bottom row.
        
    - NULL Value
        
        A field with a NULL value is a field with no value.
        
        If a field in a table is optional, it is possible to insert a new record or update a record without adding a value to this field. Then, the field will be saved with a NULL value.
        
        **Note:** A NULL value is different from a zero value or a field that contains spaces. A field with a NULL value is one that has been left blank during record creation!
        
          
        
        It is not possible to test for NULL values with comparison operators, such as =, <, or <>.
        
        We will have to use the IS NULL and IS NOT NULL operators instead.
        
        # IS NULL Syntax
        
        SELECT _column_names_FROM _table_name_WHERE _column_name_ IS NULL;
        
        # IS NOT NULL Syntax
        
        SELECT _column_names_FROM _table_name_WHERE _column_name_ IS NOT NULL;
        
    - Aggregate handling of null values
        
        The aggregate functions – `COUNT`, `SUM`, `AVG`, `MAX`, `MIN` and `LIST` – don't handle `NULL` in the same way as ordinary functions and operators. Instead of returning `NULL` as soon as a `NULL` operand is encountered, they only take non-`NULL` fields into consideration while computing the outcome. That is, if you have this table:
        
        |MyTable|
        |---|
        |[[ID]]|
        |[[1]]|
        |[[2]]|
        |[[3]]|
        |[[4]]|
        |[[5]]|
        
          
          
        
        ...the statement `**select sum(Amount) from MyTable**` returns 54, which is 37 + 5 + 12. Had all five fields been summed, the result would have been `NULL`. For `AVG`, the non-`NULL` fields are summed and the sum divided by the number of non-`NULL` fields.
        
        There is one exception to this rule: `COUNT(*)` returns the count of all rows, even rows whose fields are all `NULL`. But `COUNT`(`_FieldName_`) behaves like the other aggregate functions in that it only counts rows where the specified field is not `NULL`.
        
        Another thing worth knowing is that `COUNT(*)` and `COUNT(``_FieldName_``)` never return `NULL`: if there are no rows in the set, both functions return 0. `COUNT(``_FieldName_``)` also returns 0 if all `_FieldName_` fields in the set are `NULL`. The other aggregate functions return `NULL` in such cases. Be warned that `SUM` even returns `NULL` if used on an empty set, which is contrary to common logic (if there are no rows, the average, maximum and minimum are undefined, but the sum is _known_ to be zero).
        
        Now let's put all that knowledge in a table for your easy reference:
        
        **Table 7. Aggregate function results with different column states**
        
        |Function|Results|
        |---|---|
        |[[Empty set]]|All-null set or column|
        |[[COUNT()]]|0|
        |[[COUNT(Field)]]|0|
        |[[MAX, MIN]]|NULL|
        |[[SUM]]|NULL|
        |[[AVG]]|NULL|
        |[[LISTb]]|NULL|
        
          
          
        
    - How is ALTER different from UPDATE?
        
        Although similar in the sense that both statements will modify a table, these statements are quite different.
        
        The **ALTER** statement is used to modify columns. With `ALTER`, you can add columns, remove them, or even modify them.
        
        The **UPDATE** statement is used to modify rows. However, `UPDATE` can only update a row, and cannot remove or add rows.
        
    - Can we add a column at a specific position to a table?
        
        No, unfortunately, you cannot specify what position to add a column to a table.
        
        By default, a new column will always be added at the end of the table. For most intents and purposes, this should not affect much, since you can always select the columns in any order, for instance, like
        
        ```SQL
        SELECT col3, col1, col2
        ```
        
        If column order is very important, then an alternative is to create a new table and add the columns in the specific order they should appear.
        
    - What if we only want to delete a specific number of rows?
        
        To delete only a specific number of rows, you can utilize the `LIMIT` statement. The value provided for `LIMIT` will be how many rows to affect.
        
        For example, this statement will only delete the first 5 rows that match the condition,
        
        ```Plain
        DELETE FROM tableWHERE condition
        LIMIT 5;
        ```
        
    - Are there any other commonly used SQL commands?
        
        The SQL commands covered in this lesson are probably the most common ones you will encounter or need to use when working with tables. Other available commands are more situational and not as commonly used.
        
        One such command is `DROP TABLE`, which you can use to permanently remove a table from a database. Deleting tables is generally not a frequent occurrence, so you might only use this once in a while. Other commands, such as `ANALYZE`, which is used to obtain statistics about a table, are also not as common and you might only use them in certain situations.
        
    - What types of data can SQLite databases store?
        
        SQLite databases can store several different types of data. Some of the most common data types, which we will encounter in this course, are `INTEGER`, `TEXT` and `REAL`.
        
        The `INTEGER` type is for a signed whole number, such as `25, 0, 100,` …
        
        The `TEXT` type is similar to strings in other programming languages, and stores a sequence of characters like ‘123 parkway street’.
        
        The `REAL` type includes floating point values, like `1.5, 3.141, 103.3333,` …
        
        It is worth noting that the values of each data type can take the state of `NULL`
        
    - Do SQL clauses always have to be entirely capitalized?
        
        No, SQLite, which Codecademy uses, is case-insensitive when it comes to clauses like `SELECT` and `FROM` which can be cased in any way. This is different from other programming languages such as Python where casing is quite important.
        
        ### Example
        
        ```Plain
        /* Both of the following queries will return the same result. */SELECT * FROM table;
        
        select * from table;
        ```
        
    - Can we alias multiple columns in a single query?
        
        Yes, you can alias multiple columns at a time in the same query.
        
        It is advisable to always include quotes around the new alias. Quotes are required when the alias contains spaces, punctuation marks, special characters or reserved keywords. It is simply more convenient to always include quotes around the new alias.
        
        ### Example
        
        ```Plain
        SELECT course_id AS "Course ID", exercise_id AS "Exercise ID"
        FROM bugs;
        ```
        
    - Can we apply DISTINCT to a SELECT query with multiple columns?
        
          
        
        Yes, the `DISTINCT` clause can be applied to any valid `SELECT` query. It is important to note that `DISTINCT` will filter out all rows that are not unique in terms of all selected columns.
        
        Feel free to test this out in the editor to see what happens!
        
        ### Example
        
        Let’s assume that in the Codecademy database there is a table `bugs` which stores information about opened bug reports. It might have columns like `course_id`, `exercise_id`, `reported_by`, `reported_date`, `report_url`, etc. For the purpose of this example, let’s say that this is our table:
        
        ```Plain
        id			course_id	exercise_id	reported_by
        1			5			4			Tod
        2			5			4			Alex
        3			5			3			Roy
        4			5			4			Roy
        5			7			4			Alex
        6			7			8			Tod
        7			14			2			Alex
        8			14			4			Tod
        9			14			6			Tod
        10			14			2			Roy
        ```
        
        Community Manager would like to know the names of the users who have reported bugs in order to send them a special _Thank You_ note. We can use a `SELECT` query with `DISTINCT` keyword to pick unique values from the `reported_by` column:
        
        ```Plain
        > SELECT DISTINCT reported_by FROM bugs;
        
        reported_by
        Alex
        Tod
        Roy
        ```
        
        Awesome! Exactly what we were expecting!
        
        Our coworker would like to know in which exercises bugs have been reported. This gets trickier because now we have to query two columns: `course_id` and `exercise_id`. Let’s try to use the same approach as before:
        
        ```Plain
        > SELECT DISTINCT course_id, exercise_id FROM bugs;
        
        course_id	exercise_id
        14			2
        5			4
        14			4
        14			6
        5			3
        7			4
        7			8
        ```
        
        Is this the result we were hoping for? Yes. It is true that there are duplicated values in the `course_id` and `exercise_id`, but every row is unique (there are no two rows with the same value in `course_id` and `exercise_id`).
        
    - Can we compare values of two columns in a WHERE clause?
        
          
        
        Yes, within a `WHERE` clause you can compare the values of two columns.
        
        When comparing two columns in a `WHERE` clause, for each row in the database, it will check the value of each column and compare them.
        
        ### Example
        
        ```Plain
        /* 
        This will return all rows where the value in the 
        x column is greater than the y column value. 
        */SELECT x, y
        FROM coordinates
        WHERE x > y;
        ```
        
    - Can we apply the LIKE operator to values other than TEXT?
        
          
        
        Yes, you can apply the `LIKE` operator to numerical values as well.
        
        Whenever you use `LIKE` however, you must always wrap the pattern within a pair of quotations, whether for matching a number or a string.
        
        ### Example
        
        ```SQL
        /* 
        This will select movies where the id number
        starts with 2 and is followed by any two numbers.
        */SELECT * 
        FROM movies
        WHERE id LIKE '2__';
        ```
        
    - How do we search for patterns containing the actual characters “%” or “_”?
        
        When searching for a pattern containing the specific characters `%` or `_`, we can utilize the escape character `\`, similarly to its use in Python.
        
        If we want to search for these specific characters, we can simply add the escape character immediately before them.
        
        ### Example
        
        ```Plain
        /* 
        In this pattern, we use an escape character before '%'.
        This will only match "%" and not be used like the
        wildcard character.
        
        This query will match any titles that end with
        ' 100%'.
        */SELECT *
        FROM books
        WHERE title LIKE '% 100\%';
        ```
        
    - When storing missing data, should I store them as NULL?
        
          
        
        It can depend entirely on how you need the data to be stored and utilized.
        
        Let’s say that you have a table of employee information, which included their address. Say that we wanted to check all rows of this table and find where any addresses are missing. If we stored the addresses as `TEXT` values, we might choose to store all the missing values as either `''` or as `NULL`.
        
        If we stored the missing address values as an empty string `''` then these values are not `NULL`. Empty strings are seen as a string of length 0. So, if we ran a query using
        
        `WHERE address IS NULL`
        
        it would not give us the rows with missing address values. We would have to check using
        
        `WHERE address = ''`
        
        With a table containing many different data types, it may be helpful and more convenient to store any missing values in general as just `NULL` so that we can utilize the `IS NULL` and `IS NOT NULL` operators.
        
    - When applying the BETWEEN operator on TEXT values, how are values compared?
        
        In most programming languages, including SQLite and Python, `TEXT` or string values are compared based on their lexicographical ordering, and when using the `BETWEEN` operator for a range of `TEXT` values in SQL, the values will be sorted in this way.
        
        Lexicographical ordering is basically the ordering you would find words in a dictionary. If we had two words, they would be compared starting from their first letter, second letter, and so on, until we find a non-matching letter. The word which has the letter that comes first in the alphabet would ultimately be sorted to come first in this lexicographical ordering.
        
        If two words have different lengths, but match up to the last letter of the shorter word, the shorter word will appear first in the ordering.
        
        ### Example
        
        ```Plain
        A = "Alien"
        B = "Aliens"
        C = "Alike"
        
        /* 
           Because A and B share the same sequence of characters 
           up to the last character of A, which is shorter, A < B. 
        
           Also, because "k" comes after "e" in the alphabet, C will 
           come last in the ordering of these 3 words.
        
           A < B < C
        */
        ```
        
    - Is the `AND` used in `BETWEEN` the same as the `AND` operator between conditions?
        
        No, although they may be assumed to be the same thing, the `AND` used with a `BETWEEN`, like
        
        `BETWEEN 1990 AND 1999`
        
        is not quite the same `AND` used when combining multiple conditions. When used in a `BETWEEN` statement, we are not combining two separate conditions, but providing a range of values to obtain the values within that range.
        
        However, we can easily rewrite a `BETWEEN` to one with two conditions, like these queries which would be identical.
        
        ```Plain
        SELECT *
        FROM movies
        WHERE year BETWEEN 1990 AND 1999;
        
        SELECT * 
        FROM movies
        WHERE year >= 1990
        AND year <= 1999;
        ```
        
    - Can we apply ORDER BY with multiple columns?
        
        Yes, following the `ORDER BY`, you can list more than one column for which to order the data by.
        
        When ordering by more than one column, it will first order the data on the first column, then, keeping the previous column order, it will order on the next column, and so on.
        
        You can also specify ascending or descending order for each listed column.
        
        ### Example
        
        ```Plain
        /* 
        This will order on the year, then order the 
        names in reverse alphabetical 
        order, preserving the order
        of the year column.
        */SELECT year, nameFROM movies
        ORDER BY year ASC, name DESC;
        ```
        
    - When using GROUP BY, do we always have to group by a selected column?
        
        No, you can `GROUP BY` a column that was not included in the `SELECT` statement.
        
        For example, this query does not list the `price` column in the `SELECT`, but it does group the data by that column.
        
        ```Plain
        SELECT name, downloads
        FROM fake_apps
        GROUP BY price;
        ```
        
        However, usually we do include the grouped by column in the `SELECT` for the sake of clarity, so that it’s easier to see what rows belong to which group.
        
    - When doing an INNER JOIN, are columns matched on NULL values?
        
        No, when you have `NULL` values in a column, these are never matched to other `NULL` values. This is because `NULL` signifies the absence of any value, and cannot be compared as they will never equal anything. Doing say `NULL = NULL` results in False.
        
        Let’s take for example the animation given in the exercise, which shows how `INNER JOIN` works. Let’s say that an additional row was added to each table, with `NULL` in the `C2` column, such that they become
        
        Left table:
        
        ```Plain
        C1, C2
        A, B
        Q, W
        X, Y
        T, NULL
        ```
        
        Right table:
        
        ```Plain
        C2, C3
        B, C
        E, R
        Y, Z
        NULL, V
        ```
        
        If we inner joined these tables the same way, we would end up with the same result, because `NULL` values are not matched.
        
        ```Plain
        C1, C2, C3
        A, B, C
        X, Y, Z
        ```
        
    - Does the order of tables in a JOIN matter?
        
          
        
        Generally, no, the order of the tables in the `JOIN` will not affect the overall results of the query.
        
        As long as you specify what columns to select, the results should appear essentially the same, just that the rows will be ordered according to the appearance in the first table.
        
        However, if you do not specify specific columns, then the order of columns will be different depending on the order of the tables. Without specifying columns in your `SELECT`, by using just `SELECT *`, the result query will order the columns by the first table’s columns followed by the second table’s columns.
        
        For example, say we had two tables:`orders`, with the columns `order_id, customer_id`and`customers`, with the columns `customer_id, address`.
        
        If we run the following query,
        
        ```Plain
        SELECT *
        FROM orders
        JOIN customers
        ON orders.customer_id = customers.customer_id;
        ```
        
        the columns in this query’s results will be ordered from left to right as:`order_id, customer_id, customer_id, address`
        
        Conversely, if we ran this query,
        
        ```Plain
        SELECT *
        FROM customers
        JOIN orders
        ON orders.customer_id = customers.customer_id;
        ```
        
        it will return the columns in the following order from left to right:`customer_id, address, order_id, customer_id`
        
        If the order of your columns matters in the result query, then it will be important to keep this in mind.
        
    - What happens if tables we perform UNION on have duplicate rows?
        
        ### Answer
        
        When you combine tables with `UNION`, duplicate rows will be excluded.
        
        To explain why this is the case, recall a Venn Diagram, which shows the relations between sets. If we perform `UNION` on two sets of data (tables), say `A` and `B`, then the data returned in the result will essentially be`A + B - (A intersect B)`
        
        In the first part,`A + B`will add together all the rows of both tables, including duplicates.
        
        The second part,`(A intersect B)`will remove every duplicate, which is where `A` and `B` intersected.
        
        If, however, you wanted to include duplicates, certain versions of SQL provides the `UNION ALL` operator.
        
        - [FAQ: Multiple Tables - Union7](https://discuss.codecademy.com/t/faq-multiple-tables-union/372709)
        
    - Is it possible for a table to have more than one unique identifier column?
        
          
        
        ### Answer
        
        Yes, it is possible for a table to have more than one column which can uniquely identify a row of data. A column that can uniquely identify a record of data is known as a `"Candidate Key"`. Tables can have multiple `"Candidate Key"`s, each of which could potentially be the `"Primary Key"`, but there must only be one `"Primary Key"` per table. Usually, the column chosen as the `"Primary Key"` follows the naming convention like `customer_id` or `product_id`.
        
        For example, say that we had a table of employee records, with the columns `employee_id` and `phone_number`. Every employee has a unique `employee_id` value, and a unique `phone_number` value. Both of these columns can be unique identifiers for a row, so they are `"Candidate keys"`, but the `"Primary Key"` would most likely be set to `employee_id`.
        
    - Can we use WITH for more than one nested query in SQL?
        
        ### Answer
        
        Yes, you can use `WITH` for more than one nested query. You can do so by listing each query using commas after the `WITH`.
        
        For example,
        
        ```Plain
        WITH
        query1 AS (SELECT column1 FROM table1 WHERE condition1),
        query2 AS (SELECT column2 FROM table2 WHERE condition2)
        …
        ```
        
    - IFNULL
        
        Return the specified value IF the expression is NULL, otherwise return the expression:
        
        SELECT IFNULL(NULL, "W3Schools.com");
        
    - SQL Case statement
        
        # The SQL CASE Statement
        
        The CASE statement goes through conditions and returns a value when the first condition is met (like an IF-THEN-ELSE statement). So, once a condition is true, it will stop reading and return the result. If no conditions are true, it returns the value in the ELSE clause.
        
        If there is no ELSE part and no conditions are true, it returns NULL.
        
        # CASE Syntax
        
        CASE WHEN _condition1_ THEN _result1_ WHEN _condition2_ THEN _result2_ WHEN _conditionN_ THEN _resultN_ ELSE _result_END;
        
    - Difference between WHERE and HAVING
        
        The main difference between WHERE and HAVING clause comes when used together with GROUP BY clause, In that case WHERE is used to filter rows before grouping and HAVING is used to exclude records after grouping. This is the most important difference and if you remember this, it will help you write better SQL queries.
        
    - GROUP_CONCAT
        
        The GROUP_CONCAT function returns a string result with the concatenated non-NULL values from a group. It returns NULL if there are no non-NULL values
        
        ```SQL
        SELECT name,
            GROUP_CONCAT(DISTINCT id ORDER BY id DESC SEPARATOR ', ')
            FROM table_name
        GROUP BY name;
        ```
        
        Example Query
        
        ```SQL
        SELECT sell_date, COUNT(DISTINCT(product)) as num_sold, 
        GROUP_CONCAT(DISTINCT product ORDER BY product ASC SEPARATOR ',') as 
        products from Activities group by sell_date;
        ```
        
    - Coalesce
        
        Return the first non-null value in a list:
        
        ```SQL
        # Write your MySQL query statement below
        
        select users.name, coalesce(sum(rides.distance), 0) 
        as travelled_distance from users left join rides on 
        users.id = rides.user_id group by users.name order by 
        travelled_distance desc, users.name asc;
        ```
        
    - Count with having clause
        
        COUNT() with HAVING
        
        The HAVING clause with SQL COUNT() function can be used to set a condition with the select statement. The HAVING clause is used instead of WHERE clause with SQL COUNT() function.
        
        The GROUP BY with HAVING clause retrieves the result for a specific group of a column, which matches the condition specified in the HAVING clause.
        
- C++
    - Pointers
        1. Is an integer that stores a memory address
        2. Memory in the computer is like one big line (or blob)
        3. A pointer, FOR ALL TYPES, is just that integer which stores the memory address
    - Stack vs heap memory
        
        1. Stack has a predefined size. Heap also has a predefined size but it can grow.
        2. Actual location is in the memory.
        3. Stack code
        
        ```C++
        int value = 5;
        int array[5];
        ```
        
        4. Heap code
        
        ```C++
        int* hvalue = new int;
        *hvalue = 5;
        
        int* harray = new int[5];
        
        delete hvalue;
        delete[] harray;
        ```
        
        5. Stack allocation is very fast
        
        6. For heap allocation, you need to delete any memory that you allocate using new.
        
        7.
        
    - Smart Pointers
        1. new allocates new memory and delete deletes unwanted memory
        2. smart pointers automate that process
        3. means that when you call new, you do not have to call delete.
        4. wrapper around a raw pointer
        5. Types of smart pointers
            1. unique_ptr - a scoped pointer that is when this pointer goes out of scope, this will be deleted. You cannot copy a unique pointer.
            2. shared_ptr - uses reference counting (similar to python)
            3. weak_ptr - does not increase the ref_count. (Don't want take ownership of the entity)
        6. Unique Ownership with std::unique_ptr
            1. Syntax
                
                ![[Screenshot_2021-06-21_at_09.45.28.png]]
                
            2. Use std::unique_ptr for exclusive-ownership resource management.
            3. Exclusive Ownership
                
                ![[Screenshot_2021-06-21_at_09.52.12.png]]
                
                ![[Screenshot_2021-06-21_at_09.53.31.png]]
                
            4. unique_ptr is moveable (move-only)
                
                ![[Screenshot_2021-06-21_at_12.12.21.png]]
                
                ![[Screenshot_2021-06-21_at_12.15.11.png]]
                
                ![[Screenshot_2021-06-21_at_12.15.41.png]]
                
                ![[Screenshot_2021-06-21_at_12.18.18.png]]
                
                ![[Screenshot_2021-06-21_at_12.19.38.png]]
                
            5. Custom Deleters in unique_ptr
                
                ![[Screenshot_2021-06-21_at_12.21.35.png]]
                
            6. Rules of thumb for smart pointers
                1. Pass by value
                2. Return by value
                3. Passing a pointer by reference is usually a code smell
                4. A function taking a unique_ptr by value shows the transfer of ownership.
                5. Why wouldnt you take ownership by reference ?
                6. Because passing by value does not really create copy. (The pointer needs to be moved)
        7. std::shared_ptr <T>
            1. shared_ptr expresses shared ownership. Specifically, reference-counting.
            2. Simple reference counting
                
                ![[Screenshot_2021-06-21_at_12.46.35.png]]
                
                ![[Screenshot_2021-06-21_at_12.49.11.png]]
                
            3. std::shared_ptr - The underlying structure
                
                ![[Screenshot_2021-06-21_at_12.52.52.png]]
                
            4. Copying a shared_ptr
                
                ![[Screenshot_2021-06-21_at_12.53.47.png]]
                
            5. ![[Screenshot_2021-06-21_at_12.57.50.png]]
                
        8. Prefer std::make_unique and std::make_shared to direct use of new.
        9. make_shared can be optimized
            
            ![[Screenshot_2021-06-21_at_13.08.56.png]]
            
        10. Use std::weak_ptr for shared_ptr-like pointers that can dangle.
        11. weak_ptr has the same physical layout as a shared_ptr
        
    - Enum vs Enum class
        
        C++ has two kinds of `enum`:
        
        1. `enum class`es
        2. Plain `enum`s
        
        Here are a couple of examples on how to declare them:
        
        ```C++
         enum class Color { red, green, blue }; // enum class
         enum Animal { dog, cat, bird, human }; // plain enum
        ```
        
        **What is the difference between the two?**
        
        - `enum class`es - enumerator names are **local** to the enum and their values do _not_ implicitly convert to other types (like another `enum` or `int`)
        - Plain `enum`s - where enumerator names are in the same scope as the enum and their values implicitly convert to integers and other types
        
        Example:
        
        ```C++
        enum Color { red, green, blue };                    // plain enum
        enum Card { red_card, green_card, yellow_card };    // another plain enum
        enum class Animal { dog, deer, cat, bird, human };  // enum class
        enum class Mammal { kangaroo, deer, human };        // another enum class
        
        void fun() {
        
            // examples of bad use of plain enums:
            Color color = Color::red;
            Card card = Card::green_card;
        
            int num = color;    // no problem
        
            if (color == Card::red_card) // no problem (bad)
                cout << "bad" << endl;
        
            if (card == Color::green)   // no problem (bad)
                cout << "bad" << endl;
        
            // examples of good use of enum classes (safe)
            Animal a = Animal::deer;
            Mammal m = Mammal::deer;
        
            int num2 = a;   // error
            if (m == a)         // error (good)
                cout << "bad" << endl;
        
            if (a == Mammal::deer) // error (good)
                cout << "bad" << endl;
        
        }
        ```
        
        ## Conclusion:
        
        enum classes should be preferred because they cause fewer surprises that could potentially lead to bugs.
        
          
        
    - explicit keyword
        
        The compiler is allowed to make one implicit conversion to resolve the parameters to a function. What this means is that the compiler can use constructors callable with a **single parameter** to convert from one type to another in order to get the right type for a parameter.
        
        Here's an example class with a constructor that can be used for implicit conversions:
        
        ```C++
        class Foo
        {
        public:
          // single parameter constructor, can be used as an implicit conversion
          Foo (int foo) : m_foo (foo)
          {
          }
        
          int GetFoo () { return m_foo; }
        
        private:
          int m_foo;
        };
        ```
        
        Here's a simple function that takes a `Foo` object:
        
        ```C++
        void DoBar (Foo foo)
        {
          int i = foo.GetFoo ();
        }
        ```
        
        and here's where the `DoBar` function is called:
        
        ```C++
        int main ()
        {
          DoBar (42);
        }
        ```
        
        The argument is not a `Foo` object, but an `int`. However, there exists a constructor for
        
        `Foo` that takes an `int` so this constructor can be used to convert the parameter to the correct type.
        
        The compiler is allowed to do this once for each parameter.
        
        Prefixing the `explicit` keyword to the constructor prevents the compiler from using that constructor for implicit conversions. Adding it to the above class will create a compiler error at the function call `DoBar (42)`. It is now necessary to call for conversion explicitly with `DoBar (Foo (42))`
        
        The reason you might want to do this is to avoid accidental construction that can hide bugs.Contrived example:
        
        - You have a `MyString` class with a constructor that constructs a string of the given size. You have a function `print(const MyString&)` (as well as an overload `print (char *string)`), and you call `print(3)` (when you _actually_ intended to call `print("3")`). You expect it to print "3", but it prints an empty string of length 3 instead.
    - Pre-compiled headers
        1. Instead of reading a bunch of header files over and over again, you can compile the header files together which the compiler can then use.
        2. Example the standard template library (defined in header files).  
            However, each time we include the vector, the compiler needs to read that vector header file and compile it.  
            Not only that since vector includes other header files which need to be compiled too.  
            So the compile times would be a lot longer.  
            
        3. So precompiled headers are already compiled binaries containing the headers which is way faster to use.
        4. Drastically reduces compilation time.
        5. Do not put headers that require changes in your precompiled header.
        6. Very useful in the case of external dependencies however be careful as to not including all your dependencies there since it makes it harder to keep track of which files use which dependencies.
        7. Generally, you should put stl in the precompiled headers.
        
    - Safety in C++
        1. There are multiple problems that can arise from allocating memory on the heap.
        2. What happens if you forget to delete the pointer after use. (May cause a memory leak)
        3. The problem of ownership - if you pass around a raw pointer to a block of memory, who is responsible for the management of that pointer - as in who would delete it after use.
        
    - RAII
    - \#define
        
        ### `**#**` **and** `**##**` **operators**
        
        In function-like macros, a `#` operator before an identifier in the _replacement list_ runs the identifier through parameter replacement and encloses the result in quotes, effectively creating a string literal. In addition, the preprocessor adds backslashes to escape the quotes surrounding embedded string literals, if any, and doubles the backslashes within the string as necessary. All leading and trailing whitespace is removed, and any sequence of whitespace in the middle of the text (but not inside embedded string literals) is collapsed to a single space. This operation is called "stringification". If the result of stringification is not a valid string literal, the behavior is undefined.
        
        When `#` appears before `__VA_ARGS__`, the entire expanded __VA_ARGS__ is enclosed in quotes:
        
        ```C++
        \#define showlist(...) puts(#__VA_ARGS__)
        showlist();            // expands to puts("")
        showlist(1, "x", int); // expands to puts("1, \"x\", int")
        ```
        
        A `##` operator between any two successive identifiers in the _replacement-list_ runs parameter replacement on the two identifiers (which are not macro-expanded first) and then concatenates the result. This operation is called "concatenation" or "token pasting". Only tokens that form a valid token together may be pasted: identifiers that form a longer identifier, digits that form a number, or operators `+` and `=` that form a `+=`. A comment cannot be created by pasting `/` and `*` because comments are removed from text before macro substitution is considered. If the result of concatenation is not a valid token, the behavior is undefined.
        
        Note: some compilers offer an extension that allows ## to appear after a comma and before __VA_ARGS__, in which case the ## does nothing when the variable arguments are present, but removes the comma when the variable arguments are not present: this makes it possible to define macros such as `fprintf (stderr, format, ##__VA_ARGS__).`
        
          
        
        Example code -
        
        ```C++
        \#include <iostream>
        
        // Make function factory and use it
        \#define FUNCTION(name, a) int fun_#\#name() { return a;}
        
        FUNCTION(abcd, 12)
        FUNCTION(fff, 2)
        FUNCTION(qqq, 23)
        
        \#undef FUNCTION
        \#define FUNCTION 34
        \#define OUTPUT(a) std::cout << "output: " \#a << '\n'
        
        // Using a macro in the definition of a later macro
        \#define WORD "Hello "
        \#define OUTER(...) WORD #__VA_ARGS__
        
        int main()
        {
        std::cout << "abcd: " << fun_abcd() << '\n';
        std::cout << "fff: " << fun_fff() << '\n';
        std::cout << "qqq: " << fun_qqq() << '\n';
        
        std::cout << FUNCTION << '\n';
            OUTPUT(million);               //note the lack of quotes
        
        std::cout << OUTER(World) << '\n';
        std::cout << OUTER(WORD World) << '\n';
        }
        ```
        
          
        
        Reference -
        
        [https://en.cppreference.com/w/cpp/preprocessor/replace](https://en.cppreference.com/w/cpp/preprocessor/replace)
        
    - friend
        
        The friend declaration appears in a class body and grants a function or another class access to private and protected members of the class where the friend declaration appears.
        
        ### **Description**
        
        1) Designates a function or several functions as friends of this class
        
        ```C++
        class Y {
            int data; // private member
            // the non-member function operator<< will have access to Y's private members
            friend std::ostream& operator<<(std::ostream& out, const Y& o);
            friend char* X::foo(int); // members of other classes can be friends too
            friend X::X(char), X::~X(); // constructors and destructors can be friends
        };
        // friend declaration does not declare a member function
        // this operator<< still needs to be defined, as a non-member
        std::ostream& operator<<(std::ostream& out, const Y& y)
        {
            return out << y.data; // can access private member Y::data
        }
        ```
        
          
        
        2) (only allowed in non-[local](https://en.cppreference.com/w/cpp/language/class#Local_classes) class definitions) Defines a non-member function and makes it a friend of this class at the same time. Such non-member function is always [inline](https://en.cppreference.com/w/cpp/language/inline).
        
        ```C++
        class X {
            int a;
            friend void friend_set(X& p, int i) {
                p.a = i; // this is a non-member function
            }
         public:
            void member_set(int i) {
                a = i; // this is a member function
            }
        };
        ```
        
        3) Designates the class, struct, or union named by the _elaborated-class-specifier_ (see [elaborated type specifier](https://en.cppreference.com/w/cpp/language/elaborated_type_specifier)) as a friend of this class. This means that the friend's member declarations and definitions can access private and protected members of this class and also that the friend can inherit from private and protected members of this class. The name of the class that is used in this `friend` declaration does not need to be previously declared.
        
        4) Designates the type named by the _simple-type-specifier_ or _typename-specifier_ as a friend of this class if that type is a (possibly [cv-qualified](https://en.cppreference.com/w/cpp/language/cv)) class, struct, or union; otherwise, the friend declaration is ignored. This declaration will not forward declare a new type.
        
        ```C++
        class Y {};
        class A {
            int data; // private data member
            class B { }; // private nested type
            enum { a = 100 }; // private enumerator
            friend class X; // friend class forward declaration (elaborated class specifier)
            friend Y; // friend class declaration (simple type specifier) (since c++11)
        };
         
        class X : A::B { // OK: A::B accessible to friend
            A::B mx; // OK: A::B accessible to member of friend
            class Y {
                A::B my; // OK: A::B accessible to nested member of friend
            };
            int v[A::a]; // OK: A::a accessible to member of friend
        };
        ```
        
    - Templates
        1. Template does not really exist until the templated function is called.
        2. Function is created (with definition) at compile time.
        3. Templated classes - (Helps in creating classes configurable at compile time.)
            
            ```C++
            template<typename T, int N>
            class Array
            {
            	private:
            		T m_Array[N];
            	public:
            		int GetSize() const { return N; }
            }
            ```
            
        4. Where would you want to use Templates?  
            Very useful for example in a Logging system. Templates are basically a compiler writing the code for you.  
            
        5. Don't get too crazy with templates.
        6. A function template is not really a function. Its a tool for cranking out functions.
        7. Templates are a recipe for generating functions.
        8. A template argument can be the name of another template instantiation.
        9. You must omit <T> immediately after ::
        10. Inside the scope of the class, forget the angle brackets.
        11. In C++17, a static data member can be defined within its class template definition by using the keyword inline.
        12. A Template argument can be an expression rather than a type.
        13. In case of templates, you place all template declarations, including definitions, in headers.
        14. You rarely place template declarations in source files.
        15. A namer used in a template declaration that is dependent on a template parameter is assumed not to name a type unless the name is qualified by the keyword _typename_.
            
            ```C++
            
            template <typename T>
            // Here you cannot use class instead of typename
            typename T::size_type munge(T const& a) {
            	typename T::size_type* i(T::npos);
            }
            ```
            
        
    - override
        
        # Override: a useful feature to prevent bugs
        
        `override` is a feature to use without moderation! Every time you define a method in the derived class that overrides a `virtual` method in the base class, you should tag it `override`:
        
        ```C++
        class Base
        {
        public:
            virtual void f()
            {
                std::cout << "Base class default behaviour\n";
            }
        };
        
        class Derived : public Base
        {
        public:
            void f() override
            {
                std::cout << "Derived class overridden behaviour\n";
            }
        };
        ```
        
        This way you show that your intention for the derived class is to override the behaviour of `f` in the base class.
        
        Note that in term of execution, the above code is equivalent to this one:
        
        ```C++
        class Derived : public Base
        {
        public:
            void f()
            {
                std::cout << "Derived class overridden behaviour\n";
            }
        };
        ```
        
        Even without writing `override`, `f` overrides the behaviour of its counterpart in the base class (as long as the `f` of the base class is `virtual` and has the same prototype). So `override` is really about expressing your intentions.
        
    - Name Mangling
        1. C++ introduced overloaded functions which required the compiler to know the difference between different function implementations.
        2. This was achieved by mangling the names of the arguments.
        
    - Function Pointers
        1. Function pointers are essentially a way to assign a function to a variable
        2. Basic Example
            
            ```C++
            \#include <iostream>
            
            void HelloWorld()
            {
            	std::cout << "Hello world" << std::endl;
            }
            
            int main()
            {
            
            	typedef void(*HelloWorldFunction)(int);
            	
            	// Function pointer
            	// Using the memory address of the function
            	auto function = &HelloWorld;
            
            	// The type in this case is
            	// void(*function)();
            
            	// Now we can call the function
            	function();
            
            	std::cin.get();
            }
            ```
            
        3. Useful example  
              
            
            ```C++
            \#include <iostream>
            \#include <vector>
            
            void PrintValue(int value)
            {
            	std::cout << "Value : " << value << std::endl;
            }
            
            void ForEach(const std::vector<int>& values,
            						void(*func)(int))
            {
            	for(int value: values)
            		{
            			func(value);
            		}
            }
            
            int main()
            {
            	std::vector<int> values = {2,4,5};
            	ForEach(values, PrintValue);
            	// Or we can use a lambda function
            	ForEach(values, [](int value) {std::cout << "Value: " << value << std::endl;});
            	std::cin.get();
            }
            ```
            
    - Lambda functions
        1. Way to define an anonymous function
        2. Lambdas reduce boilerplate code.
        3. Whenever you have a function pointer, you can use a lambda.
        4. Lambda expressions reference -
            
            [https://en.cppreference.com/w/cpp/language/lambda](https://en.cppreference.com/w/cpp/language/lambda)
            
        5. If we want to have outside variables inside our lambda function, we need to "capture" them.
        6. Using = or & we can define either to pass everything by value or reference respectively.
        7. Most useful lambda is [&]() (Capture by reference)
        8. You can define the state by defining a data member in the square brackets. All instances of this lambda will have their own counter i.  
            By default, lambda types operator () is const  
            
            ```C++
            [i=0]() mutable { return ++i; };
            ```
            
        9. Lambdas + Templates = Generic Lambdas
            
            ```C++
            auto plus = [value=1](auto x) { return x + value};
            ```
            
        10. Generic lambdas are just templates under the hood.
        11. **What is 'this' in a lambda?**  
            this does not mean the lambda object. It means whatever this means in the outer scope.  
            
        12. **How do I name the parameter types of a generic lambda?  
              
            **You can use decltype.
        
    - Small string optimization
    - std::async
    - std::bind
    - std::function
    - lvalues and rvalues
        1. You cannot assign something to an rvalue
            
            ```C++
            int main() {
              // i here is an lvalue and 10 is the 
            	// rvalue
            		int i = 10;
            		GetValue() = 109;
            }
            
            // We are returning an lvalue reference
            int& GetValue()
            {
            	static int val = 10;
            	return val;
            }
            ```
            
        2. You cannot take an lvalue reference from an rvalue.
        3. rvalue have a temporary memory.
        4. You CAN take a const lvalue reference from an rvalue.
        5. rvalues are basically temporary objects
        6. An rvalue reference uses 2 ampersands and we cannot pass in an lvalue then
            
            ```C++
            void PrintName(std::string&& name)
            {
            	std::cout << name << std::endl;
            }
            
            std::string name= "Nav";
            
            // This will throw error
            PrintName(name);
            
            // This will work fine
            PrintName(name + "Rabdionm");
            ```
            
        7. An rvalue reference cannot be bound to an lvalue.
    - Move semantics
    - Looking at what the compiler does for you?
        1. Which one of the following is better? 
            
            ```C++
            int sum(const std::vector<int>& a) {
            	int result = 0;
            	for (size_t i =0; i < a.size(); ++i) {
            		result += a[i];
            	}
            	return result;
            }
            ```
            
            ```C++
            int sum(const std::vector<int>& a) {
            	int result = 0;
            	for (int x: a) result += x;
            	return result;
            }
            ```
            
        2. Always use google benchmark (or some other) instead of relying on reading assembly code to make sense of which code is 'better'.
        3. [https://quick-bench.com/](https://quick-bench.com/)
        4. x86 Assembly 101 -
            1. Has 16 64 bit registers.
            2. rax is return value register
            3. instructions can take anywhere from 0 to 3 operands
            4. op is eg. call, ret, add, sub, cmp etc
            5. dest, src is register or memory reference.
                
                ![[Screenshot_2021-06-14_at_13.18.09.png]]
                
        5. Difference between the traditional for loop and the range based for loop
            
            ![[Screenshot_2021-06-14_at_13.41.26.png]]
            
        6. Multiplication example
            
            ![[Screenshot_2021-06-14_at_13.46.18.png]]
            
        7. Can we do better than the above shown multiplication?  
            Maybe not  
            
            ![[Screenshot_2021-06-14_at_13.50.48.png]]
            
        8. Let the compilers do the stuff for you since 9 times out of 10, it would be smarter than you.
        9. Division takes lot more cpu cycles than multiplication.
        10. Clever constant division  
              
            
            ![[Screenshot_2021-06-14_at_15.14.10.png]]
            
        
    - Introduction to LLVM
        1. What is LLVM IR ?
            1. LLVM IR is a generic assembly language
            2. Easy to lower to different targets
            3. Example
                
                ![[Screenshot_2021-06-17_at_09.33.20.png]]
                
            4. Instructions in LLVM IR
                1. Have an opcode
                2. Take a set of input operands
                3. Produce one or zero result values
                4. Explicitly typed
            5. Important instruction classes
                1. Arithmetic instructions such as add, sub etc
                2. Compare constructions
                3. Control flow instructions
                4. Call instructions
                5. Load and store instructions
            6. Basic Blocks:
                1. Can have a block
                2. Consist of a list of instructions that are executed sequentially
                3. Last instruction is called terminator
                4. In the above example, entry is one such block.
            7. Functions:
                1. Have a name and type signature (eg. Foo)
                2. Consists of a list of basic blocks
                3. The first basic block is the special entry block but the flow between the other blocks is determined by the block terminators.
                    
                    ![[Screenshot_2021-06-17_at_09.46.52.png]]
                    
            8. Module:
                1. Top level container of a program containing
                    1. Functions
                    2. Declarations
                    3. Global variables
        2. Common terminology
            1. Result value of an instruction is referred to by an identifier, they are not variables. (We cannot assign to them and they need to be unique)
            2. def and uses
                
                ![[Screenshot_2021-06-17_at_09.55.33.png]]
                
            3. %cond is a **user** of %sum
            4. Can access the list of uses for any given def
                
                ![[Screenshot_2021-06-17_at_09.58.22.png]]
                
            5. All uses must be reachable from their def
                
                ![[Screenshot_2021-06-17_at_10.01.27.png]]
                
        3. Three different forms of IR
            
            ![[Screenshot_2021-06-17_at_10.04.03.png]]
            
        4. IR Transformation Examples
            1. Removing Dead blocks
                
                ![[Screenshot_2021-06-17_at_10.06.23.png]]
                
            2. How can we get the predecessors of a block ?
                
                ![[Screenshot_2021-06-17_at_10.12.08.png]]
                
            3. Implementation
                
                ![[Screenshot_2021-06-17_at_10.19.53.png]]
                
            4. Simplifying Conditional branches
                
                ![[Screenshot_2021-06-17_at_10.22.46.png]]
                
            5. Implementation
                
                ![[Screenshot_2021-06-17_at_10.41.12.png]]
                
        5. Target-specific pass information
            1. Middle-end passes simplify programs and use heuristics to improve them.
            2. Middle-end passes try to minimize the number of IR instructions and simplify control flow.
            3. Keep IR generic
                
                ![[Screenshot_2021-06-17_at_10.46.54.png]]
                
        
    - LLVM Primer
        1. First step for a compiler is that it takes the code , Parses it and generates an AST (Abstract syntax tree)
        2. AST
            
            ![[Screenshot_2021-06-17_at_15.45.19.png]]
            
        3. The next step is the Code generator. A code generator takes in the AST and generates the machine code (Binary code).
            
            ![[Screenshot_2021-06-17_at_15.48.46.png]]
            
        4. The assembler is responsible for converting assembly language to machine code.
        5. Clang is like the proof of concept for the LLVM core.
        
    - Overview of clang
    - Writing an LLVM pass: 101
        1. Setup and background
            1. LLVM Pass - Analysis vs Transformation
                1. A pass operates on some unit of IR (eg. Module or Function)
                    1. Transformation will modify it
                    2. Analysis will generate some high-level information
                2. Analysis results are produced lazily
                3. Transformation pass managers record what is preserved.
            2. LLVM IR files- function vs module
                1. An IR module (Modules and functions are units of IR)
                    
                    ![[Screenshot_2021-06-17_at_11.42.16.png]]
                    
            3. OPT
                1. LLVM's modular optimizer
                2. Takes an LLVM source file
                    1. optimisation - returns an LLVM source file (output.ll)
                    2. analysis - produces analysis results
                        
                        ![[Screenshot_2021-06-17_at_11.44.46.png]]
                        
                    3. load plugins - i.e. shared objects with custom passes.
        2. HelloWorld Pass
            1. This first pass is very basic. It will just visit a function, print it's name and print the number of arguments it takes.
            2. Implementation
                
                ![[Screenshot_2021-06-17_at_11.50.36.png]]
                
            3. HelloWorld - registration
                
                ![[Screenshot_2021-06-17_at_12.47.35.png]]
                
        3. Transformation Pass
            
            ![[Screenshot_2021-06-17_at_13.15.50.png]]
            
            ![[Screenshot_2021-06-17_at_13.18.05.png]]
            
        4. Analysis Tool
            1. Static analysis Tool
                
                ![[Screenshot_2021-06-17_at_13.30.02.png]]
                
                  
                
                ![[Screenshot_2021-06-17_at_15.19.20.png]]
                
                ![[Screenshot_2021-06-17_at_15.24.18.png]]
                
        5. Integration with Opt
        6. Testing
        7. Final Hints
    - Building useful tools with LLVM and clang
    - Concurrency in C++
        1. What is a data race and how do we fix it ?
            1. What is concurrency ?
                
                ![[Screenshot_2021-06-18_at_09.39.04.png]]
                
            2. Compiler rewriting accesses
                
                ![[Screenshot_2021-06-18_at_09.43.31.png]]
                
            3. Hardware reordering accesses
                
                ![[Screenshot_2021-06-18_at_09.45.10.png]]
                
            4. Memory Model
                
                ![[Screenshot_2021-06-18_at_09.47.17.png]]
                
            5. Starting a new thread
                
                ![[Screenshot_2021-06-18_at_10.01.32.png]]
                
            6. Joining finished threads
                
                ![[Screenshot_2021-06-18_at_10.17.11.png]]
                
            7. Getting the result of a thread
                
                ![[Screenshot_2021-06-18_at_10.33.57.png]]
                
            8. Example of a data race
                
                ![[Screenshot_2021-06-18_at_11.45.44.png]]
                
            9. Fixing the race via std::atomic<T>
                
                ![[Screenshot_2021-06-18_at_11.50.03.png]]
                
            10. Logical synchronization
                
                ![[Screenshot_2021-06-18_at_11.54.10.png]]
                
            11. Threads always startup running immediately. There is no thread.start method. To make the thread wait we can use synchronization primitives.
            12. Using atomic bool
                
                ![[Screenshot_2021-06-18_at_11.59.41.png]]
                
            13. std::mutex  
                  
                
                ![[Screenshot_2021-06-18_at_12.09.17.png]]
                
            14. std::mutex is frequently used to protect data.
                
                ![[Screenshot_2021-06-18_at_12.23.26.png]]
                
            15. Protection must be complete
                
                ![[Screenshot_2021-06-18_at_12.59.58.png]]
                
            16. What about exception safety ?
                
                ![[Screenshot_2021-06-18_at_13.04.34.png]]
                
                1. Freeing resources, freeing heap allocated pointers or unlocking mutexes must be done inside the destructor.
                    
                    ![[Screenshot_2021-06-18_at_13.07.22.png]]
                    
                2. CTAD - Class template argument deduction
                3. A mutex lock is a resource.
                    
                    ![[Screenshot_2021-06-18_at_13.08.48.png]]
                    
                    ![[Screenshot_2021-06-18_at_13.11.11.png]]
                    
                4. If you are not going to pass it around, use lock_guard since it is a little more efficient. (Emphasis on the little)
                5. unique_lock (like unique_ptr) is movable but not copyable.
                6. scoped_lock
                    
                    ![[Screenshot_2021-06-18_at_13.17.06.png]]
                    
                7. condition_variable for "wait until"
                    
                    ![[Screenshot_2021-06-18_at_13.25.02.png]]
                    
                    ![[Screenshot_2021-06-18_at_13.28.00.png]]
                    
                8. Producer-Consumer paradigm - Mutex + condition_variable
                    
                    ![[Screenshot_2021-06-18_at_13.32.26.png]]
                    
                9. Waiting for initialization
                    
                    ![[Screenshot_2021-06-18_at_13.34.13.png]]
                    
                    ![[Screenshot_2021-06-18_at_13.35.46.png]]
                    
                10. How to initialize a data member
                    
                    ![[Screenshot_2021-06-18_at_15.51.08.png]]
                    
                    ![[Screenshot_2021-06-18_at_15.51.54.png]]
                    
                11. once_flag
                    
                    ![[Screenshot_2021-06-18_at_15.53.54.png]]
                    
                12. Comparison of primitives
                    
                    ![[Screenshot_2021-06-18_at_16.20.57.png]]
                    
                13. Shared mutex (Reader / Writer Lock)
                    
                    ![[Screenshot_2021-06-18_at_16.22.52.png]]
                    
                14. counting_semaphore
                    
                    ![[Screenshot_2021-06-18_at_16.28.30.png]]
                    
                    ![[Screenshot_2021-06-18_at_16.31.04.png]]
                    
                15. std::latch - Non-reusable starting gate
                    
                    ![[Screenshot_2021-06-18_at_16.44.18.png]]
                    
                16. std::barrier (A resettable latch)
                    
                    ![[Screenshot_2021-06-18_at_16.45.27.png]]
                    
                17. Synchronization with std::latch
                    
                    ![[Screenshot_2021-06-18_at_16.49.11.png]]
                    
                18. No synchronization primitives are copyable.
                19. Synchronization with std::barrier
                    
                    ![[Screenshot_2021-06-18_at_16.51.58.png]]
                    
                20. Comparison of C++20 primitives
                    
                    ![[Screenshot_2021-06-18_at_16.54.19.png]]
                    
                21. Promise/Future
                    
                    ![[Screenshot_2021-06-18_at_17.01.26.png]]
                    
                22. Patterns for sharing data
                    
                    ![[Screenshot_2021-06-18_at_17.05.31.png]]
                    
                23. Blue/Green pattern
                    
                    ![[Screenshot_2021-06-18_at_17.06.30.png]]
                    
                    ![[Screenshot_2021-06-18_at_17.07.55.png]]
                    
        2. C++ 11 mutex and RAII lock types
        3. condition_variable
        4. Static initialization and once_flag
        5. New C++17 and C++20 primitives
        6. The blue/green pattern
    - Memory consistency
        1. Memory consistency is a concurrency problem.
        2. The real issues occur when you have 2 or more processors executing the code.
        3. In a modern multi-processor environment, the program you wrote is almost certainly NOT the program that is executed
        4. An optimizing compiler can heavily refactor your code in order to hide pipeline latencies or take advantage of micro-architectural optimizations.  Although there are a large number of optimisations a compiler can apply, from our perspective there are two that are significant:
            - It can remove what it considers redundant reads or writes.
            - It can decide to move a memory access earlier in order to give it more time to complete before the value is required, or later in order to balance out the accesses through the program.
        5. These optimizations allow the processor to execute instructions out of order to reduce latency between opcodes and remove stalls while waiting for data.
        6. Memory optimization features like caches, interconnects and write buffers minimize the impact of high latency memory operations by allowing the processor to read / write from low-latency memory structures.
        7. There is often asynchronicity between the processor and the main memory.
        8. **Breaking the sequential execution illusion**
            
            In the example below we’ve got a four processor system. We are using two flags to synchronise the execution of threads C and D.  Given this code it is reasonable to speculate that the output from the program will either be “flagA”, “flagB” (or nothing).   The condition statements for Thread C and Thread D must _never_ both be true.
            
            [![](https://i2.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Slide12.png?resize=640%2C404&ssl=1)](https://i2.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Slide12.png?resize=640%2C404&ssl=1)
            
            Notice, to show the effects of read / write of the two flags I’ve added reads of the flags into local variables.  Consider this as the equivalent of reading the values into registers.
            
            On the diagram below time flows down the page so you can see relative timings and sequencing between the operations.
            
            In the simple case where there is no optimisation the sequential execution constraint holds true.
            
            [![](https://i1.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Slide13.png?resize=640%2C417&ssl=1)](https://i1.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Slide13.png?resize=640%2C417&ssl=1)
            
            A simple compiler optimisation can cause a failure of our sequential execution.  In this case Thread C’s code is optimised so that it reads flagB before flagA.  It is now possible to provide an execution sequence where both Thread C and Thread D’s conditions are true; and both messages are output.
            
            [![](https://i2.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Slide14.png?resize=640%2C415&ssl=1)](https://i2.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Slide14.png?resize=640%2C415&ssl=1)
            
            In Thread C:
            
            - flagB is read before it is set by Thread B; hence b = false
            - flagA is read after it is set by Thread A; hence a = true
            
            In Thread D:
            
            - flagA is read before its value is set by Thread A; hence a = false
            - flagB is read after its value is set by Thread B; hence b = true
            
            Please note, the same effect could occur as the result of a processor out-of-order execution, or by a memory optimisation architecture (for example a write-back buffer).  Remember the rule:  you can’t guarantee read order for main memory reads (which I’ve just exploited), and writes are asynchronous (which would potentially cause the same effect)
            
        9. How to solve the memory inconsistency problem ?  
            Let us take the example of 2 thread classes - a Producer, which creates data and inserts it onto a stack; and a Consumer, that retrieves data from a stack.  
              
            
            ```C++
            class Producer
            {
            	public:
            		Producer(SimpleStack& s) :
            			sum {0}, stack {&s} {}
            
            	protected:
            		void operator()();
            
            	private:
            		int sum;
            		SimpleStack* stack;
            };
            
            void Producer::operator()()
            {
            	for(int i=0; i < 1000; ++i)
            	{
            		stack->push(i);
            		sum += i;
            	}
            	std::cout << "Produced : " << sum << std::endl;
            }
            ```
            
              
            
            ```C++
            class Consumer
            {
            	public:
            		Consumer(SimpleStack& s) :
            			sum {0},
            			stack {&s} {}
            
            		protected:
            			void operator()();
            		
            		private:
            			int sum;
            			SimpleStack* stack;
            		
            };
            
            void Consumer::operator()()
            {
            	for(int i = 0; i < 1000; ++i)
            	{
            		int val = stack->pop();
            		sum += val;
            	}
            	std::cout << "Consumed: " << sum << std::endl;
            }
            ```
            
              
            
            ```C++
            // Basic Stack Implementation
            \#include <cstdint>
            
            class SimpleStack
            {
            	public:
            		bool push(int val);
            		int pop();
            
            	private:
            		static constexpr uint32_t size { 10 };
            		uint32_t count { 0 };
            		int stack[size];
            }
            
            bool Stack::push(int val) {
            	
            	if (count == size) return false;
            
            	stack[count] = val;
            	++count;
            	return true;
            }
            
            int Stack::pop() {
            	if (count == 0) return -1;
            
            	--count;
            	int val = stack[count];	
            	return val;
            }
            ```
            
              
            
            There is a problem with the stack array since one thread is inserting into the array and the other thread is reading from the array. This gives us the potential for a data race.
            
            ## Part 1 – No data races
            
            A data race can be defined as follows:
            
            > When an expression in one thread of execution writes to a memory location and another expression in a different thread of execution reads or modifies the same memory location, the expressions are said to conflict. A program that has two conflicting expressions has a data race.
            
            In our example the memory location is not the stack array (although the symptoms of this problem will be exhibited in the array) but the count variable, which represents the current top-of-stack . Both the Consumer and Producer threads modify the count. If there is any interleaving of program opcodes that could result in a write instruction being next to a read instruction there exists a data race condition.
            
            To better understand the issue we have to look at the generated assembler code for the Stack.  The assembly code in this article are ARM Cortex M Thumb2 instructions, generated by the arm-gcc compiler (with the optimiser turned off to enhance readability)
            
            [![](https://i1.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Atomics_Slide04.png?resize=640%2C375&ssl=1)](https://i1.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Atomics_Slide04.png?resize=640%2C375&ssl=1)
            
            Notice that the modification of the count object is not a single operation – the read, modify and write operations are independent opcodes.
            
            Also notice the count value is only read once, even though the source code states to read it twice – once on the check for full/empty and once when the stack array is accessed.  A simple analysis by the compiler identifies that no program expression modifies the count between the two reads; so it ignores the second read (known as a _redundant load_).
            
              
            
            If we trace a sample execution timeline it is trivial to demonstrate we have a race condition in our _Stack._
            
            [![](https://i1.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Atomics_Slide05.png?resize=640%2C442&ssl=1)](https://i1.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Atomics_Slide05.png?resize=640%2C442&ssl=1)
            
            The two read operations don’t conflict.  On almost all modern processors a read (load) is a single opcode; thus multiple threads reading from the same location should never cause us a problem.
            
            The two interleaved writes are the issue in this case.  The modification (increment) of the count made by the Producer thread is overwritten by the Consumer thread write.  The next push() by the Producer thread is likely to overwrite the previous value.
            
            Our first criteria for sequentially consistent code must therefore be:  **You cannot have a race condition in your program.**
            
              
            
            The easiest way to achieve this is to guarantee an ‘exclusive’ or ‘atomic’ read-modify-write operation on main memory objects. A normal object does not necessarily provide that guarantee (it may happen on some processor families, but not on others). Therefore we are required as programmers to tell the compiler that an object is ‘special’ and requires different operations for read/write compared to normal objects.
            
            std::atomic is a template class that can be parameterized with any scalar type (that is, built-in type).
            
            _(There are type aliases for all the usual scalar types but there are so many of them, and they’re kind of clumsy, so it’s generally much easier to just use the template)_
            
            [![](https://i1.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Atomics_Slide09.png?resize=640%2C419&ssl=1)](https://i1.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Atomics_Slide09.png?resize=640%2C419&ssl=1)
            
            Atomic types are guaranteed to implement atomic (indivisible) read and read-modify-write operations on their underlying object.
            
            Atomic types have a basic load() / store() access interface.  There are also atomic read-modify-write operations available, for example fetch_add() and fetch_sub()as used here.  Operator overloads for common manipulations are provided.
            
            ## Load-acquire / Store-release semantics
            
            std::atomics mitigates the problem of optimizing transformations by simulating what is referred to as _Read-acquire_ / _Store-release_ semantics.
            
            _Load-acquire_ semantics is a property that can only apply to operations that read from shared memory, whether they are read-modify-write operations or plain loads.  Acquire semantics prevent memory reordering of the read-acquire with any read or write operation which follows it in program order.
            
            [![](https://i2.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Atomics_Slide15.png?resize=640%2C397&ssl=1)](https://i2.wp.com/blog.feabhas.com/wp-content/uploads/2016/07/Atomics_Slide15.png?resize=640%2C397&ssl=1)
            
            _Store-release_ semantics is a property that can only apply to operations that write to shared memory, whether they are read-modify-write operations or plain stores. Release semantics prevent memory reordering of the write-release with any read or write operation which precedes it in program order.
            
            Note that Read-acquire / Store-release are hardware _and_ software concepts.  In terms of software, these concepts prevent compiler reorganization of std::atomic reads and or writes.  From a hardware perspective, these concepts require the compiler to issue synchronization instructions (barriers, or fences).
            
            Using a std::atomic type provides all the guarantees of volatile objects but (by default) also guarantees the order of reads / writes between atomic objects.  Implementations for atomic types do this by not only disabling compiler optimisations but also hardware optimisations.
            
            **The use of atomics should therefore be reserved for sharing data between units-of-execution; and their use should be minimised.**
            
              
            
    - C++ Threads
        1. Threads example
            
            ```C++
            \#include <iostream>
            \#include <thread>
            
            
            static bool s_Finished = false;
            
            void DoWork()
            {
            	using namespace std::literals::chrono_literals;
            	std::cout << std::this_thread::get_id() << std::endl;
            	while (!s_Finished)
            	{
            		std::cout << "Working...\n";
            		// Make this thread sleep for 1s
            		std::this_thread::sleep_for(1s);
            	}
            }
            
            int main()
            {
            	// Pass in a function pointer
            	std::thread worker(DoWork);
            
            	std::cin.get();
            	s_Finished = true;
            	
            	// Wait for the worker thread to end
            	worker.join();
            
            		
            	
            	std::cin.get();
            }
            ```
            
    - C++ ABI
    - Default arguments in virtual functions
        
        In more detail, yes, you can specify different default parameters. They won't work the same way as the virtual functions. A virtual function is called on the dynamic type of the object, while the default parameter values are based on the static type.
        
        Given
        
        ```C++
        class A {
            virtual void foo(int i = 1) { cout << "A::foo" << i << endl; }
        };
        class B: public A {
            virtual void foo(int i = 2) { cout << "B::foo" << i << endl; }
        };
        void test() {
        A a;
        B b;
        A* ap = &b;
        a.foo();
        b.foo();
        ap->foo();
        }
        ```
        
        you should get A::foo1 B::foo2 B::foo1  
          
          
          
          
        [http://www.gotw.ca/gotw/005.htm](http://www.gotw.ca/gotw/005.htm)  
          
        
        This is a bad idea because the default arguments you get will depend on the _static_ type of the object, whereas the `virtual` function dispatched to will depend on the _dynamic_ type.
        
        That is to say, when you call a function with default arguments, the default arguments are substituted at compile time, regardless of whether the function is `virtual` or not.
        
          
        
        ```C++
        struct A {
            virtual void display(int i = 5) { std::cout << "Base::" << i << "\n"; }
        };
        struct B : public A {
            virtual void display(int i = 9) override { std::cout << "Derived::" << i << "\n"; }
        };
        
        int main()
        {
            A * a = new B();
            a->display();
        
            A* aa = new A();
            aa->display();
        
            B* bb = new B();
            bb->display();
        }
        ```
        
        Which produces the following output:
        
        ```C++
        Derived::5
        Base::5
        Derived::9
        ```
        
        With the aid of the explanation above, it is easy to see why. At compile time, the compiler substitutes the default arguments from the member functions of the static types of the pointers, making the `main` function equivalent to the following:
        
        ```C++
            A * a = new B();
            a->display(5);
        
            A* aa = new A();
            aa->display(5);
        
            B* bb = new B();
            bb->display(9);
        ```
        
    - References and RVO
        
        Hey I noticed you’ve been suggesting to add `&` to return values in a couple of C++ PRs — what’s the motivation behind this? This triggers a pretty obscure part of C++ called “reference lifetime extension” and it’s [usually discouraged](https://abseil.io/tips/107):
        
        > you probably shouldn’t rely on lifetime extension in the explicit case of reference-initialization: it’s not gaining you much/any performance, and it is subtle, fragile, and prone to cause extra work for your reviewers and future maintainers
        
        [**abseil.io**](http://abseil.io/)[**abseil / Tip of the Week #107: Reference Lifetime Extension**](https://abseil.io/tips/107)An open-source collection of core C++ library code
        
        [![](https://slack-imgs.com/?c=1&o1=wi32.he32.si&url=https%3A%2F%2Fabseil.io%2Ffavicons%2Ffavicon.ico)](https://slack-imgs.com/?c=1&o1=wi32.he32.si&url=https%3A%2F%2Fabseil.io%2Ffavicons%2Ffavicon.ico)
        
        **abseil.io**
        
        [**abseil / Tip of the Week #107: Reference Lifetime Extension**](https://abseil.io/tips/107)
        
        An open-source collection of core C++ library code
        
    - Structured concurrency
        
          
        
    - Expect the expected
        
        [https://youtu.be/PH4WBuE1BHI](https://youtu.be/PH4WBuE1BHI)
        
- Software Design Patterns
    1. Design solution to a commonly met problem.
    2. Not universally applicable.
    3. Layered Pattern is the most important pattern used in Bloomberg.
    4. Brokers and Events busses (A publisher-subscriber architecture) is used.
    5. Pipe-filter (similar to the bash commands pipe) are also very important.
    6. N-tier (Layered) architecture pattern - Higher level layer depends on the lower layer but not the other way around.  
          
        The benefit of this approach is that changing the presentation layer does not really affect the lower layers.  
        
    7. Bloomberg terminal has 3 tiers.
    8. The difference is that the presentation layer is split between the client and server.
- Compiler Design
    - Operator Precedence Parsing
        1. Operator Grammar - A grammar that is used to define the mathematical operations with some restrictions on the grammar.
        2. Operator Grammar - E → E + E / E * E / id
        3. Identifiers cannot be side-to-side.
        4. Operator relation table
            
            ![[Screenshot_2021-06-25_at_08.53.03.png]]
            
        5. Even though the grammar does not talk about the precedence, the relation table does talk about the precedence.
- Python
    - Multi-threading in Python> [!important]  
        > The code is actually pretty similar to multiprocessing.  
        
        1. Benefits when tasks are IO bound.
        2. Use multi-processing when task is CPU bound.
        3. When we run tasks concurrently, it does not run the task at the same time. (Not parallel)
        
        ```Python
        import time
        import threading
        
        start = time.perf_counter()
        
        def do_something():
        	print('Sleeping 1 second')
        	time.sleep(1)
        	print('Done sleeping')
        
        thread1 = threading.Thread(target=do_something)
        thread2 = threading.Thread(target=do_something)
        
        t1.start()
        t2.start()
        
        t1.join()
        t2.join()
        
        finish = time.perf_counter()
        
        print(f'Finished in {round(finish - start, 2)} second(s)')
        ```
        
          
        
        **Faster and easier way**
        
        Thread Pool Executor
        
        ```Python
        import time
        import concurrent.futures
        
        start = time.perf_counter()
        
        def do_something():
        	print('Sleeping 1 second')
        	time.sleep(1)
        	print('Done sleeping')
        
        
        with concurrent.futures.ThreadPoolExecutor() as executor:
        	f1 = executor.submit(do_something, 1)
        
        thread1 = threading.Thread(target=do_something)
        thread2 = threading.Thread(target=do_something)
        
        t1.start()
        t2.start()
        
        t1.join()
        t2.join()
        
        finish = time.perf_counter()
        
        print(f'Finished in {round(finish - start, 2)} second(s)')
        ```
        
          
        
    - Multi-processing in Python> [!important]  
        > Watch this video for more information - https://www.youtube.com/watch?v=fKl2JW_qrso  
        
        If we have tasks that need not be run synchronously, then we can use the multiprocessing module in Python.
        
        Tasks can be either -  
        1. CPU bound or  
        2. IO-bound. Eg network operations or file system operations.  
          
        
        Threading does not really improve performance on CPU bound tasks because those threads are still only running one process.
        
          
        
        Processes take a little more time compared to threads to spin up.
        
          
        
        Unlike threads, when passing arguments to processes you need to ensure that the arguments can be serialized using pickle. (i.e. converting python objects to a format that can be deconstructed and reconstructed in another python script).  
          
        
        **Synchronous**
        
        ```Python
        import time
        
        start = time.perf_counter()
        
        def do_something():
        	print('Sleeping 1 second')
        	time.sleep(1)
        	print('Done sleeping')
        
        
        do_something()
        do_something()
        
        finish = time.perf_counter()
        
        print(f'Finished in {round(finish-start, r)} seconds(s)')
        ```
        
          
        
        **Multiprocessing**
        
        ```Python
        import time
        import multiprocessing
        start = time.perf_counter()
        
        def do_something(seconds):
        	print(f'Sleeping {seconds} second(s)')
        	time.sleep(seconds)
        	print('Done sleeping')
        
        processes = []
        for _ in range(10):
        	p = multiprocessing.Process(target=do_something, args=[1.5])
        	p.start()
        	processes.append(p)
        
        for p in processes:
        	p.join()
        
        finish = time.perf_counter()
        
        print(f'Finished in {round(finish-start, r)} seconds(s)')
        ```
        
          
          
        **Multiprocessing (New easier way)**
        
        Process pool executor.
        
        Using the context manager, automatically joins the processes.
        
        ```Python
        import time
        import concurrent.futures
        start = time.perf_counter()
        
        def do_something(seconds):
        	print(f'Sleeping {seconds} second(s)')
        	time.sleep(seconds)
        	return 'Done sleeping'
        
        with concurrent.futures.ProcessPoolExecutor() as executor:
        	# results = [executor.submit(do_something, i) for i in range(10)]
        	secs = [5, 4, 3, 2, 1]
        	# Map returns the results as an array
        	results = executor.map(do_something, secs)
        	
        	for res in results:
        		print(res)
        
        finish = time.perf_counter()
        
        print(f'Finished in {round(finish-start, r)} seconds(s)')
        ```
        
          
        
        > [!important]  
        > Use threads for IO bounds tasks and processes for CPU bound tasks.  
        
          
        
        - Sharing data between processes
            
            Whenever a new process is created, the process gets its own new address space. Address space is basically the area where the process will store all it's variables.
            
              
            
            So generally global variables don't really work. Therefore, we need a way to share the data between these processes.
            
              
            
            We will be using **shared memory**.
            
              
            
            There are two ways to use shared memory
            
            1. Value  
            2. Array  
            
            Shared memory has different set of methods.
            
            ```Python
            import multiprocessing
            
            result = []
            
            def calc_square(numbers, result):
            	for idx, n in enumerate(numbers):
            		result[idx] = n * n
            
            
            if __name__ == "__main__":
            	
            	numbers = [2, 3, 5]
            	result = multiprocessing.Array('i', 3)
            	p = multiprocessing.Process(target=calc_square,
            	args=(numbers, result))
            
            	p.start()
            	p.join()
            
            	print('outside process ' + str(result[:]))
            	
            ```
            
              
            
            For value
            
            ```Python
            import multiprocessing
            
            result = []
            
            def calc_square(numbers, result, value):
            	v.value = 5.67
            	for idx, n in enumerate(numbers):
            		result[idx] = n * n
            
            
            if __name__ == "__main__":
            	
            	numbers = [2, 3, 5]
            	result = multiprocessing.Array('i', 3)
            	value = multiprocessing.Value('d', 0.0)
            	p = multiprocessing.Process(target=calc_square,
            	args=(numbers, result, value))
            
            	p.start()
            	p.join()
            
            	print('outside process ' + str(result[:]))
            	
            ```
            
              
            
            **Sharing data between Processes using Queue**
            
              
            
            ```Python
            import multiprocessing
            
            result = []
            
            def calc_square(numbers, queue):
            	for idx, n in enumerate(numbers):
            		queue.put(n * n)
            
            
            if __name__ == "__main__":
            	
            	numbers = [2, 3, 5]
            	queue = multiprocessing.Queue(
            	p = multiprocessing.Process(target=calc_square,
            	args=(numbers, queue))
            
            	p.start()
            	p.join()
            
            	while not q.empty():
            		print(q.get())
            	
            ```
            
              
            
            Difference between multiprocessing queue and python queue module.
            
            ![[Screenshot_2020-10-18_at_09.52.58.png]]
            
              
            
              
            
    - Multi-processing locks
        1. Resources cannot be accessed at the same time by two threads or processes so we need locks
        
    - Multiprocessing Pools (Map reduce)
    - GIL - Python
        1. Threads share the same state and process information. (Unlike multiple-processes)
        2. Also sometimes called lightweight processes
        3. If you have multiple threads and they are both trying to change the same module attribute at the same time, you would have race conditions.
        4. Python uses reference counting to manage the lifetime of objects.
        5. If 3 people hold reference to an object, the reference count for the object will be 3
        6. Reference counting with threads is problematic though and may result in an object living forever. (Results in memory leak).
        7. What we can do is to add locks.
            
            1. Add one lock per global
            2. one lock per group of globals
            3. reference count locks(s)
            
            But all these approaches may result in a deadlock.  
            Happens when you have multiple locks and people acquire them in the opposite order.  
            
        8. The Global interpreter lock -
            1. There is this one lock and you have to be holding it if you want to interact with the CPython interpreter in any way.
            2. No deadlocks possible
            3. I/O bound tasks are actually fine.
            4. However, if your tasks are CPU bound, since only one thread can hold the GIL at one time, your multi-threaded program effectively becomes single-threaded. Essentially, only one thread can run bytecode at a time.
        
    - Design Patterns
        1. Proxy pattern
            1. Problem - We have high res images on website. When customer comes, there is a long loading time.
            2. Solution - replace the images with the placeholders (proxies)
        2. How to use design patterns ?
            
            1. Create 3 updates that gives sudo-updates over iPhone users.
            2. But now we want updates for android users as well. (7 updates)
            3. And then for windows phone also we need updates.
            4. We have 3 different groups of people and each use a different language. And the user's language is also different from all.
            
            **Adapter design pattern**
            
            With the adapter design pattern, we can add an adapter class infront of the different phone classes and we do the translation in the adapter class.
            
              
            
            **Standard Design**
            
            ```Python
            class Elf:
            	def nall_nin(self):
            		print("Elf says: calling the overlord")
            
            class Dwarf:
            	def ring_ming(self):
            		print("Dwarf says: calling the overlord")
            
            class Human:
            	def calling(self):
            		print("Human says: calling the overlord")
            
            if __name__ == '__main__':
            	minions = [Elf(), Dwarf(), Human()]
            	
            	for minion in minions:
            		if isinstance(minion, Elf):
            			minion.nall_nin()
            		elif isinstance(minion, Dwarf):
            			minion.ring_ming()
            		else:
            			minion.calling()
            ```
            
            In this design, however, if we add a new minion, we need to change the main function. This becomes cumbersome.
            
              
            
            **Facade design pattern**
            
            API with interfaces.
            
            Make one call to the facade and the facade makes the calls to all the minions.  
              
              
            **Observer design pattern**
            
            With the observer pattern, we can make all our minions call back to the main observer whenever there is a state change. (No requirement for active polling)
            
              
            
    - Pass by object reference
        - Pass by reference vs Pass by value
            
            When you pass function arguments by reference, those arguments are only references to existing values. In contrast, when you pass arguments by value, those arguments become independent copies of the original values.
            
            Python passes arguments by assignment. That is, when you call a Python function, each function argument becomes a variable to which the passed value is assigned.
            
              
            
            ### **Understanding Assignment in Python**
            
            Python’s language reference for [assignment statements](https://docs.python.org/3/reference/simple_stmts.html#assignment-statements) provides the following details:
            
            - If the assignment target is an identifier, or variable name, then this name is bound to the object. For example, in `x = 2`, `x` is the name and `2` is the object.
            - If the name is already bound to a separate object, then it’s re-bound to the new object. For example, if `x` is already `2` and you issue `x = 3`, then the variable name `x` is re-bound to `3`.
            
            All [Python objects](https://realpython.com/pointers-in-python/) are implemented in a particular structure. One of the properties of this structure is a counter that keeps track of how many names have been bound to this object.
            
              
            
            Let’s stick to the `x = 2` example and examine what happens when you assign a value to a new variable:
            
            1. If an object representing the value `2` already exists, then it’s retrieved. Otherwise, it’s created.
            2. The reference counter of this object is incremented.
            3. An entry is added in the current [namespace](https://realpython.com/python-scope-legb-rule/) to bind the identifier `x` to the object representing `2`. This entry is in fact a [key-value pair stored in a dictionary](https://realpython.com/python-namespaces-scope/#python-namespace-dictionaries)! A representation of that dictionary is returned by `locals()` or `globals()`.
            
            Now here’s what happens if you reassign `x` to a different value:
            
            1. The reference counter of the object representing `2` is decremented.
            2. The reference counter of the object that represents the new value is incremented.
            3. The dictionary for the current namespace is updated to relate `x` to the object representing the new value.
            
              
            
              
            
            Python allows you to obtain the reference counts for arbitrary values with the function `sys.getrefcount()`. You can use it to illustrate how assignment increases and decreases these reference counters. Note that the interactive interpreter employs behavior that will yield different results, so you should run the following code from a file:
            
            `from sys import getrefcount print("--- Before assignment ---") print(f"References to value_1: {getrefcount('value_1')}") print(f"References to value_2: {getrefcount('value_2')}") x = "value_1" print("--- After assignment ---") print(f"References to value_1: {getrefcount('value_1')}") print(f"References to value_2: {getrefcount('value_2')}") x = "value_2" print("--- After reassignment ---") print(f"References to value_1: {getrefcount('value_1')}") print(f"References to value_2: {getrefcount('value_2')}")`
            
            This script will show the reference counts for each value prior to assignment, after assignment, and after reassignment:
            
              
            
            - `-- Before assignment --- References to value_1: 3 References to value_2: 3 --- After assignment --- References to value_1: 4 References to value_2: 3 --- After reassignment --- References to value_1: 3 References to value_2: 4`
            
            These results illustrate the relationship between identifiers (variable names) and Python objects that represent distinct values. When you assign multiple variables to the same value, Python increments the reference counter for the existing object and updates the current namespace rather than creating duplicate objects in memory.
            
              
            
            Function arguments in Python are **local variables**. What does that mean? **Local** is one of Python’s [scopes](https://realpython.com/python-scope-legb-rule/). These scopes are represented by the namespace dictionaries mentioned in the previous section. You can use `locals()` and `globals()` to retrieve the local and global namespace dictionaries, respectively.
            
            Upon execution, each function has its own local namespace:
            
            >>>`>>> def show_locals(): ... my_local = True ... print(locals()) ... >>> show_locals() {'my_local': True}`
            
            Using `locals()`, you can demonstrate that function arguments become regular variables in the function’s local namespace. Let’s add an argument, `my_arg`, to the function:
            
              
            
            `def show_locals(my_arg): ... my_local = True ... print(locals()) ... >>> show_locals("arg_value") {'my_arg': 'arg_value', 'my_local': True}`
            
            You can also use `sys.getrefcount()` to show how function arguments increment the reference counter for an object:
            
            >>>`>>> from sys import getrefcount >>> def show_refcount(my_arg): ... return getrefcount(my_arg) ... >>> getrefcount("my_value") 3 >>> show_refcount("my_value") 5`
            
            The above script outputs reference counts for `"my_value"` first outside, then inside `show_refcount()`, showing a reference count increase of not one, but two!
            
              
            
            That’s because, in addition to `show_refcount()` itself, the call to `sys.getrefcount()` inside `show_refcount()` also receives `my_arg` as an argument. This places `my_arg` in the local namespace for `sys.getrefcount()`, adding an extra reference to `"my_value"`.
            
            By examining namespaces and reference counts inside functions, you can see that function argument work exactly like assignments: Python creates bindings in the function’s local namespace between identifiers and Python objects that represent argument values. Each of these bindings increments the object’s reference counter.
            
              
            
    - Object oriented programming
        - Inheritance
        - Multiple inheritance
        - Generators
    - Operator and Function overloading in Custom Python classes
        
    - Pytest
        - Introduction
        - Selectively run tests
        - Fixtures
        - Parameters
    - Python generators
        
        **What are generators ?**  
        Generator functions are a special kind of function that return a lazy iterator. These are objects that you can loop over like a list. However, unlike lists, lazy iterators do not store their contents in memory.  
        
        > [!important]  
        > “lazy” means that when you create an iterator, it doesn’t generate all the items it can yield just then. It waits until you ask for them with next(). Items are not created until they are requested.  
        
          
          
        **Example 1: Reading Large Files  
          
        **A common use case of generators is to work with data streams or large files, like CSV files. These text files separate data into columns by using commas. This format is a common way to share data. Now, what if you want to count the number of rows in a CSV file?
        
        ```Python
        csv_gen = csv_reader("some_csv.txt")
        row_count = 0
        
        for row in csv_gen:
            row_count += 1
        
        print(f"Row count is {row_count}")
        ```
        
          
        
        Looking at this example, you might expect `csv_gen` to be a list. To populate this list, `csv_reader()` opens a file and loads its contents into `csv_gen`. Then, the program iterates over the list and increments `row_count` for each row.
        
        This is a reasonable explanation, but would this design still work if the file is very large? What if the file is larger than the memory you have available? To answer this question, let’s assume that `csv_reader()` just opens the file and reads it into an array:  
          
        
        ```Python
        def csv_reader(file_name):
            file = open(file_name)
            result = file.read().split("\n")
            return result
        ```
        
        This function opens a given file and uses file.read() along with .split() to add each line as a separate element to a list. If you were to use this version of csv_reader() in the row counting code block you saw further up, then you’d get the following output:  
          
        
        ```Python
        Traceback (most recent call last):
          File "ex1_naive.py", line 22, in <module>
            main()
          File "ex1_naive.py", line 13, in main
            csv_gen = csv_reader("file.txt")
          File "ex1_naive.py", line 6, in csv_reader
            result = file.read().split("\n")
        MemoryError
        ```
        
          
        
        In this case, `open()` returns a generator object that you can lazily iterate through line by line. However, `file.read().split()` loads everything into memory at once, causing the `MemoryError`.
        
        Before that happens, you’ll probably notice your computer slow to a crawl. You might even need to kill the program with a `KeyboardInterrupt`. So, how can you handle these huge data files? Take a look at a new definition of `csv_reader()`:
        
        ```Python
        def csv_reader(file_name):
            for row in open(file_name, "r"):
                yield row
        ```
        
        In this version, you open the file, iterate through it, and yield a row and results in no memory errors.  
          
          
        **What’s happening here?**  
        Well, you’ve essentially turned csv_reader() into a generator function. This version opens a file, loops through each line, and yields each row, instead of returning it.  
          
          
        **Generator Comprehension**  
        (Very similar to a list comprehension)  
        
        ```Python
        csv_gen = (row for row in open(file_name))
        ```
        
          
          
        **Difference between yield and return**
        
        - Using `yield` will result in a generator object.
        - Using `return` will result in the first line of the file _only_.
        
          
        
        **Example 2: Getting an infinite sequence  
          
        **To get a finite sequence, we can simply call range() and evaluate it in a list context.  
          
        Generating an infinite sequence, however, will require the use of a generator, since your computer memory is finite:  
          
        
        ```Python
        def infinite_sequence():
            num = 0
            while True:
                yield num
                num += 1
        ```
        
        Instead of using a for loop, you can also call next() on the generator object directly. This is especially useful for testing a generator in the console:  
          
        
        ```Python
        >>> gen = infinite_sequence()
        >>> next(gen)
        0
        >>> next(gen)
        1
        >>> next(gen)
        2
        >>> next(gen)
        3
        ```
        
        > [!important]  
        > In practice, you’re unlikely to write your own infinite sequence generator. The itertools module provides a very efficient infinite sequence generator with itertools.count().  
        
          
          
        **Understanding Generators**  
        Generator functions look and act just like regular functions, but with one defining characteristic. Generator functions use the Python yield keyword instead of return.  
          
        
        This looks like a typical [function definition](https://realpython.com/defining-your-own-python-function/), except for the Python yield statement and the code that follows it. `yield` indicates where a value is sent back to the caller, but unlike `return`, you don’t exit the function afterward.
        
        Instead, the **state** of the function is remembered. That way, when `next()` is called on a generator object (either explicitly or implicitly within a `for` loop), the previously yielded variable `num` is incremented, and then yielded again. Since generator functions look like other functions and act very similarly to them, you can assume that generator expressions are very similar to other comprehensions available in Python.  
          
          
        **Building generators with Generators Expressions**  
        Like list comprehensions, generator expressions allow you to quickly create a generator object in just a few lines of code. They’re also useful in the same cases where list comprehensions are used, with an added benefit: you can create them without building and holding the entire object in memory before iteration. In other words, you’ll have no memory penalty when you use generator expressions. Take this example of squaring some numbers:  
          
        
        ```Python
        num_squared_lc = [num**2 for num in range(10)]
        num_squared_gc = (num**2 for num in range(3, 10))
        
        
        if __name__ == '__main__':
            print(num_squared_lc)
            print(num_squared_gc)
            print(next(num_squared_gc))
            print(next(num_squared_gc))
        ```
        
          
          
        **Profiling Generator Performance**  
        Lets look at the memory benefits we get when using generators.  
        We will inspect the size of the resulting objects using sys.getsizeof() :  
          
        
        ```Python
        >>> import sys
        >>> nums_squared_lc = [i * 2 for i in range(10000)]
        >>> sys.getsizeof(nums_squared_lc)
        87624
        >>> nums_squared_gc = (i ** 2 for i in range(10000))
        >>> print(sys.getsizeof(nums_squared_gc))
        120
        ```
        
        There is one thing to keep in mind, though. If the list is smaller than the running machine’s available memory, then list comprehensions can be faster to evaluate than the equivalent generator expression. To explore this, let’s sum across the results from the two comprehensions above. You can generate a readout with cProfile.run():  
          
        
        ```Python
        >>> import cProfile
        >>> cProfile.run('sum([i * 2 for i in range(10000)])')
                 5 function calls in 0.001 seconds
        
           Ordered by: standard name
        
           ncalls  tottime  percall  cumtime  percall filename:lineno(function)
                1    0.001    0.001    0.001    0.001 <string>:1(<listcomp>)
                1    0.000    0.000    0.001    0.001 <string>:1(<module>)
                1    0.000    0.000    0.001    0.001 {built-in method builtins.exec}
                1    0.000    0.000    0.000    0.000 {built-in method builtins.sum}
                1    0.000    0.000    0.000    0.000 {method 'disable' of '_lsprof.Profiler' objects}
        
        
        >>> cProfile.run('sum((i * 2 for i in range(10000)))')
                 10005 function calls in 0.003 seconds
        
           Ordered by: standard name
        
           ncalls  tottime  percall  cumtime  percall filename:lineno(function)
            10001    0.002    0.000    0.002    0.000 <string>:1(<genexpr>)
                1    0.000    0.000    0.003    0.003 <string>:1(<module>)
                1    0.000    0.000    0.003    0.003 {built-in method builtins.exec}
                1    0.001    0.001    0.003    0.003 {built-in method builtins.sum}
                1    0.000    0.000    0.000    0.000 {method 'disable' of '_lsprof.Profiler' objects}
        ```
        
          
        Here, you can see that summing across all values in the list comprehension took about a third of the time as summing across the generator. If speed is an issue and memory isn’t, then a list comprehension is likely a better tool for the job.  
          
          
        **Understanding the python yield statement  
          
        **
        
        When you call a generator function or use a generator expression, you return a special iterator called a generator. You can assign this generator to a variable in order to use it. When you call special methods on the generator, such as `next()`, the code within the function is executed up to `yield`.
        
        When the Python yield statement is hit, the program suspends function execution and returns the yielded value to the caller. (In contrast, `return` stops function execution completely.) When a function is suspended, the state of that function is saved. This includes any variable bindings local to the generator, the instruction pointer, the internal stack, and any exception handling.
        
        This allows you to resume function execution whenever you call one of the generator’s methods.  
          
        In this way, all function evaluation picks back up right after yield.  
          
        
        ```Python
        >>> def multi_yield():
        ...     yield_str = "This will print the first string"
        ...     yield yield_str
        ...     yield_str = "This will print the second string"
        ...     yield yield_str
        ...
        >>> multi_obj = multi_yield()
        >>> print(next(multi_obj))
        This will print the first string
        >>> print(next(multi_obj))
        This will print the second string
        >>> print(next(multi_obj))
        Traceback (most recent call last):
          File "<stdin>", line 1, in <module>
        StopIteration
        ```
        
          
          
        
        > [!important]  
        > StopIteration is a natural exception that’s raised to signal the end of an iterator. for loops, for example, are built around StopIteration. You can even implement your own for loop by using a while loop:  
        
        ```Python
        >>> letters = ["a", "b", "c", "y"]
        >>> it = iter(letters)
        >>> while True:
        ...     try:
        ...         letter = next(it)
        ...     except StopIteration:
        ...         break
        ...     print(letter)
        ...
        a
        b
        c
        y
        ```
        
          
          
        **Using advanced generator methods**  
          
        
        In addition to `yield`, generator objects can make use of the following methods:
        
        - `.send()`
        - `.throw()`
        - `.close()`
        
          
        
        **send() method**  
          
        
          
        
    - Dataclasses
        
          
        
    - Itertools
        1. **What is itertools and why should you use it?**  
            The functions in itertools “operate” on iterators to produce more complex iterators. Consider, for example, the built-in zip() function, which takes any number of iterables as arguments and returns an iterator over tuples of their corresponding elements:  
              
            
            ```Python
            list(zip([1, 2, 3], ['a', 'b', 'c']))
            [(1, 'a'), (2, 'b'), (3, 'c')]
            ```
            
            How, exactly, does `zip()` work?
            
            `[1, 2, 3]` and `['a', 'b', 'c']`, like all lists, are iterable, which means they can return their elements one at a time.
            
            > [!important]  
            > Technically, any Python object that implements the .__iter__() or .__getitem__() methods is iterable.  
            
            The iter() built-in function, when called on an iterable, returns an iterator object for that iterable:  
              
            
            ```Python
            >>> iter([1, 2, 3, 4])
            <list_iterator object at 0x7fa80af0d898>
            ```
            
            You can call next(iter_object) to iterate through the iter object.  
              
              
            **How does zip work under the hood ?**  
            Under the hood, the zip() function works, in essence, by calling iter() on each of its arguments, then advancing each iterator returned by iter() with next() and aggregating the results into tuples. The iterator returned by zip() iterates over these tuples.  
              
            What does the map() built-in function do ?  
              
            
    - Functools
    - Collections
    - Evaluate Expressions dynamically with eval()
    - Python Inner Functions
    - Python args and kwargs
        1. Passing multiple arguments to a function  
            args and **kwargs allow you to pass multiple arguments or keyword arguments to a function. Consider the following example. This is a simple function that takes two arguments and returns their sum:  
              
            
            ```Python
            def sum_two_numbers(a, b):
                return a + b
            ```
            
            This function works fine, but it’s limited to only two arguments. What if you need to sum a varying number of arguments, where the specific number of arguments passed is only determined at runtime? Wouldn’t it be great to create a function that could sum all the integers passed to it, no matter how many there are?
            
        2. **Using the Python args Variable in Function definitions -  
              
            **Pass a varying number of positional arguments.
            
            ```Python
            # sum_integers_args.py
            def my_sum(*args):
                result = 0
                # Iterating over the Python args tuple
                for x in args:
                    result += x
                return result
            
            def sum_n_numbers(*args):
                return sum(args)
            
            print(my_sum(1, 2, 3))
            ```
            
            > [!important]  
            > Note that args is just a name. You’re not required to use the name args. You can choose any name that you prefer, such as integers.  
            
            ```Python
            # sum_integers_args_2.py
            def my_sum(*integers):
                result = 0
                for x in integers:
                    result += x
                return result
            
            print(my_sum(1, 2, 3))
            ```
            
            > [!important]  
            > The function still works, even if you pass the iterable object as integers instead of args. All that matters here is that you use the unpacking operator (*).Bear in mind that the iterable object you’ll get using the unpacking operator * is not a list but a tuple. A tuple is similar to a list in that they both support slicing and iteration. However, tuples are very different in at least one aspect: lists are mutable, while tuples are not.  
            
              
            
        3. **Using kwargs  
              
            ***kwargs works just like *args, but instead of accepting positional arguments it accepts keyword (or named) arguments.  
              
            
            ```Python
            def concatenate(**kwargs):
                result = ""
                # Iterating over the Python kwargs dictionary
                for arg in kwargs.values():
                    result += arg
                return result
            
            print(concatenate(a="Real", b="Python", c="Is", d="Great", e="!"))
            ```
            
        4. **Ordering arguments in a function**
            
            But what if you want to create a function that takes a changeable number of both positional _and_ named arguments?
            
            In this case, you have to bear in mind that **order counts**. Just as non-default arguments have to precede default arguments, so `*args` must come before `**kwargs`.
            
            To recap, the correct order for your parameters is:
            
            1. Standard arguments
            2. `args` arguments
            3. `*kwargs` arguments
            
              
            
        5. **Unpacking with Asterisk operators: * and ****  
            The single and double asterisk unpacking operators were introduced in Python 2. As of the 3.5 release, they have become even more powerful, thanks to PEP 448. In short, the unpacking operators are operators that unpack the values from iterable objects in Python. The single asterisk operator * can be used on any iterable that Python provides, while the double asterisk operator ** can only be used on dictionaries.  
              
            
            ```Python
            # print_unpacked_list.py
            my_list = [1, 2, 3]
            print(*my_list)
            
            
            # Here, my_sum() explicitly states that a, b, and c are required arguments.
            def my_sum(a, b, c):
                print(a + b + c)
            
            my_list = [1, 2, 3]
            my_sum(*my_list)
            ```
            
            When you use the * operator to unpack a list and pass arguments to a function, it’s exactly as though you’re passing every single argument alone. This means that you can use multiple unpacking operators to get values from several lists and pass them all to a single function.
            
            ```Python
            # sum_integers_args_3.py
            def my_sum(*args):
                result = 0
                for x in args:
                    result += x
                return result
            
            list1 = [1, 2, 3]
            list2 = [4, 5]
            list3 = [6, 7, 8, 9]
            
            print(my_sum(*list1, *list2, *list3))
            ```
            
            There are other convenient uses of the unpacking operator. For example, say you need to split a list into three different parts. The output should show the first value, the last value, and all the values in between. With the unpacking operator, you can do this in just one line of code:
            
            ```Python
            # extract_list_body.py
            my_list = [1, 2, 3, 4, 5, 6]
            
            a, *b, c = my_list
            
            print(a)
            print(b)
            print(c)
            ```
            
            In this example, my_list contains 6 items. The first variable is assigned to a, the last to c, and all other values are packed into a new list b.
            
              
            
            Another interesting thing you can do with the unpacking operator * is to split the items of any iterable object. This could be very useful if you need to merge two lists, for instance:  
              
            
            ```Python
            # merging_lists.py
            my_first_list = [1, 2, 3]
            my_second_list = [4, 5, 6]
            my_merged_list = [*my_first_list, *my_second_list]
            
            print(my_merged_list)
            ```
            
            The unpacking operator * is prepended to both my_first_list and my_second_list.
            
              
            
            You can even merge two different dictionaries by using the unpacking operator **  
              
            
            ```Python
            # merging_dicts.py
            my_first_dict = {"A": 1, "B": 2}
            my_second_dict = {"C": 3, "D": 4}
            my_merged_dict = {**my_first_dict, **my_second_dict}
            
            print(my_merged_dict)
            ```
            
              
            
            Remember that the * operator works on any iterable object. It can also be used to unpack a string:  
              
            
            ```Python
            a = [*"RealPython"]
            print(a)
            ```
            
            In Python, strings are iterable objects, so * will unpack it and place all individual values in a list a
            
    - Decorators
        1. A decorator is a function that takes  another function and extends the behavior of the latter function without explicitly modifying it.
        2. In Python, functions are first-class objects. This means that functions can be passed around and used as arguments, just like any other object (string, int, float, list, and so on).
        3. Example of functions being a first class object in Python
            
            ```Python
            def say_hello(name):
                return f"Hello {name}"
            
            def be_awesome(name):
                return f"Yo {name}, together we are the awesomest!"
            
            def greet_bob(greeter_func):
                return greeter_func("Bob")
            ```
            
            greet_bob function takes the greeter_func function as an argument.
            
            ```Python
            >>> greet_bob(say_hello)
            'Hello Bob'
            
            >>> greet_bob(be_awesome)
            'Yo Bob, together we are the awesomest!'
            ```
            
            The say_hello function is named without parentheses. This means that only a reference to the function is passed. The function is not executed. The greet_bob() function, on the other hand, is written with parentheses, so it will be called as usual.
            
              
            
        4. **Inner Functions  
              
            **It’s possible to define functions inside other functions. Such functions are called inner functions.
            
            ```Python
            def parent():
                print("Printing from the parent() function")
            
                def first_child():
                    print("Printing from the first_child() function")
            
                def second_child():
                    print("Printing from the second_child() function")
            
                second_child()
                first_child()
            ```
            
            Furthermore, the inner functions are not defined until the parent function is called. They are locally scoped to parent(): they only exist inside the parent() function as local variables. Try calling first_child(). You should get an error:
            
            ```Python
            Traceback (most recent call last):
              File "<stdin>", line 1, in <module>
            NameError: name 'first_child' is not defined
            ```
            
              
            
        5. **Returning Functions from Functions  
              
            **Python also allows you to use functions as return values.  
              
            
            ```Python
            def parent(num):
                def first_child():
                    return "Hi, I am Emma"
            
                def second_child():
                    return "Call me Liam"
            
                if num == 1:
                    return first_child
                else:
                    return second_child
            ```
            
            Note that you are returning first_child without the parentheses. Recall that this means that you are returning a reference to the function first_child. In contrast first_child() with parentheses refers to the result of evaluating the function. This can be seen in the following example:  
              
            
            ```Python
            >>> first = parent(1)
            >>> second = parent(2)
            
            >>> first
            <function parent.<locals>.first_child at 0x7f599f1e2e18>
            
            >>> second
            <function parent.<locals>.second_child at 0x7f599dad5268>
            ```
            
            You can now use first and second as if they are regular functions, even though the functions they point to can’t be accessed directly:  
              
            
            ```Python
            >>> first()
            'Hi, I am Emma'
            
            >>> second()
            'Call me Liam'
            ```
            
            Finally, note that in the earlier example you executed the inner functions within the parent function, for instance first_child(). However, in this last example, you did not add parentheses to the inner functions—first_child—upon returning. That way, you got a reference to each function that you could call in the future.
            
        6. Simple Decorators:  
            Example of a simple decorator:  
            
            ```Python
            def my_decorator(func):
                def wrapper():
                    print("Something is happening before the function is called.")
                    func()
                    print("Something is happening after the function is called.")
                return wrapper
            
            def say_whee():
                print("Whee!")
            
            say_whee = my_decorator(say_whee)
            ```
            
            What happens when you call say_whee() ?  
              
            
            ```Python
            >>> say_whee()
            Something is happening before the function is called.
            Whee!
            Something is happening after the function is called.
            ```
            
            The so-called decoration happens at the following line:
            
            ```Python
            say_whee = my_decorator(say_whee)
            ```
            
            In effect, the name say_whee now points to the wrapper() inner function. Remember that you return wrapper as a function when you call my_decorator(say_whee):
            
            ```Python
            >>> say_whee
            <function my_decorator.<locals>.wrapper at 0x7f3c5dfd42f0>
            ```
            
            However, `wrapper()` has a reference to the original `say_whee()` as `func`, and calls that function between the two calls to [`print()`](https://realpython.com/courses/python-print/).
            
            Put simply: **decorators wrap a function, modifying its behavior.**
            
            Example of a very simple decorator -
            
            ```Python
            def my_decorator(func):
                def wrapper():
                    print("What is your name ?")
                    func()
                    print("F*** you ezekeil")
            
                return wrapper
            
            @my_decorator
            def say_my_name():
                print("Ezekeil")
            
            if __name__ == "__main__":
                say_my_name()
            ```
            
              
            
        7. Dynamic decorator behaviour changes  
            The way a decorator modifies a function can change dynamically.  
              
            
            ```Python
            from datetime import datetime
            
            def dynamic_decorator(func):
                def wrapper():
                    if 7 <= datetime.now().hour < 22:
                        func()
                    else:
                        pass
                
                return wrapper
            
            @dynamic_decorator
            def say_whee():
                print("Wehhhee")
            ```
            
              
            
        8. **Returning values from decorated functions  
              
            **To fix this, you need to make sure the wrapper function returns the return value of the decorated function. Change your [decorators.py](http://decorators.py/) file:  
              
            
            ```Python
            def do_twice(func):
                def wrapper_do_twice(*args, **kwargs):
                    func(*args, **kwargs)
                    return func(*args, **kwargs)
                return wrapper_do_twice
            ```
            
              
            
        9. **Python introspection**  
            However, after being decorated, say_whee() has gotten very confused about its identity. It now reports being the wrapper_do_twice() inner function inside the do_twice() decorator. Although technically true, this is not very useful information.  
              
            To fix this, decorators should use the @functools.wraps decorator, which will preserve information about the original function.  
              
            
            ```Python
            import functools
            
            def do_twice(func):
                @functools.wraps(func)
                def wrapper_do_twice(*args, **kwargs):
                    func(*args, **kwargs)
                    return func(*args, **kwargs)
                return wrapper_do_twice
            ```
            
            > [!important]  
            > The @functools.wraps decorator uses the function functools.update_wrapper() to update special attributes like __name__ and __doc__ that are used in the introspection.  
            
              
            
        10. **Examples**
            1. Debugging Code  
                  
                
                ```Python
                import functools
                
                def debug(func):
                    """Print the function signature and return value"""
                    @functools.wraps(func)
                    def wrapper_debug(*args, **kwargs):
                        args_repr = [repr(a) for a in args]                      # 1
                        kwargs_repr = [f"{k}={v!r}" for k, v in kwargs.items()]  # 2
                        signature = ", ".join(args_repr + kwargs_repr)           # 3
                        print(f"Calling {func.__name__}({signature})")
                        value = func(*args, **kwargs)
                        print(f"{func.__name__!r} returned {value!r}")           # 4
                        return value
                    return wrapper_debug
                ```
                
                  
                
        11. **Fancy Decorators**  
              
              
            Decorating classes  
              
            Some commonly used decorators that are even built-ins in Python are @classmethod, @staticmethod, and @property.  
              
            The @classmethod and @staticmethod decorators are used to define methods inside a class namespace that are not connected to a particular instance of that class. The @property decorator is used to customize getters and setters for class attributes.  
              
            
            ```Python
            class Circle:
                def __init__(self, radius):
                    self._radius = radius
            
                @property
                def radius(self):
                    """Get value of radius"""
                    return self._radius
            
                @radius.setter
                def radius(self, value):
                    """Set radius, raise error if negative"""
                    if value >= 0:
                        self._radius = value
                    else:
                        raise ValueError("Radius must be positive")
            
            		@property
                def area(self):
                    """Calculate area inside circle"""
                    return self.pi() * self.radius**2
            
                def cylinder_volume(self, height):
                    """Calculate volume of cylinder with circle as base"""
                    return self.area * height
            
                @classmethod
                def unit_circle(cls):
                    """Factory method creating a circle with radius 1"""
                    return cls(1)
            
                @staticmethod
                def pi():
                    """Value of π, could use math.pi instead though"""
                    return 3.1415926535
            ```
            
              
            
            In this class:
            
            - `.cylinder_volume()` is a regular method.
            - `.radius` is a mutable property: it can be set to a different value. However, by defining a setter method, we can do some error testing to make sure it’s not set to a nonsensical negative number. Properties are accessed as attributes without parentheses.
            - `.area` is an immutable property: properties without `.setter()` methods can’t be changed. Even though it is defined as a method, it can be retrieved as an attribute without parentheses.
            - `.unit_circle()` is a class method. It’s not bound to one particular instance of `Circle`. Class methods are often used as factory methods that can create specific instances of the class.
            - `.pi()` is a static method. It’s not really dependent on the `Circle` class, except that it is part of its namespace. Static methods can be called on either an instance or the class.
            
            The other way to use decorators on classes is to decorate the whole class. This is, for example, done in the new dataclasses module in Python 3.7:  
              
            
            ```Python
            from dataclasses import dataclass
            
            @dataclass
            class DataClassCard:
                rank: str
                suit: str
            ```
            
            A data class comes with basic functionality already implemented. For instance, you can instantiate, print, and compare data class instances straight out of the box:  
              
            
            ```Python
            >>> queen_of_hearts = DataClassCard('Q', 'Hearts')
            >>> queen_of_hearts.rank
            'Q'
            >>> queen_of_hearts
            DataClassCard(rank='Q', suit='Hearts')
            >>> queen_of_hearts == DataClassCard('Q', 'Hearts')
            True
            ```
            
            Compare that to a regular class. A minimal regular class would look something like this:
            
            ```Python
            class RegularCard
                def __init__(self, rank, suit):
                    self.rank = rank
                    self.suit = suit
            ```
            
            While this is not much more code to write, you can already see signs of the boilerplate pain: rank and suit are both repeated three times simply to initialize an object. Furthermore, if you try to use this plain class, you’ll notice that the representation of the objects is not very descriptive, and for some reason a queen of hearts is not the same as a queen of hearts:  
              
            
            ```Python
            >>> queen_of_hearts = RegularCard('Q', 'Hearts')
            >>> queen_of_hearts.rank
            'Q'
            >>> queen_of_hearts
            <__main__.RegularCard object at 0x7fb6eee35d30>
            >>> queen_of_hearts == RegularCard('Q', 'Hearts')
            False
            ```
            
            Seems like data classes are helping us out behind the scenes. By default, data classes implement a .**repr**() method to provide a nice string representation and an .**eq**() method that can do basic object comparisons. For the RegularCard class to imitate the data class above, you need to add these methods as well:  
              
            
            ```Python
            class RegularCard
                def __init__(self, rank, suit):
                    self.rank = rank
                    self.suit = suit
            
                def __repr__(self):
                    return (f'{self.__class__.__name__}'
                            f'(rank={self.rank!r}, suit={self.suit!r})')
            
                def __eq__(self, other):
                    if other.__class__ is not self.__class__:
                        return NotImplemented
                    return (self.rank, self.suit) == (other.rank, other.suit)
            ```
            
              
            
        12. **Nesting Decorators**  
            You can apply several decorators to a function by stacking them on top of each other  
              
            
            ```Python
            from decorators import debug, do_twice
            
            @debug
            @do_twice
            def greet(name):
                print(f"Hello {name}")
            ```
            
            Think about this as the decorators being executed in the order they are listed. In other words, @debug calls @do_twice, which calls greet(), or debug(do_twice(greet()))
            
              
            
        13. **Decorators with arguments**
            
            Sometimes, it’s useful to **pass arguments to your decorators**. For instance, `@do_twice` could be extended to a `@repeat(num_times)` decorator. The number of times to execute the decorated function could then be given as an argument.
            
            This would allow you to do something like this:
            
            ```Python
            @repeat(num_times=4)
            def greet(name):
                print(f"Hello {name}")
            
            def repeat(num_times):
                def decorator_repeat(func):
                    @functools.wraps(func)
                    def wrapper_repeat(*args):
                        value = None
                        for _ in range(num_times):
                            value = func(*args)
                        return value
            
                    return wrapper_repeat
                return decorator_repeat
            ```
            
              
            
            There are a few subtle things happening in the `repeat()` function:
            
            - Defining `decorator_repeat()` as an inner function means that `repeat()` will refer to a function object—`decorator_repeat`. Earlier, we used `repeat` without parentheses to refer to the function object. The added parentheses are necessary when defining decorators that take arguments.
            - The `num_times` argument is seemingly not used in `repeat()` itself. But by passing `num_times` a [closure](https://realpython.com/inner-functions-what-are-they-good-for/) is created where the value of `num_times` is stored until it will be used later by `wrapper_repeat()`.
            
              
            
        14. **Stateful Decorators**  
            Sometimes, it’s useful to have a decorator that can keep track of state. As a simple example, we will create a decorator that counts the number of times a function is called.  
              
            
            ```Python
            import functools
            
            def count_calls(func):
                @functools.wraps(func)
                def wrapper_count_calls(*args, **kwargs):
                    wrapper_count_calls.num_calls += 1
                    print(f"Call {wrapper_count_calls.num_calls} of {func.__name__!r}")
                    return func(*args, **kwargs)
                wrapper_count_calls.num_calls = 0
                return wrapper_count_calls
            
            @count_calls
            def say_whee():
                print("Whee!")
            ```
            
            The state—the number of calls to the function—is stored in the function attribute .num_calls on the wrapper function. Here is the effect of using it:  
              
            
            ```Python
            >>> say_whee()
            Call 1 of 'say_whee'
            Whee!
            
            >>> say_whee()
            Call 2 of 'say_whee'
            Whee!
            
            >>> say_whee.num_calls
            2
            ```
            
              
            
        15. **Classes as Decorators  
              
            **
            
            Recall that the decorator syntax `@my_decorator` is just an easier way of saying `func = my_decorator(func)`. Therefore, if `my_decorator` is a class, it needs to take `func` as an argument in its `.__init__()` method. Furthermore, the class instance needs to be [callable](https://docs.python.org/reference/datamodel.html#emulating-callable-objects) so that it can stand in for the decorated function.
            
            For a class instance to be callable, you implement the special `.__call__()` method:  
              
            
            ```Python
            class Counter:
                def __init__(self, start=0):
                    self.count = start
            
                def __call__(self):
                    self.count += 1
                    print(f"Current count is {self.count}")
            ```
            
            The .**call**() method is executed each time you try to call an instance of the class:  
              
            
            ```Python
            >>> counter = Counter()
            >>> counter()
            Current count is 1
            
            >>> counter()
            Current count is 2
            
            >>> counter.count
            2
            ```
            
            Therefore, a typical implementation of a decorator class needs to implement .**init**() and .**call**():  
              
            
            ```Python
            import functools
            
            class CountCalls:
                def __init__(self, func):
                    functools.update_wrapper(self, func)
                    self.func = func
                    self.num_calls = 0
            
                def __call__(self, *args, **kwargs):
                    self.num_calls += 1
                    print(f"Call {self.num_calls} of {self.func.__name__!r}")
                    return self.func(*args, **kwargs)
            
            @CountCalls
            def say_whee():
                print("Whee!")
            ```
            
            > [!important]  
            > The .init() method must store a reference to the function and can do any other necessary initialization. The .call() method will be called instead of the decorated function. It does essentially the same thing as the wrapper() function in our earlier examples. Note that you need to use the functools.update_wrapper() function instead of @functools.wraps.  
            
              
            
        16. **Creating Singletons**  
              
            A singleton is a class with only one instance. There are several singletons in Python that you use frequently, including None, True, and False.  
            **It is the fact that None is a singleton that allows you to compare for None using the is keyword.  
              
              
            **Using is returns True only for objects that are the exact same instance. The following @singleton decorator turns a class into a singleton by storing the first instance of the class as an attribute. Later attempts at creating an instance simply return the stored instance:
            
            ```Python
            import functools
            
            def singleton(cls):
                """Make a class a Singleton class (only one instance)"""
                @functools.wraps(cls)
                def wrapper_singleton(*args, **kwargs):
                    if not wrapper_singleton.instance:
                        wrapper_singleton.instance = cls(*args, **kwargs)
                    return wrapper_singleton.instance
                wrapper_singleton.instance = None
                return wrapper_singleton
            
            @singleton
            class TheOne:
                pass
            ```
            
            As you see, this class decorator follows the same template as our function decorators. The only difference is that we are using cls instead of func as the parameter name to indicate that it is meant to be a class decorator.  
              
            
        17. **Caching return values  
              
            **
            
            ```Python
            import functools
            from decorators import count_calls
            
            def cache(func):
                """Keep a cache of previous function calls"""
                @functools.wraps(func)
                def wrapper_cache(*args, **kwargs):
                    cache_key = args + tuple(kwargs.items())
                    if cache_key not in wrapper_cache.cache:
                        wrapper_cache.cache[cache_key] = func(*args, **kwargs)
                    return wrapper_cache.cache[cache_key]
                wrapper_cache.cache = dict()
                return wrapper_cache
            
            @cache
            @count_calls
            def fibonacci(num):
                if num < 2:
                    return num
                return fibonacci(num - 1) + fibonacci(num - 2)
            ```
            
              
            
            In the standard library, a [Least Recently Used (LRU) cache](https://realpython.com/lru-cache-python/) is available as [`@functools.lru_cache`](https://docs.python.org/library/functools.html#functools.lru_cache).
            
            This decorator has more features than the one you saw above. You should use `@functools.lru_cache` instead of writing your own cache decorator:  
              
            
            ```Python
            import functools
            
            @functools.lru_cache(maxsize=4)
            def fibonacci(num):
                print(f"Calculating fibonacci({num})")
                if num < 2:
                    return num
                return fibonacci(num - 1) + fibonacci(num - 2)
            ```
            
              
            
            The `maxsize` parameter specifies how many recent calls are cached. The default value is 128, but you can specify `maxsize=None` to cache all function calls. However, be aware that this can cause memory problems if you are caching many large objects.
            
            You can use the `.cache_info()` method to see how the cache performs, and you can tune it if needed. In our example, we used an artificially small `maxsize` to see the effect of elements being removed from the cache:  
              
              
            
    - Typing
    - Data types
    - Classes
    - CPython
    - Python AST
    - Python Extensions
    
- Research Papers
    - A Practical Wait-free simulation for Lock-free data structures
        - Lock-based concurrency
            1. The weakest form of concurrent data structures
            2. Eg. A unordered_map behind a mutex.
            3. Variation is fine-grained locking where the locking is not over the entire data structure but to parts of it. Eg- Every bucket (which is a linked list) in the hashmap is protected by locks.
            4. They are by nature blocking - If one thread is accessing the data structure, other threads (that may require access) are essentially blocked.
            5. This means that if a thread panics or crashes and does not release the lock, no other thread can make progress.
            6. Algorithms using locks are generally simpler.
        - Non-blocking concurrency (You need atomics for anything that does not use locks)
            1. Includes things that are lock-free, wait-free and obstruction-free.
            2. The weakest one is obstruction-freedom - Means that no thread can prevent other threads from making progress. If one thread gets stuck, it does not prevent other threads from making progress.
            3. One eg - We could have a rule on a data structure that the data structure makes progress as long as every thread makes progress.
            4. Obstruction free does not guarantee progress, it just guarantees that no thread can prevent progress.
            5. Lock-free is a slightly stronger guarantee that dictates that at any given point of time some thread can make progress. (This could end up in starvation).
            6. Wait-free is sort of the extreme case of lock-free.
            7. In a wait-free data structure, you have to ensure that every thread makes progress in a finite number of steps.
            8. So in wait-free ds have a notion of helping. If a thread cant make progress, it stores the fact that it cant make progress and other threads are required to help this thread.
            9. And so as long as any thread make progress, all threads make progress eventually.
            10. The stronger the guarantees, the more complex the data structure is likely to be and generally worse single-threaded performance (even-though you make scale better).
        - What does simulation mean?
            1. It is fairly hard to design wait-free algorithms.
            2. You take a lock free algorithm and you run it on machinery that itself is wait free and this guarantees that the overall thing you have combined is also wait-free.
        - The fast-path-slow-path method
            1. The basic idea is to have a fast path where if I am making progress without having to retry then I should just do that and because that should be the common case, we will make it as efficient as possible.
            2. Then there is a slow path where if you detect that there is contention then you take a much slower path where you carefully coordinate with the other threads.
            3. The fast-path is basically the lock-free version.
            4. The wait free part is that if it does not work, coordinate with everyone else and retry.
            5. The paper presents an automatic transformation that takes a linearizable lock-free data structure in a normalized representation and produces a practical wait-free data structure from it.
        - The basic algorithm
            
            1. The move from the lock-free implementation to the wait-free one is executed by simulating the lock-free algorithm in a wait-free manner.
            2. A normalized lock-free implementation has some mechanism for detecting failure to make progress (due to contention).
            3. When an operation fails to make progress it asks for help from the rest of the threads.
            4. A thread asks for help by enqueuing a description of it's current computation state on a wait-free queue.
            5. Each thread checks once in a while whether a help request is enqueued on the help queue
            6. Threads that notice an enqueued request for help move to helping a single operation on the top of the queue.
            7. Help includes reading the computation state of the operation to be helped and then continuing the computation from that point, until the operation completes and its result is reported.
            8. The helping threads sync during the execution of an operation at critical points, which occur just before and just after a modification of the data structure.
            9. Modifications of the shared data structure occur using a CAS (compare and swap) primitive.
            10. A helping thread runs the op it attempts to help independently until reaching a CAS instruction that modifies the shared structure.
            11. At that point, it coordinates with all helping threads which CAS should be executed.
            12. Before executing the CAS, helping threads agree on what the CAS params should be (address, expected val, new val).
            13. After the decision, helping threads attempt to exec the CAS and then they sync to ensure they all learn whether the CAS was successful.
            14. Its somewhat like execute in parallel → Agree → execute in parallel → Agree and so on....
            15. Upon completion, the op result in written into the computation state, the computation state is removed from the queue and the owner thread can return.
                
                The slow part is kind of like consensus.
                
                ![[Screenshot_2021-07-13_at_17.01.59.png]]
                
            
            - Challenges
                1. Obtaining succinct description of the computation state.
                2. Proper sync between the concurrent helping threads.
                3. Sync between the helping threads and threads executing other ops on the fast lock-free path.
        - Heuristics for detecting when is there a contention
            1. Counting the number of failed CASes can serve as a good contention failure counter.
        - CAS Description
            1. A struct that holds the triplet (address, expected, new).
            2. Contains an address on which a CAS should be executed
            3. The value we expect to find in the aforementioned address
            4. The new value we would like to atomically write to this address if the expected value is currently there.
            5. Given a pointer to the CAS description, it is possible to execute it and it could either succeed or fail.
        - Wait-free algorithm examples
        
- API design
    1. Prefer to have all parameters as optional
    2. Helps in designing for the future
    
- Golang
    1. [https://codewithyury.com/golang-pass-by-pointer-vs-pass-by-value/#:~:text=Passing%20by%20value%20in%20Go,allocating%20variable%20on%20the%20heap](https://codewithyury.com/golang-pass-by-pointer-vs-pass-by-value/#:~:text=Passing%20by%20value%20in%20Go,allocating%20variable%20on%20the%20heap). Passing by value in Go may be significantly cheaper than passing by pointer. This happens because Go uses escape analysis to determine if variable can be safely allocated on function’s stack frame, which could be much cheaper then allocating variable on the heap.
    2. The two goroutines you have create run concurrently and hence the order they push to the channel is unpredictable. So, the Go channels are FIFO, you just have an unpredictable order of who goes first and who goes last _into_ your channel.
- MOVE (Bloomberg)
    
    1. Explanation: error codes are useful to help callers to figure out _programmatically_ what happened (as opposed to a human just reading the output). This means that error codes need to have a well-defined meaning. Values like 403, 401 etc are HTTP status codes and they do have a well-defined meaning. moveaction however is not an HTTP API, so we can’t reuse as-is HTTP code meaning. This is not unique to our team, it’s a common thing with BAS/protobuf APIs. Instead, we have plans to eventually define our own error codes (that’s why we have the code field in the error types). This hasn’t been done yet though -- mainly because nobody complained yet. So [we use](https://bbgithub.dev.bloomberg.com/move/sysmvsvc/blob/8c24bb85c6215480db925be1641ab5b8e6eadb6f/src/sysmvsvc-internal/s_sysmvsvc_createjobrequesthandler.h#L17-L18) 99 as “this is a placeholder, should eventually be defined better”.
    2. Difference between TCP accept and TCP connect -  
        Either tell if a service uses a particular port (TCP Accept).  
        Dynamic TCP connection is modern and better  
        
    3. UCBR requires setup for both primary routes as well as backup routes. When developing a CLI, you shouldnt overengineer it and rely on just printing out the results to stdout.
    4. When analyzing, you can just use > operator and grep for any file based analysis as needed.
    5. Currently, we check for collisions between BPKG and BPKG List. This is not needed Package lists can be handled as an atomic unit And do the collision detection without expanding the package lists. Backwards compatibility is essential for interface design changes BPKG lists are just a list of BPKGs.
    6. DIAG DL - Check out the datalayer of a Rapid screen
    7. **Mohamed Ramadan**
        
        [10:28 AM]
        
        dpkgmovesvc accept calls from devsun in dev:
        
        {RSVC /S 274278-3 /G DEVSUN <GO>}you can see group of machines at:
        
        {RSVC /G DEVSUN <GO>} -> View groupso I think RRRR CCC 456 should work for dev
        
        **Navneet Madhu Kumar**
        
        [10:28 AM]
        
        How did you find that ?
        
        **Mohamed Ramadan**
        
        [10:28 AM]
        
        that’s what I mentioned above, the links (edited)
        
        **Navneet Madhu Kumar**
        
        [10:59 AM]
        
        Hey Mohamed, so I checked and it does hit dcot(dev). I am wondering how I can check the same for alpha
        
        [10:59 AM] As in how can I make it hit the alpha instance
        
        **Mohamed Ramadan**
        
        [11:10 AM]
        
        the same show that only alpha route to dpkgmovesvc is {RSVC /S 274278-1 /G DCOTA <GO>}, so no specific UI routes to the service, {RAPM CCC} show CCC run on USER machines so I would expect if the CCC used in alpha to have route {RSVC /S 274278-3 /G USER(SN1) <GO>} already setup which is not the case
        
        [11:11 AM] most of our system not correctly set-up on alpha so I would check if it is used in alpha by anyone because routes didn’t say so
        
    8. Regarding giving permissions for PRQS -
    
    > hmm.. i think this is only partly data driven.In code, we check for multi-stage deployments or mixed deployments (tiers and machines). In those cases we require “super approver”.In the database, prqs2db.typepvflevel, types can customize the approval rules. Typically super_approver is set only for PVF PRQS 2 or 45. However, they can customize it.In general, if users want to use non-standard PVF levels, there should be good reasoning/discussions.
    
    1. The terminal toolbar buttons I found very useful, especially during triaging:
        
        1. Command: {NXTW DRQS WS /CD /ST /TY /GR /S <paste><go>}
        2. Command: {NXTW RTCP <PASTE> <GO>}
        3. Command: {NXTW BPKG <PASTE> <GO>}
        
        Notice the `<PASTE>` in the command section, it can actually get whatever in your clipboard and you just need to do a copy on some word and click the button  to run the command without pasting! (edited)
        
    
- Miscellaneous articles
    - [http://slashslash.info/2018/02/a-foolish-consistency/](http://slashslash.info/2018/02/a-foolish-consistency/) — but I was thinking about a different article when I said “one of the best”; this one is just good 🙂
    - [**A Foolish Consistency**](http://slashslash.info/2018/02/a-foolish-consistency/)
    - [Making Wrong Code Look Wrong – Joel on Software](https://www.joelonsoftware.com/2005/05/11/making-wrong-code-look-wrong/)
    - This is a hyperlink to an article titled "Making Wrong Code Look Wrong" on the website "Joel on Software". The article discusses the importance of making code that is incorrect or problematic visually distinct from code that is correct and functional.
    - [**Working Effectively with Legacy Code**](https://learning.oreilly.com/library/view/working-effectively-with/0131177052/ch23.html)
    - [https://developers.google.com/style/timeless-documentation](https://developers.google.com/style/timeless-documentation)
    - [https://blog.codinghorror.com/the-first-rule-of-programming-its-always-your-fault/](https://blog.codinghorror.com/the-first-rule-of-programming-its-always-your-fault/)