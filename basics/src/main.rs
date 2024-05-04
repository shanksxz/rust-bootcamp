const MAX_POINTS : u128 = 100_000; // constants are always immutable and we must specify the type
static mut COUNTER: u32 = 0; // static variables can be mutable and we must specify the type

// const vs static
// const is always immutable and is not allowed to be mutable
// static is allowed to be mutable and is allowed to be mutable

fn another_function(z : i32) -> () {
    println!("The value of z is: {}", z);
}

fn return_something(z : i32) -> i32 {
    // return z; // return keyword is optional
    println!("Returning Something....");
    z + 1 // no semicolon means it is an expression and the value is returned
}

fn main() {

    another_function(3);
    print!("The value of z is: {}", return_something(3)); 

    // println! is a macro that prints text to the console
    println!("Hello, world!");

    // variables are immutable by default
    let x = 5; 

    // to make a variable mutable, use the mut keyword
    let mut y = 5;
    y = 6;

    // constants are always immutable

    // shadowing 
    let z = 5;
    let z = z + 1; // shadowed z

    // mutability vs shadowing
    // mutability changes the value of the same variable
    // shadowing creates a new variable with the same name and shadows the old one

    // scope
    let a = 5;
    {
        let a = 6;
        println!("a is {}", a); // prints 6
    }

    println!("a is {}", a); // prints 5

    // data types

    // boolean
    let b: bool = true;

    // unsigned integers
    let c: u8 = 255; // 8 bits (2^8 - 1 = 255)
    let d: u16 = 65535; // 16 bits
    let e: u32 = 4294967295; // 32 bits
    let f: u64 = 18446744073709551615; // 64 bits
    let g: u128 = 340282366920938463463374607431768211455; // 128 bits

    // signed integers
    let h: i8 = 127; // 8 bits (2^7 - 1 = 127)
    let i: i16 = 32767; // 16 bits
    let j: i32 = 2147483647; // 32 bits
    let k: i64 = 9223372036854775807; // 64 bits
    let l: i128 = 170141183460469231731687303715884105727; // 128 bits

    // floating point
    let m: f32 = 3.14; // 32 bits
    let n: f64 = 3.141592653589793; // 64 bits

    // platform dependent
    let o: isize = 0; // 32 or 64 bits 
    let p: usize = 0; // 32 or 64 bits

    // char, &str, and String
    let q: char = 'a'; // single unicode character
    let r: &str = "hello"; // string slice
    let s: String = String::from("hello"); // heap allocated string

    // arrays can only have one type
    let t: [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 i32s
    let u = [0; 5]; // array of 5 i32s initialized to 0
    let v = [1, 2, 3, 4, 5]; // array of 5 i32s (type inferred)
    
    // tuples
    let w: (i32, f64, char) = (1, 3.14, 'a'); // tuple of i32, f64, and char
    let (x, y, z) = w; // destructuring
    println!("x is {}", x); // prints 1
    println!("y is {}", y); // prints 3.14
    println!("z is {}", z); // prints a

    // empty tuple
    let _ = (); // function that returns nothing returns an empty tuple

    // type aliasing
    type MyType = i32;
    let a: MyType = 5;

    let l = MAX_POINTS; // using a constant

    let m = unsafe {
        COUNTER += 1; // using a static variable
        COUNTER 
    };

    // control flow
    let a = 5;

    // expression must be a boolean
    if a > 5 {
        println!("a is greater than 5");
    } else if a < 5 {
        println!("a is less than 5");
    } else {
        println!("a is equal to 5");
    }

    let b = if a > 5 { 1 } else { 0 }; 


    // loops
    loop {
        println!("Infinite Loop");
        break; // break keyword is used to break out of the loop
    }

    let mut c = 0;
    while c < 5 {
        println!("While Loop");
        c += 1;
    }

    // for loop
    for d in 0..5 {
        println!("For Loop");
    }

    let e = [1, 2, 3, 4, 5];
    for f in e {
        println!("{}", f);
    }

}
