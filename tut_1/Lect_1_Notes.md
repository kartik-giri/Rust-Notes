# Lecture 1

## Introduction 
Rust is a programming language focused on safety, speed, and concurrency. Developed by Graydon Hoare at Mozilla, it is designed to provide memory safety without sacrificing performance. Rust achieves this through a strict compiler and a unique system of ownership and borrowing.

### Rust essential commands:
1. `cargo new foldername` create new rust project.
2. `cargo build`: This command will will create executable file of our program.
3. `cargo run`: This command will run the program and show the result in our terminal.
4. `rustfmt src/Filename.rs`: This will format our rust program code.
5. `cargo build --release`: This command is used to compile it with optimizations.This command will create an executable in target/release instead of target/debug. It is used when product is finally ready.
6. `rustup update`: This command will instal latest version of rust.
7. `rustup doc`: Will open local copy of rust doc offline.

### Basic syntax of rust program:
```rust
fn main() {
    println!("Namaste all of you!");
}
```
* The `main` function is special: it is always the first code that runs in every executable Rust program.
* `println!` is the macro in rust which is used to print. `!` means that we’re calling a macro instead of a normal function
* ; means the statement is ended.

## Variables
* In rust variables are immutable by default. To make variable mutable we need to add mut keyword.
* The variables are assigned using let keyword just like JS.
* The scope of variable is defined by the block of code in which it is declared.
* Function is a named block of code which is usable.
* Shadowing allows a var to be re=declared in the same scope with same name.

```rust
fn main(){
    // VARIABLE
    let num1 = 12; // immutable var
    println!("Number is:{num1}");

    let mut num2: i32 = 34; // mutable var
    num2 =3444;
    println!("Number 2 is:{num2}"); 
}
```
### Shadowing
* We can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one.
* With shadowing we can also re-declare the variable with different data-type.
```rust
fn main(){
    // Shadowing
    let roll_no: u32 = 22;
    println!("My roll no: {roll_no}");

    let roll_no: u32 = 122;
    println!("My other roll no: {roll_no}");
}
```

### Unused variables
* We can declare unused variable like this `_varName`.
* Or add this `#[allow(unused_variables)]` in code.
```rust
fn main() {
    let _x = 1; 
}
```

### Destructuring
* We can destructure a tuple or array to separate variables.
```rust
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
```
### Constants
* Constants are always immutable they are assigned using `const` keyword.
* Rust naming convention for constants is to use all uppercase with underscores between words.
```rust
fn main(){
    
    const INTEREST_RATE : u32 = 10;
    println!("Interest rate is:{INTEREST_RATE}");
}    
```
