use super::Named;
#[test]
fn const_primitive() {
    named!(const FIVE = (i32(5), b"five"));
    assert_eq!(format!("{FIVE:?}"), r#"easy_dst::Named<i32>(5, "five")"#);
}

#[test]
#[should_panic]
fn invalid_byte_slice() {
    std::panic::set_hook(Box::new(|_| {}));
    let _not_utf8 = Named::new::<3>(&(5, *b"\xFF\x00\xFF"));
}
