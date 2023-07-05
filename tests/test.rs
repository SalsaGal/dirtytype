use dirtytype::Dirty;

#[test]
fn clean_test() {
    let mut values = vec![];
    let mut add = Dirty::new(5);
    let mut add_func = |val: &mut i32| values.push(*val);

    add.clean(&mut add_func);
    *add = 3;
    add.clean(&mut add_func);
    add.clean(&mut add_func);
    *add = 6;
    add.clean(&mut add_func);
    add.clean(&mut add_func);

    dbg!(&values);
    assert!(&[3, 6]
        .into_iter()
        .zip(values.into_iter())
        .all(|(a, b)| a == b))
}

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
