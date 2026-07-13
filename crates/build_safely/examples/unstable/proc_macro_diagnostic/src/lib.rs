#![cfg_attr(unstable_proc_macro_diagnostic, feature(proc_macro_diagnostic))]
#![allow(unused)]

extern crate proc_macro;

#[cfg(has_proc_macro_diagnostic)]
mod has {
    use proc_macro::Diagnostic;

    #[test]
    fn has() {
        // This test just verifies that the Diagnostic type is available
        // We can't really test proc_macro functionality in a test
    }
}

#[cfg(not(has_proc_macro_diagnostic))]
mod has_not {
    /// ```compile_fail
    /// use proc_macro::Diagnostic;
    /// ```
    #[test]
    fn has_not() {}
}
