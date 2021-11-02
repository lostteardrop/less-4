pub trait Getlamp{
    fn get_name(&self) -> &String;
    fn get_time(&self) -> i32;
}
pub struct Lamp{
    pub name : String,
    pub time : i32,
}
impl Getlamp for Lamp {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_time(&self) -> i32 {
        self.time
    }
}
fn print_info(item : impl Getlamp) {
    println!("name: {},time {}", item.get_name(),item.get_time());
}
fn main() {  
    let red = Lamp { name: String::from("red"), time: 5};
    let yellow = Lamp { name: String::from("yellow"), time: 3};
    let green = Lamp { name: String::from("green"), time:20};
    print_info(red);
    print_info(yellow);
    print_info(green);
}