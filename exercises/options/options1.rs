// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.



// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
    pub enum Command {  
        Uppercase,  
        Trim,  
        Append(usize),  
    }  
      
    mod my_module {  
        use super::Command;  
      
        pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {  
            let mut output: Vec<String> = Vec::new();  
      
            for (string, command) in input.into_iter() {  
                let mut processed_string = string;  
                match command {  
                    Command::Uppercase => processed_string = processed_string.to_uppercase(),  
                    Command::Trim => processed_string = processed_string.trim().to_string(),  
                    Command::Append(n) => {  
                        let append_str = "a".repeat(n);  
                        processed_string.push_str(&append_str);  
                    }  
                }  
                output.push(processed_string);  
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
            assert_eq!(output[2], "fooa");  
            assert_eq!(output[3], "baraaaaa");  
        }  
    }