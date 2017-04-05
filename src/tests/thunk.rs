use super::super::thunk::*;

#[test]
pub fn test_basic_thunkage() {
    let mut a = Thunk::new(move || {
        8
    });
    assert!(*a.eval() == 8)
}
