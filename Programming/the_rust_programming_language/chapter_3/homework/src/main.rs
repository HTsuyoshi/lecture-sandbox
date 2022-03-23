fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0)*5.0/9.0
}

fn fib(n: i32) -> i32 {
    if n < 2 {
        return n
    }
    fib(n - 1) + fib(n - 2)
}

fn print_carol_music() {
    let numbers  = ["first", "second", "third", "forth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth", "eleventh", "Twelfth"];
    let lines = ["A partridge in a pear tree.", "Two turtle doves... etc."
, "Three french hens... etc." , "Four calling birds... etc." , "Five golden rings... etc." , "Six geese a-laying... etc." , "Seven swans a-swimming... etc." , "Eight maids a-milking... etc." , "Nine ladies dancing... etc." , "Ten lords a-leaping... etc." , "'Leven pipers piping... etc." , "Twelve drummers drumming... etc.'"];
    println!("Christmas Carols");
    println!("Miscellaneous");
    println!("The Twelve Days Of Christmas");

    for i in 0..12 {
        println!("On the {} day of Christmas",numbers[i]);
        println!("My true love gave to me");
        println!("{}", lines[i]);
    }
}

fn main() {
    println!("150 F in celsius is {}!", fahrenheit_to_celsius(150.0));
    println!("19nth number of fibonacci is {}!", fib(19));
    println!("18nth number of fibonacci is {}!", fib(18));
    print_carol_music();
}
