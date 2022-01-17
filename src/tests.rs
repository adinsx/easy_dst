use super::Named;
#[test]
fn const_primitive() {
    named!(const FIVE = (i32(5), b"five"));
    assert_eq!(format!("{FIVE:?}"), r#"easy_dst::Named<i32>(5, "five")"#);
}

#[test]
fn const_custom() {
    #[derive(Debug)]
    struct TestStruct {
        #[allow(dead_code)]
        num: i32,
    }
    named!(const FIVE = (TestStruct (TestStruct{num: 5}), b"five"));
    assert_eq!(
        format!("{FIVE:?}"),
        r#"easy_dst::Named<easy_dst::tests::const_custom::TestStruct>(TestStruct { num: 5 }, "five")"#
    );
}

#[test]
#[should_panic]
fn invalid_byte_slice() {
    std::panic::set_hook(Box::new(|_| {}));
    let _not_utf8 = Named::new::<3>(&(5, *b"\xFF\x00\xFF"));
}
