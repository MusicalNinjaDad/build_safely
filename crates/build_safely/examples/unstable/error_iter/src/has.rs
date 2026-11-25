use std::{error::Error, fmt::Display, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MyErr(MyErr2);

impl Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for MyErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MyErr2(&'static str);

impl Display for MyErr2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for MyErr2 {}

#[test]
fn has() {
    let inner = MyErr2("oops");
    let outer = MyErr(inner);
    let e = &outer as &dyn Error;
    let inner_source = e.sources().find_map(|err| err.downcast_ref::<MyErr2>());
    assert_eq!(inner_source, Some(&inner));
}
