struct Foo(usize);
impl Foo {
    const fn new<N>(val: N) -> Self
    where
        usize: [const] From<N>,
    {
        Self(val.into())
    }
}

#[test]
fn has() {
    const FOO_5: Foo = Foo::new(5_u8);
    const {
        assert!(FOO_5.0 == 5);
    }
}
