// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]

mod tests {

    use std::option::Option; // 引入Option类型

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // 使用if let来检查optional_target是否为Some类型，并解构出值
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None; range]; // 初始化长度为range的向量，每个元素都是None

        for i in 0..range { // 循环从0开始到range-1
            optional_integers[i] = Some(i as i8); // 直接赋值
        }

        let mut cursor = range as i8 - 1; // 初始化为range-1，因为我们将从最大的数开始比较

        // 使用while let在循环中检查optional_integers.pop()返回的值
        while let Some(Some(integer)) = optional_integers.pop() { // 解构两次Option
            assert_eq!(integer, cursor); // 直接比较integer和cursor
            cursor -= 1;
        }

        assert_eq!(cursor, -1); // 最后cursor应该为-1，因为我们从9开始递减到0
    }
}
