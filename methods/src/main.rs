struct Rectangle{
    width : u32,
    height : u32,
}

impl Rectangle{
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle{
        width : 32,
        height : 42,
    };
    let rect2 = Rectangle {
        width : 45,
        height : 35,
    };
    println!("The area of rect 1 is {}",rect1.calculate_area());
    println!("The area of rect 2 is {}",rect2.calculate_area());
}
