use dirtytype::Dirty;

#[test]
fn test() {
    let mut value = Dirty::new(5);
    assert!(!value.dirty);
    *value += 2;
    assert!(value.dirty);
}
