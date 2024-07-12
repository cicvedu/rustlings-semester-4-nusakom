// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;  
  
#[derive(Clone, Copy, PartialEq, Eq)]  
enum Progress {  
    None,  
    Some,  
    Complete,  
}  
  
fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {  
    let mut count = 0;  
    for val in map.values() {  
        if *val == value { 
            count += 1;  
        }  
    }  
    count  
}  
  
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {  
    map.values().filter(|&v| *v == value).count()  
}  
  
fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {  
    let mut count = 0;  
    for map in collection {  
        count += count_for(map, value);  
    }  
    count  
}  
  
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {   
    collection.iter().flat_map(|map| map.values()).filter(|&v| *v == value).count()  
}  
  
#[cfg(test)]  
mod tests {  
    use super::*;  
  
    fn get_map() -> HashMap<String, Progress> {   
        return HashMap::from([  
            ("variables1".to_string(), Progress::Complete),  
            ("from_str".to_string(), Progress::None),  
            ("another_one".to_string(), Progress::Some),  
            ("done".to_string(), Progress::Complete),  
            ("not_done".to_string(), Progress::None),    
        ]);  
    }  
  
    #[test]  
    fn count_complete() {  
        let map = get_map();  
        assert_eq!(2, count_iterator(&map, Progress::Complete)); // 假设 Complete 有两个  
    }  
  
    #[test]  
    fn count_some() {  
        let map = get_map();  
        assert_eq!(1, count_iterator(&map, Progress::Some)); // 假设 Some 有一个  
    }  
  
    #[test]  
    fn count_none() {  
        let map = get_map();  
        assert_eq!(2, count_iterator(&map, Progress::None)); // 假设 None 有两个  
    }  
  
    #[test]  
    fn count_complete_equals_for() {  
        let map = get_map();  
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];  
        for progress_state in progress_states {  
            assert_eq!(  
                count_for(&map, progress_state),  
                count_iterator(&map, progress_state)  
            );  
        }  
    }  
}