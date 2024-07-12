// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.



mod sausage_factory {  
    // 这个函数只在模块内部可见  
    fn get_secret_recipe() -> String {  
        String::from("Ginger")  
    }  
  
    // 这个函数现在可以被模块外部调用  
    pub fn make_sausage() {  
        let recipe = get_secret_recipe();  
        println!("Secret recipe: {}", recipe);  
        println!("sausage!");  
    }  
}  
  
fn main() {  
    sausage_factory::make_sausage();  
}