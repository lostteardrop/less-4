struct Rectangle{
    width:i32,
    height:i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle{width:2,height:3};
    println!("area:{}",rectangle.area());
}
