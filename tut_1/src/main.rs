fn main(){
    
    const INTEREST_RATE : u32 = 10;
    println!("Interest rate is:{INTEREST_RATE}");
    
    println!("Namaste all of you!");
     
    // VARIABLE
    let num1 = 12; // immutable var
    println!("Number is:{num1}");

    let mut num2: i32 = 34; // mutable var
    num2 =3444;
    println!("Number 2 is:{num2}"); 

    // Shadowing
    let roll_no: u32 = 22;
    println!("My roll no: {roll_no}");

    let roll_no: u32 = 122;
    println!("My other roll no: {roll_no}");
}