

#[test]
fn test_define() {
    use iron_macros::define;

    define!(hello, 5.0);

    assert_eq!(hello!(), 5.0);
}