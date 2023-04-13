use dirtytype::Dirty;

#[test]
fn test() {
    let mut value = Dirty::Clean(5);
    assert!(value.is_clean());
    *value += 2;
    assert!(value.is_dirty());
    assert_eq!(value, Dirty::Dirty(7));
    value.clear();
    assert_eq!(value, Dirty::Clean(7));
    println!("{}", *value);
    assert!(value.is_clean());
}
