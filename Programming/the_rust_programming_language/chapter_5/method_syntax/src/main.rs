struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    // Associated functions

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Methods

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    let rec_1 = Rectangle {
        width: 29,
        height: 49,
    };

    println!("The rectangle area is {}", rec.area());

    if rec.width() {
        println!("The rectangle width is {}", rec.width);
    }

    println!("The rectangle rec can hold rectangle 1? {}", rec.can_hold(&rec_1));

    let sqr = Rectangle::square(10);

    println!("The new square created has the length of {}", sqr.width);

}
