

#[test]
fn test_define() {
    use sea_macros::define;

    define!(hello, 5.0);

    assert_eq!(hello!(), 5.0);
}

#[test]
fn test_c_loop() {
    use sea_macros::c_loop;

    c_loop!(i = 0; i > 5; i += 1; {
        dbg!(i);
        println!("{}", i);
    });
}