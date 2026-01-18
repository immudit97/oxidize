// Data Type in Rust
// two data type subsets: scalar and compound

// Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
// we are using 

// What does _x do?

// Variables whose names start with an underscore tell the compiler:

// “I know this variable is unused, and that’s okay.”

fn main(){
    let _x: i128 = 5;
    let _y: f32 = 5.5;
    let _z: bool = true;

    // telling the compiler preceeding with an underscore I have declared but I am not gonna use which usually would't happen in rust
    // check for value m would throw error
    let _k: i32 = 10;  

    // if you declare wither use or comment 
    // let m: i64 = 64;    

    println!("x is {_x}");
    println!("y is {_y}");
    println!("z is {_z}");


    let sum_p = 4 + 5;
    println!("Sum p is {sum_p}");
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


    println!("sum is {sum}");
    println!("difference is {difference}");
    println!("product is {product}");
    println!("quotient is {quotient}");
    println!("truncated is {truncated}");
    println!("remainder is {remainder}");

    // when we want to declare a type then use : with variable name, otherwise this can be used directly


}
