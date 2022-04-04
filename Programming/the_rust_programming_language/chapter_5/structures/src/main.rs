struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("email@email.com.br"),
        username: String::from("nom3"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("emailsdfa");

    let user2 = build_user(String::from("string"), String::from("username"));

    println!("active: {}, username: {}, email: {}, sign-in count: {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    // Instead of instantianting every attribute of the object
    //
    // let user2 = User {
    //    active: user1.active
    //    username: user1.username
    //    email: String::from("new_email@email.com")
    //    sign_in_count: user1.sign_in_count
    // }
    //
    // You can skip this part and just copy every attribute, and just change what you need

    let user2 = User {
       email: String::from("new_email@email.com"),
       ..user1
    };

    println!("Now the update values of the user2 will be:");
    println!("active: {}, username: {}, email: {}, sign-in count: {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    // Remember that now we can no longer use the user1, because its attributes were borrowed to
    // the user2 object
    //
    // But if the attributes name, and email were created the user1 would still usable, because
    // the other attributes implement the Copy Trait

    // Tuples structs without name fields

    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);

    println!("The values of the red color are respectively: R: {}, G: {}, B:{}", red.0, red.1, red.2);
    println!("The values of the origin point are respectively: X: {}, Y: {}, Z:{}", origin.0, origin.1, origin.2);

    // Unit-Like structs, they behave similarly to (). They can be useful when you need to
    // implement a trait on some type, but don't have any data that you wantto store in the type
    // itself

    let _subject = AlwaysEqual;

    // The AlwaysEqual instance is equal to every instance of any other type.

    // In the user struct we used the String type instead of &str string slice type. This is a
    // deliberate choice, because we want each instance of this struct to own all of its data and
    // for that data to be valid for as long as the entire struct is valid
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // Instead of using email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}
