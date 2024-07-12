// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    // 不获取所有权，只是借用
    let last_char = get_char(&data);
    println!("Last character: {}", last_char);

    // 获取所有权并变更字符串
    let uppercased_data = string_uppercase(data);
    println!("Uppercased: {}", uppercased_data);
}

// 不应获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 应获取所有权
fn string_uppercase(mut data: String) -> String {
    data = data.to_uppercase();
    data
}
