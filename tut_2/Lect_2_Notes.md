# Lecture 2
## Data Types
* The two data type subsets: scalar and compound.
* Rust is a statically typed language, which means that it must know the types of all variables at compile time.
* A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

## Number - Integer
* Integer is a whole number which means it does not have a fraction or decimals.
* The integer size can varies from 8 bits(1 byte) to 128 bits(16 bytes).
* Default type for int: i32 and for floats: f64
* If most significant bit is 0 than number is +ve and if most significant bit is 1 than number is -ve.
* We can't assign signed integer to unsigned integer.
* In rust numbers could be underflow and overflow and will throw error.
* Integer is of 2 types:
1. Signed integer: It consist of both +ve and -ve integer.
2. Unsigned integer: It only contain +ve integers.
![alt text](<Screenshot (219).png>)

```rust
fn main() {
    //Integer
    let num: u32 = 100;
    for i in 1..100{
        println!("{}",num-i);
    }
}
```

### Integer - What is a word?
* Processor does not not read 1 byte at a time from memory.But actually read 1 word at a time.
* In a 32-bit processor it can access 4 bytes(32 bits) at a time.
* In a 64-bit processor it can access 8 bytes(64 bits) at a time.
![alt text](<Screenshot (221).png>)

## Number - Floating point
```rust
fn main() {
    //floating point
    let float1 = 33.4;
    let float2 = 44.3;
    println!("{}", float1/float2);
}
```
* f32 = size of 32 bits
* f64 = size of 64 bits
* All floating-point types are signed.

## Integer Overflow
Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error; we’ll discuss panics in more depth in the “Unrecoverable Errors with panic!” section in Chapter 9.

When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

* Wrap in all modes with the wrapping_* methods, such as wrapping_add.
* Return the None value if there is overflow with the checked_* methods.
* Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
* Saturate at the value’s minimum or maximum values with the saturating_* methods.

## Boolean
*  Booleans are one byte in size.
```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```
## Char
* Rust’s char type is the language’s most primitive alphabetic type.
* Note that we specify char literals with single quotes.
* Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

