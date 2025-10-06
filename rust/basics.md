# Rust

## introduction

- Rust is a systems programming language focused on speed, memory safety, and parallelism.  
- It is designed to be safe, concurrent, and practical.

## Basics

### Variables 

- Assigned using `let` keyword.
- Variables are immutable by default.
    - Ensures safety and easy concurrency.
- To make a variable mutable, use the `mut` keyword.
```rust
let mut x = 5; // mutable variable  
```
- If reassigning a variable, it will give compilation error like `cannot assign twice to immutable variable x`.
- A variable can be used only if it is initialized.
- If a variable is uninitialized, it will give compilation error like `use of possibly uninitialized variable: x`.
- If a variable is uninitialized but also unused, it will give only a warning like `variable x is assigned to but never used`.
    - To get rid of the warning, you can use underscore `_` before the variable name.
- Scope of a variable is the block in which it is defined.
- We can declare mulitple variables using destructuring assignment.
```rust
let (x, y) = (1, 2); // x = 1, y = 2
```

### Constants

- Values bound to a name and are not allowed to change
- You are not allowed to use `mut` keyword with constants.
- Constants are defined using `const` keyword.
- Type of the data must be annotated.
- Rust's naming convention for constants is `UPPERCASE_WITH_UNDERSCORES`.

### Constant Evaluation

- Process of computing the result of expression during compilation
- Constant expressions are always evaluated at compile time.

### Shadowing

- You can declare a new variable with the same name as previous variable.
- Its called 1st variable is shadowed by the new variable.
- Shadowing works either till the end of the scope or until the new variable is redefined.
- We can change the data type of the variable while shadowing.


## Data Types

- Rust is a statically typed language, meaning the type of a variable must be known at compile time.
- If not provided the datatype, Rust will infer it. Will display an error if it cannot infer the type.

### Scaler Types

- Scalar types represent a single value.
- Four scaler types in rust

- __Integers__ : 
    - A number without a fractional component.
    - Can be signed or unsigned.
    - Signed integers can be negative, zero, or positive and are represented by `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.
    - Unsigned integers can only be positive and are represented by `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.
    - Signed numbers are stored in two's complement form.
    - Each signed variant can store values from `-(2^(n-1))` to `2^(n-1) - 1`, where `n` is the number of bits.
    - `isize` and `usize` types depend on the architecture of the machine (32-bit or 64-bit).
    - We can assign integer datatype like this also : 
    ```rust
        let x = 5_u8; // x is of type u8
    ```

    - __Integer Overflow__ : 
        - If an integer exceeds its maximum value, for example, if `u8` exceeds 255, integer overflow occurs.
        - It can result in two behaviors : 
            - When in debug mode, the program will panic ( throw an error ) at runtime and terminate.
            - When compiling in release model ( using the `--release` flag ), Rust doesnot check for overflow that causes panic. 
            - Instead, Rust performs a two's complement wrapping, which means it wraps around to the minimum value. For example, if `u8` is 257, it wraps around to 1.
            - Program will not panic, but the value will not be expected. 
            - Relying on integer overflow's wrapping behavior is considered an error.
            - To handle the scenarios, we can use these families of methods provided by standard library : 
                - Wrap in all modes with `wrapping_*` methods.
                - Returns `None` value if there is overflow with `checked_*` methods.
                - Returns the value and a boolean indicating whether there was an overflow with `overflowing_*` methods.
                - Saturate at the value's min or max values with `saturating_*` methods.

- __Floating-Point Types__ : 
    - Rust has two floating-point types, `f32` and `f64`, which are 32-bit and 64-bit respectively.
    - Default type is `f64` because on modern CPUs, it is faster to use than `f32` and it is more precise.
    - All floating-point types are signed.
    - All floating-point types are represented in IEEE-754 format.
    - __Numeric Operations__ :
        - Integer division truncates the decimal part, while floating-point division does not.

- __Boolean Type__ :
    - Represents a value that can be either `true` or `false`.
    - The type is `bool`.
    - Size is 1 byte.

- __Character Type__ :
    - Type is `char`.
    - Specified using single quotes, e.g., `'a'`.
    - Rust's `char` is 4 bytes in size and represents a Unicode scaler value.

### Compound Types

- Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

- __Tuples__ :
    - Tuples have a fixed length, once declared, they cannot grow or shrink.
    - Can be created using comma seperated values inside parentheses.
    - Can contain values of different types.
    - Syntax : 
    ```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    ```
    - We can access the tuple elements using `tuple_name.index` syntax e.g. `x.0` gives 1st element of the tuple named x
    - Tuples with no values are called unit tuples and are represented by `()`.
    - Expressions implicitly return unit type if they do not return any other value.

- __Arrays__ :
    - Every element in an array must have the same type.
    - Arrays have a fixed length, once declared, they cannot grow or shrink.
    - Whereas vectors can grow or shrink in size.
    - Syntax : 
    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    ```
    - We can assign all elements of an array to the same value using the syntax `[value; length]`.
    ```rust
    let a = [3; 5]; // creates an array [3, 3, 3, 3, 3]
    ```
    - We can access the array elements using `array_name[index]` syntax e.g. `a[0]` gives 1st element of the array named a
    - If we try to access an index that is out of bounds, it will give a runtime error.

## Functions

- Rust uses snake case `{}` as conventional style for function and variable names, where all letters are lowercase and words are seperated by underscores.
- Function in rust is defined by `fn` keyword followed by the name of the function and a set of paranthesis.

### Parameters

- Special values that are part of the function's signature.
- We can provide conrete values to the function, called arguments.
- Type of parameters must be specified.

### Statements and Expressions

- Rust is an expression-based language, meaning that most constructs are expressions that evaluate to a value.
- __Statements__ are instructions that perform some actions and do not return a value.
    - Example : `let x = 5;` is a statement that binds the value 5 to the variable x.
- __Expressions__ evaluate to a value and can be used in statements.
- In rust, we cannot write `x = y = 6` because `y = 6` is a statement and does not return a value.
- Expressions do not end with a semicolon, while statements do.
- We can make a statement an expression by removing the semicolon at the end and vice versa. 

### Functions with Return values

- We can specify a return value for a function using the `->` syntax.
- return values are not named in rust.
- The return value is the last expression in the function body.
- If we want to return early from a function, we can use the `return` keyword.

- A diverging function is a function that does not return a value to the caller, such as `panic!` or `loop`.
- To make a function never return a value, we can use the `!` type and in the function body, we can use the `panic!` macro or an infinite loop.
- Unimplemented and todo macros also return diverging type.
```rust
fn diverging_function() -> ! {
    panic!("This function never returns");
}
```
- Diverging functions can be used when value of any type is expected, as they can be used in place of any type.

## Comments

- In rust, comments are written using `//` for single-line comments.
- For multi-line comments, we can use `//` for every line or use `/* */` syntax.

## Control Flow

- Control flow is used to change the flow of execution of the program.

### if Expressions

- Allows us to branch our code based on conditions.
- Blocks of code associated with conditions are called arms.
- We can use `else` to specify a block of code to execute if the condition is false.
- We can use `else if` to specify multiple conditions.
- `else` and `else if` are optional.
- The condition must be a boolean expression.
- if statements can be bound to let keywords and variables
- Here the value of the variable will be the value of the last expression in the if block.
```rust
let x = if condition { 5 } else { 6 }; // x will be
// 5 if condition is true, otherwise 6
```
- `if` and `else` conditions must return the same type.
- If they do not, it will give a compilation error like `mismatched types`.
- 

### Loops

- `loop` is an infinite loop that runs until it is explicitly stopped.
- We can use `break` to exit the loop and `continue` to skip the current iteration and move to the next iteration.
- WHen using loops within loops, then `break` and `continue` will only affect the innermost loop.
- A loop label can be used to specify which loop to break or continue.
- A loop label is a name followed by a colon, e.g., `'outer`.
- __while__ : 
    - A `while` loop runs as long as a condition is true.
    - Syntax : 
    ```rust
    while condition {
        // code to execute
    }
    ```
- __for__ :
    - A `for` loop iterates over a collection
    - Syntax : 
    ```rust
    for element in collection {
        // code to execute
    }
    ```
    - We can use `.rev()` method to iterate in reverse order.


## Ownership

- Set of rules that governs how memory is managed in Rust.
- If any rule is violated, the compiler will throw an error and will not compile.
- Features of ownnership doesnot slow down the performance of the program.

### Basics of heaps and stacks

- Both heap and stack are part of the available memory at runtime.
- Stack removes the last item added to it, while heap allows us to allocate memory dynamically.
- In heap, the memory allocator finds a block of memory that is large enough to hold the data and returns a pointer to that memory.
- Pushing to the stack is faster, because it doesnot have to search for a block of memory. As location is always at the top of the stack.
- Accessing the data in heap is slower, because you have to dereference the pointer to get the data.
- When a function is called, all the values passed in the function (even potentially, pointers to data on the heaps) are pushed onto the stack.
- When the function returns, all the values are popped from the stack.
- Main function of ownerships is to manage heap data.

### Ownership Rules

- Each value in Rust has a variable that is its _owner_.
- There can be one owner at a time.
- When owner goes out of scope, the value will be dropped.

- String type is a growable, heap-allocated data structure.
- Syntax : 
```rust
let s = String::from("hello"); // creates a new string
```
- This string is mutable.

### Memory and Allocation

- For string literal, we know the content at compile time, so the text is hardcoded into the binary.
- This makes string literal fast and efficient.
- But this makes the string literal immutable.
- Using `String` type, we can create a mutable string that can grow in size, allocated memory in the heap
- Its content is not known at compile time, so it is allocated in the heap.
- This means that memory must be requested at runtime.
- We need a way of returning this memory to the allocator when it is no longer needed.
- In other languages, it is either done manually or by garbage collector.
- In Rust, the memory is automatically returned to the allocator when the owner goes out of scope.
- This is done by the `drop` function, which is called automatically when the owner goes out of scope.

- The complex datatypes, like String, have 3 parts : 
    - Pointer to the data in the heap
    - Length of the data
    - Capacity of the data (how much space is allocated in the heap)
- Only the content of the string is stored in the heap.
- Rest are stored in the stack. 
- So, when we assign on string to another like `s1 = s2`, it will not copy the data, but only the pointer to the data.
- To avoid double free error, do not use `s1` after `s2` is assigned to it.
- Double free error occurs when we try to free the memory that is already freed.
- when you assign a completely new value to an existing variable, Rust will call the `drop` function and free the original value's memory immediately.

### Variables and data interaction with clone

- If we want to create a copy of the data, we can use the `clone` method.
- It will copy not only the contents in the stack, but also the contents in the heap.
- It is more expensive than just copying the pointer, so it should be used only when necessary.

### Stack-only Data:copy

- Integers that are known to have a fixed size at compile time, are stored entirely in the stack.
- Rust has a special trait called `Copy` that allows us to copy the data instead of moving it.
- Types that implement the `Copy` trait are called _copy types_.

### Ownership and Functions

- Mechanism of passing a paremeter works similarly to variable assignment.
- Returning values can also transfer ownership.
- If a function takes ownership of a value, the value will be dropped when the function returns.

## Referencing and Borrowing

- When wanting to use the value even after passing it to a function, we can use references.
- A reference is a pointer to a value that allows us to use the value without taking ownership of it.
- Unlike pointer, a reference is guaranteed to point to a valid value.
- The value will not be dropped when the reference stop being used.
- The action of referencing a value without taking ownership is called __borrowing__.
- References are immutable by default, meaning we cannot change the value of the reference.

### Mutable References

- We can create a mutable reference to a value using the `&mut` keyword.
- Syntax : 
```rust
let mut x = 5;
let y = &mut x; // y is a mutable reference to x
*y += 1; // we can change the value of x through y
```
- If a variable has a mutable reference, we cannot have any other references to that variable.
- This prevents data races.
- Data race is a situation where two or more threads access the same data at the same time, and at least one of them is modifying the data.
- Multiple mutable references can be made but in different scopes.
- We cannot have a mutable reference and an immutable reference to the same variable at the same time.

### Dangling References

- When a reference points to a value that has been dropped, it is called a dangling reference.
- Rust prevents dangling references by ensuring that a reference cannot outlive the value it points to.

## Structs

- A struct or structure is a custom data type that allows us to create a complex data type by combining multiple values of different types.
- Structs are defined using the `struct` keyword.
- Syntax : 
```rust
struct Point {
    x: i32,
    y: i32,
}
```
- Structures can have multiple types of fields.
- Unlike tuples, each field in a struct has a name.
- We don't have to rely on the order of the fields to access them.
- To use the struct, we can create an instance of it using the syntax : 
```rust
let p = Point { x: 1, y: 2 }; // creates a new instance of Point
```
- We can access the fields of the struct using the dot notation : 
```rust
println!("x: {}, y: {}", p.x, p.y); // prints x: 1, y: 2
```
- The entire struct instance should be mutable if we want to change any of its fields.
- When the function parameter and struct name is same, we can use shorthand syntax to create the struct instance.
```rust
struct Point {
    x: i32,
    y: i32,
}
fn create_point(x: i32, y: i32) -> Point {
    Point { x, y } // shorthand syntax
}
```
- We can also create a tuple struct, which is a struct that has no named fields, but only a tuple-like structure.

### Update structs

- We can create a new instance of a struct by copying the values from an existing instance and updating some of the fields.
```rust
let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, ..p1 }; // creates a new instance of Point with x = 3 and y = 2
```
- The `..` syntax is used to copy the values of the fields that are not specified.