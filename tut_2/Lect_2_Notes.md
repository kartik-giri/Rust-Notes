# Lecture 2

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

