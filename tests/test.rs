use dirtytype::Dirty;

#[test]
fn test() {
    let mut value = Dirty::new(5);
    assert!(!value.dirty);
    *value += 2;
    assert!(value.dirty);
    assert_eq!(*value, 7);

    let mut name = Dirty::new("Foo".to_owned());
    name.push_str(" Bar");
    assert_eq!(*name, "Foo Bar");
}
