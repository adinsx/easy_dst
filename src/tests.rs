use super::Named;

#[derive(Debug)]
struct TestStruct {
    #[allow(dead_code)]
    num: i32,
}
#[test]
fn const_primitive() {
    named!(const FIVE = <i32>(5, "five"));
    assert_eq!(format!("{FIVE:?}"), r#"easy_dst::Named<i32>(5, "five")"#);
}

#[test]
fn const_custom() {
    named!(const FIVE = (TestStruct{num: 5}, "five"));
    assert_eq!(
        format!("{FIVE:?}"),
        r#"easy_dst::Named<easy_dst::tests::TestStruct>(TestStruct { num: 5 }, "five")"#
    );
}

#[test]
fn const_slice() {
    named!(const FIVE = <[u32; _]>([1,2,3,4,5], "five"));
    assert_eq!(
        format!("{FIVE:?}"),
        r#"easy_dst::Named<[u32; 5]>([1, 2, 3, 4, 5], "five")"#
    );
}

#[test]
fn const_ident() {
    const IDENT: TestStruct = TestStruct { num: 5 };
    named!(const FIVE = <TestStruct>(IDENT, "five"));
    assert_eq!(
        format!("{FIVE:?}"),
        r#"easy_dst::Named<easy_dst::tests::TestStruct>(TestStruct { num: 5 }, "five")"#
    );
}

#[test]
#[should_panic]
fn invalid_byte_slice() {
    std::panic::set_hook(Box::new(|_| {}));
    let _not_utf8 = Named::new::<3>(&(5, *b"\xFF\x00\xFF"));
}
