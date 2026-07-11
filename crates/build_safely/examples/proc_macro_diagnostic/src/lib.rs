#![cfg_attr(unstable_proc_macro_diagnostic, feature(proc_macro_diagnostic))]

extern crate proc_macro;

#[cfg(test)]
#[cfg(has_proc_macro_diagnostic)]
mod tests {
    use proc_macro::Diagnostic;

    #[test]
    fn has() {
        // This test just verifies that the Diagnostic type is available
        // We can't really test proc_macro functionality in a test
    }
}

#[cfg(test)]
#[cfg(not(has_proc_macro_diagnostic))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!(0, 0);
    }
}
