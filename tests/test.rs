use dirtytype::Dirty;

#[test]
fn test() {
    let mut value = Dirty::Clean(5);
    assert!(value.is_clean());
    *value += 2;
    assert!(value.is_dirty());
}
