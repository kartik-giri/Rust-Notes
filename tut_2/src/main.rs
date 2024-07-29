fn main() {
    //Integer
    let num: u32 = 100;
    for i in 1..100{
        println!("{}",num-i);
    }

    //floating point
    let float1 = 33.4;
    let float2 = 44.3;
    println!("{}", float1/float2);

    let a = 0.1;
    let b = 0.3;
    assert!(a+b == 0.4);
}
