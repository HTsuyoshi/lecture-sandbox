fn main()
{
    // Another solution, instead of letting the function get the ownership we can just let the
    // function borrow the ownership of an object for a moment

    let s1 = String::from("Uma string qualquer");
    let s1_length = len(&s1);

    println!("A String s1 tem o valor: {}", s1);
    println!("O tamanho da String s1 é {}", s1_length);

    let mut s1 = String::from("Uma string qualquer");

    borrow_change(&mut s1);
    println!("A nova String s1 tem o valor: {}", s1);

    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // We cannot borrow the same object more than once at time, to prevent data races at compile
    // time, data race is similar to a race condition and happens when these tree behaviours
    // occur:
    // - Two or more pointer access the same data at the same time
    // - At least one of the pointers is being used to write to the data
    // - There's no mechansm being used to synchronize access to the data

    // To create two references we can use a scope to limit the first variable

    {
        let r1 = &mut s1;
        println!("Usando String MUTAVEL s1 dentro de um escopo: {}", r1);
    }

    let r2 = &mut s1;
    println!("Usando String MUTAVEL s1 fora de um escopo: {}", r2);


    // let mut s = String::from("Hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("Using two references r1 and r2: {}, {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("Using MUTABLE reference r3: {}", r3);

    // let dangling = dangling_reference();
    // Rust dont let you create dangling references

    let new_string = new_string();
    println!("Uma string nova foi criada com o valor {}", new_string);
}

fn len(s1: &String) -> usize
{
    s1.len()
    // Here, as s1 is a reference it isnt dropped

    // // Error you cannot change something that isnt yours
    // s1.push_str("aasdf");
    // s1.len()
}

fn borrow_change(s1: &mut String)
{
    s1.push_str(" asdfasdf");
}

// fn dangling_reference() -> &String // Return a reference string
// {
//     let s = String::from("string");
//     &s // Return s to String
// } // Here, s goes out of scope, and is dropped. Its memory goes away

fn new_string() -> String
{
    let s = String::from("new_stringaa");
    s
}
