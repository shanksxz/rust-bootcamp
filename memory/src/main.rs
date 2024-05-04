/* Ownership */

// fn main() {
//     let s1 = String::from("hello"); // heap allocated string
//     println!("s1 is {}", s1);

//     let s2 = s1; // s1 is moved to s2

//     // println!("s1 is {}", s1); // error: value borrowed here after move    


//     let s1 = String::from("hello");

//     let s2 = s1.clone(); // s1 is cloned to s2

//     println("s2 is {}", s2);

//     let s3 = s2;

//     println!("s2 is {}", s2); // s2 is still valid
//     // st2 is valid because s3 is a clone of s2 and not a move

//     let z = 5;
//     let y = z; // z is copied to y
//     println!("z is {}", z); // z is still valid
//     // primitive types are copied instead of moved coz they are stored on the stack
// }

/* Ownership in terms of functionc calls */

// fn takes_ownership(s: String) {
//     println!("s is {}", s);
// }

// fn makes_copy(s: i32) {
//     println!("s is {}", s);
// }

// fn generate_string() -> String {
//     String::from("New String")
//     // by returning ownership will transfered to the variable
// }

// fn add_to_string(mut p1 : String) -> String {
//     p1.push_str(" Hello After Appending!");
//     p1
// }

// fn main() {
//     let s = String::from("hello");
//     // takes_ownership(s); // s is moved to the function
//     // println!("s is {}", s); // error: value borrowed here after move

//     let x = 5;
//     makes_copy(x); // x is copied to the function
//     println!("x is {}", x); // x is still valid


//     let s1 = generate_string();
//     println!("s1 is {}",s1);
    
//     let s2 = s1;
//     // println!("s1 is {}",s1); // error s1 is ownership is transfered to s2
//     println!("s2 is {}",s2);


//     let s3 = add_to_string(s);
//     println!("s3 is {}", s3);
//     // println!("s is {}", s); // s will not valid coz its value is ownership is transfered the function

// }


/* Borrowing */

fn print_string(m: &String) {
    println!("String is {}", m);
}

fn add_to_string(mut p : String){
    // println!("Initially String is : ", m);

    // adding something
    p.push_str(" kkklkk");

    // this fn will not return anything coz we are just modifying the value not returning it
}

fn main() {
    
    let mut s1 = String::from("Hello World");
    let k = &s1;                                  
    print_string(&k); // no error
    let k5  = &mut s1; // error: cannot borrow `s1` as mutable because it is also borrowed as immutable
    // we can't have both mutable and immutable references in the same scope
    // print_string(&k); // k5 will give

    // let k1 = &s1;
    // let k2 = &mut s1;
    // add_to_same_string(&mut k5);
}

/* Slicies */

// fn main() {

//     let tweet = String::from(
//         "This is a tweet from a user"
//     );

//     let trimmed_tweet = trim_tweet(&tweet); 


//     let tweet2 = "This is a tweet from a user";
//     // let trimmed_tweet2 = trim_tweet(tweet2); // error: expected struct `String`, found `&str`
//     let trimmed_tweet2 = trim_tweet2(tweet2); // no error
//     let trimmed_tweet3 = trim_tweet2(&tweet); // no error coz &String can be converted to &str using deref coercion


//     /* with arrays */
//     let a = [1, 2, 3, 4, 5];
//     let slice = &a[1..3]; // slice is of type &[i32] and its value is [2, 3], 3 is not included

// }

// fn trim_tweet(tweet: &String) -> &str {
//    &tweet[0..20]
// }

// fn trim_tweet2(tweet: &str) -> &str {
//     &tweet[0..20]
//  }