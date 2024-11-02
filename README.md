## Data Types in Rust

### 1. Boolean Type

- **Definition**: Represents a value that can be either `true` or `false`.
- Example:

```rust
let b1: bool = true;
```

### 2. Unsigned Integers

- **Types**:
    - `u8`: Unsigned 8-bit integer (0 to 255).
    - `u16`: Unsigned 16-bit integer (0 to 65,535).

```rust
let i1_u8: u8 = 1;   
let i1_u16: u16 = 1;  
```

### 3. Signed Integers

- **Types**:
    - `i8`: Signed 8-bit integer (-128 to 127).
    - `i16`: Signed 16-bit integer (-32,768 to 32,767).
- **Example**

```rust
let i7_i8: i8 = 1;    
let i8_i16: i16 = 1;  
```

### 4. Floating Point Numbers

- **Types**:
    - `f32`: 32-bit floating-point number.
    - `f64`: 64-bit floating-point number (default in Rust).
- **Example**:
    
    ```rust
    
    let f1: f32 = 1.6;
    let f2: f64 = 1.0;
    
    ```
    

### Platform-Specific Integers

- **Types**:
    - `usize`: Unsigned integer type for indexing (size depends on architecture, 32-bit or 64-bit).
    - `isize`: Signed integer type for indexing (size depends on architecture).
- **Example**:
    
    ```rust
    
    let p1: usize = 1;
    let p2: isize = 1;
    
    ```
    

### 6. Character, String Slice, and String

- **Types**:
    - `char`: Represents a single Unicode character.
    - `&str`: String slice, a view into a string.
    - `String`: Owned string type.
- **Example**:
    
    ```rust
    
    let c1: char = 'c';                // Character
    let s1: &str = "hello";           // String slice
    let s2: String = String::from("hello"); // Owned string
    
    ```
    

### 7. Arrays

- **Definition**: Fixed-size list of elements of the same type.
- **Example**:
    
    ```rust
    
    let a1: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 integers
    let i1: i32 = a1[0]; // Accessing the first element (1)
    
    ```
    

### 8. Tuples

- **Definition**: A fixed-size collection that can hold elements of different types.
- **Example**:
    
    ```rust
    
    let t1: (i32, f64, &str) = (42, 3.14, "hello"); // Tuple with an integer, float, and string slice
    let (i1, f1, s1): (i32, f64, &str) = t1; // Destructure the tuple into variables
    println!("Integer: {}", i1);   // prints 42
    println!("Float: {}", f1);      // prints 3.14
    println!("String slice: {}", s1); // prints "hello"
    
    ```
    

### 9. Type Aliasing

- **Definition**: Create an alias for an existing type for readability or convenience.
- **Example**:
    
    ```rust
    
    type age = u8; // Define a new type alias for u8
    let al: age = 57; // Use the alias to declare a variable
    
    ```
    

```rust
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

```
