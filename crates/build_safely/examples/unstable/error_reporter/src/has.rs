use std::{
    error::{Error, Report},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MyErr(MyErr2);

impl Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "myerr")
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
        write!(f, "myerr2: {}", self.0)
    }
}

impl Error for MyErr2 {}

#[test]
fn has() {
    let inner = MyErr2("oops");
    let outer = MyErr(inner);
    let message = Report::new(outer);
    assert_eq!(message.to_string(), "myerr: myerr2: oops");
}
