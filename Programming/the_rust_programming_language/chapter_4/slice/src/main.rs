fn main() {

    /* The slice type
     *
     * As slice type reference a contiguous sequence of elements in a collection rather than the
     * whole collection. A slice is a kind of reference, so it does not have ownership
     * */

    let mut statement = String::from("Essa é uma frase longa");
    println!("A variavel statement tem o valor: {}", statement);

    let first_word = first_word(&statement);
    println!("A primeira palavra da String statement tem o tamanho: {}", first_word);

    // After you get and store the size of the first word, if the word change its size, the value
    // becomes wrong

    // fn second_word(s: &String) -> (usize, usize) {
    // Now we woudl use more indexes and we would still not related to the string

    /* String slices
     *
     * let s = String::from("aaaaaa");
     * let word = &s[0..4]; The word variable stores the pointer and the length of the string
     *
     * These two examples are the same, both of them gets the whole string:
     *
     * let word = &s[0..6];
     * let word = &s[0..];
     *
     * You can also drop the two numbers to slice the entire string
     *
     * let word = &s[0..];
     * */

    let first_word = first_word_2(&statement);
    println!("A primeira palavra da String statement tem o valor: {}", first_word);

    statement.clear();

    let first_word = first_word_2(&statement);
    println!("Depois de limpar a String statement a primeira palavra tem o valor: {}", first_word);

    // Other slices
    let a = [1, 2, 3, 4];
    let slice = &a[1..3];

    assert_eq!(slice, &[2,3]);
}

fn first_word(s: &String) -> usize
{
    // To go through the String element by element, we will use the method as_bytes
    let bytes = s.as_bytes();

    for (i, &item) in bytes
        .iter() // We create an iterator over the array using the iter method
        .enumerate() // The enumerate wraps the result of iter, returning each element as part of tuple
    {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// fn first_word_2(s: &String) -> &str {
fn first_word_2(s: &str) -> &str // Using &str instead of &String allow the function recieve both &String and &str values
// If we have a &str we can pass it directly, but if we have &String we can papss a reference
// Some examples are:
//
// - let my_string = String::from("hello world");
//
// - let word = first_word(&my_string[0..6]);
// - let word = first_word(&my_string[..]);
// - let word = first_word(&my_string);
// - let my_string_literal = "hello world";
// - let word = first_word(&my_string_literal[0..6]);
// - let word = first_word(&my_string_literal[..]);
// - let word = first_word(my_string_literal);
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
