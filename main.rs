fn main() {
    // Boolean
    let b1: bool = true; 

    // Unsigned integers
    let i1_u8: u8 = 1;   
    let i1_u16: u16 = 1;  

    // Signed integers
    let i7_i8: i8 = 1;    
    let i8_i16: i16 = 1;  

    // Floating point numbers
    let f1: f32 = 1.6;   
    let f2: f64 = 1.0;   

    // Platform-specific integers
    let p1: usize = 1;    
    let p2: isize = 1;    

    // Characters, &str and String
    let c1: char = 'c';                // Character
    let s1: &str = "hello";           // String slice
    let s2: String = String::from("hello"); // Owned string

    // Arrays
    let a1: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 integers
    let i1: i32 = a1[0]; // Accessing the first element (1)

    // Tuples
    // Create a tuple with different types
    let t1: (i32, f64, &str) = (42, 3.14, "hello");

    // Destructure the tuple into individual variables
    let (i1, f1, s1): (i32, f64, &str) = t1;

    // Print the values from the tuple
    println!("Integer: {}", i1);       // prints 42
    println!("Float: {}", f1);          // prints 3.14
    println!("String slice: {}", s1);   // prints "hello"

    // Type aliasing
    type age = u8; // Define a new type alias for u8
    let al: age = 57; // Use the alias to declare a variable

    // Print the alias variable
    println!("Age: {}", al); // prints 57
}
