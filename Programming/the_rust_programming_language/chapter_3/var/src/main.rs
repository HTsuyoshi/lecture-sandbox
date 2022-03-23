fn main() {

    // Variables and Constants
    // Immutable variable

    let y = 5;
    println!("The value of y is: {}", y);
    // Mutable variable

    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    // const != variables
    // constants are always Immutable, and you arent allowed to use mut

    const EULER: f32 = 1.35 * 2.0;
    println!("The value of constant EULER is: {}", EULER);

    // OBS: rust compiler uses constant evaluation for calculate the result of an operation
    // Shadowing

    {
        let y = y - 1;
        println!("The value of y in this scope is: {}", y);

        let space = "    ";
        let space = space.len();
        println!("The size of space is: {}", space);
    }

    tuples();
    array();
    fun_with_parameters(23);
    statements_expressions();
    let returned = return_value();
    println!("The value returned is {}", returned);
    let returned = wrong_return_value(1);

    // Data Types
    // Scalar and Compound

    /* Scalar Types: Represents a single value. The four primary scalar types: integers,
    * floating-point, numbers, booleans and characters
    * Integer: i8, i16, i32, i64, i128, isize
    * Unsigned Integer: u8, u16, u32...
    *
    * isize represents 64 bits if you are using a 64-bits architeture or 32 if you are using 32-bit (Used in collections)
    * architeture
    * Representations:
    * Decimal: 1_000 = 1000
    * Hex:     0xff = 16² + 16
    * Octal:   0o77 = 8² + 8
    * Binary:  0b1000_0000 = 128
    * Byte:    b'A'
    * */

    /* Integer Overflow
     *
     * Debug mode:
     *
     * Panic will occur and the program exit
     *
     * Release mode:
     *
     * It will use two complement wrapping (256 turns into 0, 257 to 1)
     *
     * */

    /* Floating-Point
     *
     * By default the float is 64 bits, but you can set to 32 as following:
     *
     * let y: f32 = 3.0;
     * */

    /* The Character Type
     *
     * Supports UTF-8 characters
     * let z = 'ℤ';
     * let heart_eyed_cat = '😻';
     *
     * */
}

/* Compound Types
 *
 * Compound types can group multiples values into one type. Rust has two primitive compound
 * types: tuples and arrays
 *
 * */

/* The Typle Type
 *
 * Tuples have a fixed length so once declared they cannot grow or shrink in size
 *
 * Tuples are created by writing comma-separated list of values inside parentheses. Each
 * position in the tuple has a type.
 *
 * */

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of (x, y, z) is ({} | {} | {})", x, y, z);
    println!("or the tuple can be acessed by comma ({} | {} | {})", tup.0, tup.1, tup.2);
}

/* The Array Type
 *
 * Every element of the array needs to be the same type. The array is also a fixed length
 * Its useful when you want data allocated on the stack rather than heap
 *
 * */

fn array() {
    let arr = [1, 2, 3, 5];
    println!("The array values are {} {} {} {}", arr[0], arr[1], arr[2], arr[3], );

    // let arr: [i32; 4]; // Empty array
    // It gives you an error
    // println!("Acessing an empty value of the array {}", arr[0]);

    // Samw error
    // println!("Acessing an higher value of the array {}", arr[5]);
}

fn fun_with_parameters(i: i32) {
    println!("The number {} was used as parameter", i);
}

fn statements_expressions() {
    let _four = {
        let x = 3;
        x + 1 // If you let semicolons at the end of the expression they are turned into a statement, so it doesnt return nothing
    };
    println!("The value of the variable _four is {}", _four)
}

// To declare the type of the returned data we use the symbol ->
// You can return of your function early by using the keyword return
// But most of the functions return the last expression implicity

fn return_value() -> i32 {
    123
    // or
    // return 123
}

// Returning an expression instead of a value

fn wrong_return_value(x: i32) -> i32 {
    // x + 1; // Returning this value instead of
    x + 1
    // lead to a type called unit type () so the
    // compiler give us an error called mismatched types
}
