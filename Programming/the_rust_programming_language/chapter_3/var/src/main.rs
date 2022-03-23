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

    // Data Types
    // Scalar and Compound

    /* Scalar Types: Represents a single value. The four primary scalar types: integers,
    * floating-point, numbers, booleans and characters
    * Integer: i8, i16, i32, i64, i128, isize
    * Unsigned Integer: u8, u16, u32...
    *
    * isize represents 64 bits if you are using a 64-bits architeture or 32 if you are using 32-bit (Used in collections)
    * architeture
    *
    * Representations:
    *
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
