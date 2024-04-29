#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
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
    let scale = 2;
    let sq = Rectangle::square(3);
    let rect1 = Rectangle {
        //width: dbg!(30 * scale),
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
    //println!("rect1 is {:?}", rect1);
    //println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} ", rect1.area());
    if rect1.width() {
        println!("The rectangle has a nonzero width {}", rect1.width);
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    //dbg!(&rect1);
}


