// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;  
  
#[derive(Hash, PartialEq, Eq)]  
enum Fruit {  
    Apple,  
    Banana,  
    Mango,  
    Lychee,  
    Pineapple,  
}  
  
fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {  
    let fruit_kinds = vec![  
        Fruit::Banana,  
        Fruit::Pineapple,  
    ];  
  
    for fruit in fruit_kinds {  
        if !basket.contains_key(&fruit) {  
            basket.insert(fruit, 1); // 插入新的水果种类，数量为1  
        }  
    }  
}  
  
#[cfg(test)]  
mod tests {  
    use super::*;  
  
    fn get_fruit_basket() -> HashMap<Fruit, u32> {  
        let mut basket = HashMap::<Fruit, u32>::new();  
        basket.insert(Fruit::Apple, 4);  
        basket.insert(Fruit::Mango, 2);  
        basket.insert(Fruit::Lychee, 5);  
        basket  
    }  
  
    #[test]  
    fn test_given_fruits_are_not_modified() {  
        let mut basket = get_fruit_basket();  
        fruit_basket(&mut basket);  
        assert_eq!(basket[&Fruit::Apple], 4);  
        assert_eq!(basket[&Fruit::Mango], 2);  
        assert_eq!(basket[&Fruit::Lychee], 5);  
    }  
  
    #[test]  
    fn at_least_five_types_of_fruits() {  
        let mut basket = get_fruit_basket();  
        fruit_basket(&mut basket);  
        assert!(basket.len() >= 5);  
    }  
  
    #[test]  
    fn greater_than_eleven_fruits() {  
        let mut basket = get_fruit_basket();  
        fruit_basket(&mut basket);  
        let count = basket.values().sum::<u32>();  
        assert!(count > 11);  
    }  
  
    #[test]  
    fn all_fruit_types_in_basket() {  
        let mut basket = get_fruit_basket();  
        fruit_basket(&mut basket);  
        for &amount in basket.values() {  
            assert_ne!(amount, 0);  
        }  
    }  
}
