#![cfg_attr(unstable_proc_macro_diagnostic, feature(proc_macro_diagnostic))]
#![allow(unused)]

extern crate proc_macro;

#[cfg(has_proc_macro_diagnostic)]
mod has {
    use proc_macro::{Diagnostic, Level};

    #[test]
    fn has() {
        let d = Diagnostic::new(Level::Warning, "beware");
        assert!(matches!(d.level(), Level::Warning));
        assert_eq!(d.message(), "beware");
    }
}

#[cfg(not(has_proc_macro_diagnostic))]
mod has_not {
    /// ```compile_fail
    /// use proc_macro::{Diagnostic, Level};
    /// let d = Diagnostic::new(Level::Warning, "beware");
    /// ```
    fn doctest() {}
}
