fn main(){
    const HAPPY_CODE: &str= "Hello World";
    const HAPPY_CODE1: i64= 5;
    const HAPPY_CODE2: f64= 7.7;
    
    println!("Value of Constant String is {HAPPY_CODE}");
    println!("Value of Integer is {HAPPY_CODE1}");
    println!("Value of Float is {HAPPY_CODE2}");
}

// To Create a seperate cargo new directory_name each time is a wrong approach
// instead we can a bion folder and have all code related files for rust there and then use below command
// go to src directory and use cargo run --bin constants
// constants is the name of constants.rs file