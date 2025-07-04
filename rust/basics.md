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