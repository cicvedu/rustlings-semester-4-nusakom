// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.


mod delicious_snacks {  
  
    // 导入fruits模块中的PEAR常量  
    pub use self::fruits::PEAR;  
  
    // 导入veggies模块中的CUCUMBER常量  
    pub use self::veggies::CUCUMBER;  
  
    mod fruits {  
        // 水果常量  
        pub const PEAR: &'static str = "Pear";  
        pub const APPLE: &'static str = "Apple";  
    }  
  
    mod veggies {  
        // 蔬菜常量  
        pub const CUCUMBER: &'static str = "Cucumber";  
        pub const CARROT: &'static str = "Carrot";  
    }  
  
}  
  
fn main() {  
    // 使用use语句导入delicious_snacks模块中的PEAR和CUCUMBER常量  
    use delicious_snacks::PEAR;  
    use delicious_snacks::CUCUMBER;  
  
    // 打印最喜欢的零食  
    println!(  
        "favorite snacks: {} and {}",  
        PEAR, // 这是水果  
        CUCUMBER // 这是蔬菜  
    );  
}