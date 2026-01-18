fn main() {
    let x: i32 = 5;
    println!("Value of x is {x}");

    let mut y: i32 = 6;
    println!("Value of y is {y}");

    y = y + 1;
    println!("Value of y 2 is {y}");

    {
        y = y * 2;
        println!("Value of y 3 is {y}");
    }

    // Shaddowing
    let z :i32 = 8;

    let z = y + z;
    println!("Value of z is {z}");

    {
        let z = z * 2;
        println!("Value of z 3 is {z}");
    }

    const HAPPY_CODE: &str= "Hello World";
    const HAPPY_CODE1: i64= 5;
    const HAPPY_CODE2: f64= 7.7;
    
    println!("Value of Constant String is {HAPPY_CODE}");
    println!("Value of Integer is {HAPPY_CODE1}");
    println!("Value of Float is {HAPPY_CODE2}");

    let k = 78;
    println!("Value of Int is {k}");

    // when we want to declare a type then use : with variable name, otherwise this can be used directly
    // this goes in rust like above
}
