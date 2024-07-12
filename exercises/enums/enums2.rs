// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]  
  
enum Message {  
    Quit,  
    Echo(String), // 假设Echo消息带有一个字符串参数  
    Move { x: i32, y: i32 }, // 假设Move消息带有x和y坐标  
    ChangeColor(u8, u8, u8), // 假设ChangeColor消息带有RGB颜色值  
    // 这里可以继续定义其他消息类型的变体  
}  
  
impl Message {  
    fn call(&self) {  
        println!("{:?}", self);  
    }  
}  
  
fn main() {  
    let quit_message = Message::Quit;  
    let echo_message = Message::Echo("Hello, world!".to_string());  
    let move_message = Message::Move { x: 10, y: 20 };  
    let change_color_message = Message::ChangeColor(255, 0, 0); // 红色  
  
    quit_message.call();  
    echo_message.call();  
    move_message.call();  
    change_color_message.call();  
}