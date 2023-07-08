#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, comparison_rect: &Rectangle) -> bool {
        self.width > comparison_rect.width && self.height > comparison_rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(10);
    println!("square area: {}", sq.area());

    println!(
        "The first rectangle can hold the second, {}",
        rect1.can_hold(&rect2),
    );

    println!(
        "The first rectangle can hold the third, {}",
        rect1.can_hold(&rect3),
    );

    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     rect1.area()
    // )
}
