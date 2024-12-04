fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");


    //variables in rust
    //Signed integers: integers that can be positive or negative values
    let integer_8: i8 = -8;         //8-bit signed integer
    let integer_16: i16 = 16;       //16-bit signed integer
    let integer_32: i32 = -32;      //32-bit signed integer
    let integer_64: i64 = 64;       //64-bit signed integer
    let integer_128: i128 = -128;   //128-bit signed integer
    let arch_integer: isize = 1;    //64-bit or 32-bit signed integer (depends on your cpu's architecture)
    //Unsigned integers: integers that can assume only positive values
    let uinteger_8: u8 = 8;         //8-bit unsigned integer
    let uinteger_16: u16 = 16;      //16-bit unsigned integer
    let uinteger_32: u32 = 32;      //32-bit unsigned integer
    let uinteger_64: u64 = 64;      //64-bit unsigned integer
    let uinteger_128: u128 = 128;   //128-bit unsigned integer
    let arch_uinteger: usize = 42;  //64-bit or 32-bit unsigned integer (depends on your cpu's architecture) 
    //Floating-point types (all of them are signed)
    let floating_32: f32 = 2.6;    //32-bit floating-point
    let floating_64: f64 = 4.91;   //64-bit floating-point
    //Boolean types -> 1 byte or 8 bits
    let isEarthRound = true;       //Non-explicit type
    let isEarthFlat: bool = false; //Explicit type annotation
    //Character type -> 4 bytes or 32 bits
    let letter = 'c';              //Non-explicit type
    let other_letter: char = 'h';  //Explicit type annotation
    let emoji = '😻';                //Emoji character
}
