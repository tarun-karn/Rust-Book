struct Rectangle{
    width : u32,
    height : u32
}

fn main (){
    let rect = Rectangle{width:32,height:43};

    let area = calculate_area(&rect);
    println!("Area of rectangle is {area}");
}
fn calculate_area(rect : &Rectangle) -> u32 {
    rect.width * rect.height
}