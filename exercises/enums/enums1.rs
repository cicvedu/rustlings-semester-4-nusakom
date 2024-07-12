// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]  
  
enum Message {  
    Quit,  
    Echo,  
    Move,  
    ChangeColor,  
    // 这里可以定义更多的消息类型  
}  
  
fn main() {  
    println!("{:?}", Message::Quit);  
    println!("{:?}", Message::Echo);  
    println!("{:?}", Message::Move);  
    println!("{:?}", Message::ChangeColor);  
}