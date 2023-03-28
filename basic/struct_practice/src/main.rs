#[derive(Debug)]  // 在結構體前加上屬性(attribute)
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 62,
        height: 46,
    };

    println!(
        "長方形的面積為 {} 平方像素。",
        rect1.area()
    );

    println!("rect1 is {:?}", rect1);
    if rect1.width() {
        println!("長方形的寬度不為零，而是 {}", rect1.width);
    }
    println!("____");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 能容納 rect2 嗎？{}", rect1.can_hold(&rect2));
    println!("rect1 能容納 rect3 嗎？{}", rect1.can_hold(&rect3));

}
