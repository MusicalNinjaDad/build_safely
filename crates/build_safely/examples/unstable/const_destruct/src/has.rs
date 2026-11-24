use std::marker::Destruct;

struct Foo;

impl Foo {
    const fn wibble<N>(val: N) -> N
    where
        N: [const] Destruct,
    {
        val
    }
}

#[test]
fn has() {
    const FOO_5: u8 = Foo::wibble(5_u8);
    const {
        assert!(FOO_5 == 5_u8);
    }
}
