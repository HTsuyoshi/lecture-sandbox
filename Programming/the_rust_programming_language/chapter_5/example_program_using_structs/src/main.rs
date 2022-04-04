#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct LotsOfRectangles {
    rec1: Rectangle,
    rec2: Rectangle,
}

fn main() {
    let ret = Rectangle {
        width: dbg! (30 * 20),
        height: 50,
    };

    let lor = LotsOfRectangles{
        rec1: Rectangle {
            width: 30,
            height: 50,
        },
        rec2: Rectangle {
            width: 30,
            height: 50,
        },
    };

    dbg!(&ret);

    println!(
        "For debugging purposes {:?}", ret
        );

    println!(
        "In larger structures we can use {:#?}", lor
        );

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&ret)
        );
}

// We can pass the Struct Rectangle as argument, instead of two numbers or a tuple

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
