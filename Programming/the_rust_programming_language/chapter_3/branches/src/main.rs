fn main() {
    // If expressions

    let n: i32 = 5;

    if n < 10 {
        println!("Your number is lower than 10");
    } else {
        println!("Your number is equal or higher than 10")
    }

    let is_321: bool = true;

    let num = if is_321 { 321 } else { 123 };

    println!("The value of num is {}", num);

    // Infinite loop

    let mut count = 0;
    loop {
        println!("INFINITE");
        count += 1;

        if count > 10 {
            break;
        }
    }

    // Returning values from loop
    let mut count = 0;
    let returned = loop {
        println!("INFINITE");
        count += 1;

        if count > 10 {
            break count;
        }
    };

    println!("The value returned by the loop is {}", returned);

    let mut n = 15;
    while n > 10 {
        println!("The value of n is {}", n);
        n -= 1;
    }

    let tup = [1, 2, 3, 4, 5];

    // The most common loop in rust
    for e in tup {
        println!("elemento {}", e);
    }

    // Using ranges
    for e in (2..10).rev() {
        println!("inverse {}", e);
    }
}
