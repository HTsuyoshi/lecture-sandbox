fn main() {
    let statement = String::from("statement stored in the heap memory");
    println!("{}", statement);

    {
        let mut statement = String::from("statement stored in the heap memory");
        statement.push_str(", add string");
        println!("{}", statement);
    }
    // Unlike other programs when a variable run out of the scope the function drop is called and
    // the memory is disallocated. The drop function is automatically called when the curly bracket
    // close

    // When a reference is created and the code goes out of scope the same memory is dropped two
    // times, so the rust compiler dont let you use invalidated reference

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world", s1); // Wont compile
    println!("{}, world", s2);

    // When you do s2 = s1 you are moving the content of s1 to s2
    // And any automatic copying can be assumed to be inexpensie in terms of runtime performance

    // If we want to deeply copy the heap data of the String, we can use a common method called
    // clone

    let s3 = s2.clone();

    println!("{}, world", s3);

    // Stack-Only Data Copy
    // Datas types such as integers that have know size at compile time are stored entirely on the
    // stack, so actual copies are quick to make

    let x: i32 = 10;
    let _y: i32 = x;

    // Ownership and Functions
    stack_fun(x);

    let _z = x; // As x is allocated in stack its easy to copy it so the function stack_fun is just taking a copy of x

    heap_fun(s3);

    // let greeting = s3; // As s3 is allocated in heap memory the own of s3 is given to the function heap_fun(), so we cannot use the s3 string anymore

    let s2 = takes_and_gives_back(s2);
    println!("{}", s2);

    // One solution is returning the ownership everytime
    let (s2, x) = takes_do_thing_and_gives_back(s2);
    println!("{} {}", s2, x);
}

fn stack_fun(random_integer: i32) {
    println!("{}", random_integer);
}

fn heap_fun(random_string: String) {
    println!("{}",random_string);
}

fn takes_and_gives_back(random_string: String) -> String {
    random_string
}

fn takes_do_thing_and_gives_back(random_string: String) -> (String, usize){
    (random_string, 12)
}
