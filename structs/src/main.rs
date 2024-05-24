// structs are used to create custom data types (like classes in other languages) / group related data together

// struct Product {
//     name : String,
//     price : f32,
//     in_stock : bool
// }

// impl Product {

//     // constructors are defined as associated functions
//     fn new(name : String, price : f32, in_stock : bool) -> Product {
//         Product {
//             name,
//             price,
//             in_stock
//         }
//     }

//     // associated functions are defined in the context of the struct
//     fn get_default_sale_tax() -> f32 {
//         // no self parameter
//         0.1
//     }

//     // methods are defined in the context of the struct
//     fn calculate_sales_tax(&self) -> f32 {
//         // &self is a reference to the struct instance
//         // immutable reference to the struct instance
//         self.price * Product::get_default_sale_tax()
//     }

//     fn set_price(&mut self, price : f32){
//         // &mut self is a mutable reference to the struct instance
//         self.price = price;
//     }

//     fn buy(self) -> i32 {
//         // self is the struct instance
//         // self is consumed by this method
//         // self is no longer available after this method is called
//         let name = self.name;
//         println!("You bought {}", name);
//         123
//     }
// }

// fn main() {
//     // let mut book = Product {
//     //     name : String::from("Rust Programming"),
//     //     price : 49.99,
//     //     in_stock : true
//     // };

//     // let price = book.price;
//     // book.in_stock = false; // typeof book must be mutable to change its fields

//     // println!("sales tax is {}", book.calculate_sales_tax());
//     // book.set_price(39.99);
//     // book.buy(); 
//     // book.set_price(29.99); // this will cause an error because book is no longer available

//     let book = Product::new(String::from("Rust Programming"), 49.99, true);


//     // tuple structs

//     let rgb_color = (255, 0, 0); // tuple
//     let cmyk_color = (255, 0, 0, 0); // tuple
    
//     // tuple vs struct 
//     // tuple cannot give names to the elements like struct

//     // tuple structs are used to give names to tuples
//     struct RgbColor(u8, u8, u8);
//     struct CmykColor(u8, u8, u8, u8);

//     let rgb_color = RgbColor(255, 0, 0);
//     let cmyk_color = CmykColor(255, 0, 0, 0);

//     // unit-like structs
//     struct UnitLikeStruct; // no fields
// }


/* ENUMS */

// struct Product {
//     name : String,
//     price : f32,
//     category : ProductCategory,
//     in_stock : bool
// }

// enum ProductCategory {
//     Book,
//     Music,
//     Movie,
//     Software
// }

// fn main() {

//     let category = ProductCategory::Book;

//     let book = Product {
//         name : String::from("Rust Programming"),
//         price : 49.99,
//         category,
//         in_stock : true
//     };
    
// }

// enum Command {
//     Undo,
//     Redo,
//     AddText(String),
//     MoveCursor(i32, i32),
//     Replace {
//         from : String,
//         to : String
//     }
// }

// impl Command {
//     fn serialize(&self) -> String {
//         // String::from("JSON string")
//         match self {
//             Command::Undo => String::from("{ \"type\" : \"Undo\" }"),
//             Command::Redo => String::from("{ \"type\" : \"Redo\"}"),
//             Command::AddText(text) => format!(
//                 "{{ \"type\" : \"AddText\", \"text\" : \"{}\" }}", text
//             ), // format! is like println! but returns a string
//             Command::MoveCursor(x, y) => format!(
//                 "{{ \"type\" : \"MoveCursor\", \"x\" : {}, \"y\" : {} }}", x, y
//             ),
//             Command::Replace { from, to } => format!(
//                 "{{ \"type\" : \"Replace\", \"from\" : \"{}\", \"to\" : \"{}\" }}", from, to  
//             )
//         }
//     }
// }

// fn main() { 
//     let cmd = Command::Undo;
//     let cmd = Command::AddText(String::from("Hello"));
//     let cmd = Command::MoveCursor(3, 5);
//     let cmd = Command::Replace {
//         from : String::from("Hello"),
//         to : String::from("World")
//     };


//     let serialized = cmd.serialize();
//     println!("{}", serialized);

//     // match
//     // let age = 30;

//     // match age {
//     //     1 => println!("age is 1"),
//     //     13..=19 => println!("age is a teenager"),
//     //     20..=29 => println!("age is in 20s"),
//     //     30..=39 => println!("age is in 30s"),
//     //     // _ => println!("age is something else") // default case
//     //     x => println!("age is {}", x) // default case
//     // }
// }


/* Options */

// fn main() {
//     // Option is an built-in enum in Rust
//     let username = get_username(1);
//     // match username {
//     //     Some(name) => println!("Hello, {}", name),
//     //     None => println!("Hello, world")
//     // }

//     // another way to write the above match
//     if let Some(name) = username {
//         println!("Hello, {}", name);
//     } // what happend here is that the value inside the Option is extracted and assigned to the variable name
// }


// fn get_username(user_id : i32) -> Option<String> {
//     let db_result = String::from("John Doe");
//     if user_id == 1 {
//         Some(db_result)
//     } else {
//         None
//     }
// }

/* Result */

// fn main() {
//     // Option is an built-in enum in Rust
//     let username = get_username(1);
//     // match username {
//     //     Some(name) => println!("Hello, {}", name),
//     //     None => println!("Hello, world")
//     // }

//     // another way to write the above match
//     if let Some(name) = username {
//         println!("Hello, {}", name);
//     } // what happend here is that the value inside the Option is extracted and assigned to the variable name
// }


// fn get_username(user_id : i32) -> Option<String> {
//     // let db_result = db_query(String::from("SELECT name FROM users WHERE id = 1"));
//     let query = format!("SELECT name FROM users WHERE id = {}", user_id);
//     let db_result = db_query(query);
//     // match db_result {
//     //     Ok(name) => Some(name),
//     //     Err(_) => None
//     // }
//     db_result.ok()
// }

// fn db_query(query : String) -> Result<String, String> {
//     if(query.is_empty()) {
//         return Err(String::from("Query is empty"));
//     } else {
//         return Ok(String::from("Query result"));
//     }
// }

/* Vectors */

fn main() {
   let mut v = Vec::new(); 
    v.push(1);

    let mut  v2 = vec![1, 2, 3, 4, 5]; // vec! is a macro that creates a vector

    let third = &v2[2]; // indexing
    let third = v2.get(2); // get method returns an Option (refrencing)

    if let Some(third) = v2.get(2) {
        println!("The third element is {}", third);
    }


    for s in &mut v2 {
        *s += 50;
    } 

    let mut v3 = vec![];
    for s in v {
        v3.push(s);
    }
    
    // let value = v.get(0); // err because v is moved to v3
}