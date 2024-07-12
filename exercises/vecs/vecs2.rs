// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.



fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element *= 2; // Multiply each element by 2
    }

    // At this point, `v` will contain each element multiplied by 2.
    v
}

fn main() {
    let v = vec![2, 4, 6, 8, 10];
    let result = vec_loop(v);
    println!("{:?}", result); // Output: [4, 8, 12, 16, 20]
}
