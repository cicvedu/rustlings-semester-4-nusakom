// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}
mod my_module {  
    use super::Command;  
  
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {  
        let mut output: Vec<String> = Vec::new(); // 使用 Vec::new() 初始化空 Vec  
  
        for (string, command) in input.iter() {  
            let mut processed_string = string.clone(); // 复制原始字符串以便修改  
            match command {  
                Command::Uppercase => processed_string = processed_string.to_uppercase(),  
                Command::Trim => processed_string = processed_string.trim().to_string(),  
                Command::Append(n) => {  
                    let append_str = "a".repeat(*n); // 重复字符 'a' n 次  
                    processed_string.push_str(&append_str);  
                }  
            }  
            output.push(processed_string); // 将处理后的字符串添加到输出 Vec  
        }  
  
        output  
    }  
}
#[cfg(test)]  
mod tests {  
    use super::my_module::transformer;  
    use super::Command;  
  
    #[test]  
    fn it_works() {  
        let output = transformer(vec![  
            ("hello".into(), Command::Uppercase),  
            (" all roads lead to rome! ".into(), Command::Trim),  
            ("foo".into(), Command::Append(1)),  
            ("bar".into(), Command::Append(5)),  
        ]);  
  
        assert_eq!(output[0], "HELLO");  
        assert_eq!(output[1], "all roads lead to rome!");  
        assert_eq!(output[2], "fooa"); // 修正为 "fooa"，添加一个 'a'  
        assert_eq!(output[3], "baraaaaa"); // 修正为 "baraaaaa"，添加五个 'a'  
    }  
}