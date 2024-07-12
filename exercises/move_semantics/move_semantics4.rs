// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.



fn main() {
    // 初始化一个空的 Vec<i32>
    let vec0: Vec<i32> = Vec::new();

    // 调用 fill_vec 函数来获取一个填充后的 Vec
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // 在 vec1 中添加一个新元素
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// fill_vec 函数不再接收参数
fn fill_vec() -> Vec<i32> {
    // 使用 Vec::new() 初始化一个新的 Vec
    let mut vec = Vec::new();

    // 向 Vec 中添加元素
    vec.push(22);
    vec.push(44);
    vec.push(66);

    // 返回填充后的 Vec
    vec
}
