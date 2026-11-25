use std::{
    error::{Error, Request, request_ref, request_value},
    fmt::Display,
};

#[derive(Debug)]
struct Foo(usize);

impl Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "foo: {}", self.0)
    }
}

impl Error for Foo {
    fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {
        request.provide_ref(&self.0);
        request.provide_value(self.0);
    }
}

#[test]
fn has() {
    let err = Foo(2);
    assert_eq!(request_ref::<usize>(&err), Some(&2));
    assert_eq!(request_value::<usize>(&err), Some(2));
}
