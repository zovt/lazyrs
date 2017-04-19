use super::super::thunk::*;

use std::ops::Add;

#[test]
pub fn test_basic_thunkage() {
    let a = ThunkOnce::new(move || {
        1 + 1
    });
    
    assert_eq!(a.eval_once(), 2)
}

#[test]
pub fn test_map() {
    let a = ThunkOnce::new(move || {
        println!("second");
        8
    }).map(|i| -> String {
        println!("third");
        i.to_string()
    });
    println!("first");
    
    assert_eq!("8".to_string(), a.eval_once())
}
