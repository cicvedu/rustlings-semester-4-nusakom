// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {  
    println!("{}", arg);  
}  
  
fn string(arg: String) {  
    println!("{}", arg);  
}  
  
fn main() {  
    // "blue" 是一个字符串切片，所以调用 string_slice  
    string_slice("blue");  
  
    // "red".to_string() 转换得到一个 String，所以调用 string  
    string("red".to_string());  
  
    // String::from("hi") 创建了一个 String，所以调用 string  
    string(String::from("hi"));  
  
    // "rust is fun!".to_owned() 是 "rust is fun!" 字符串切片的 owned 版本，即 String，所以调用 string  
    string("rust is fun!".to_owned());  
  
    // "nice weather".into() 将 &str 转换为 String，所以调用 string  
    string("nice weather".into());  
  
    // format!("Interpolation {}", "Station") 返回一个 String，所以调用 string  
    string(format!("Interpolation {}", "Station"));  
  
    // &String::from("abc")[0..1] 是一个字符串切片的引用，所以调用 string_slice  
    string_slice(&String::from("abc")[0..1]);  
  
    // "  hello there ".trim() 返回去除空格后的字符串切片，所以调用 string_slice  
    string_slice("  hello there ".trim());  
  
    // "Happy Monday!".to_string().replace("Mon", "Tues") 返回一个新的 String，所以调用 string  
    string("Happy Monday!".to_string().replace("Mon", "Tues"));  
  
    // "mY sHiFt KeY iS sTickY".to_lowercase() 返回小写字母的 String，所以调用 string  
    string("mY sHiFt KeY iS sTickY".to_lowercase());  
}