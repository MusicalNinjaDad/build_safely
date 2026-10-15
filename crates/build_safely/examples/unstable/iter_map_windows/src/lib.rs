#![cfg_attr(unstable_iter_map_windows, feature(iter_map_windows))]
#![allow(unused)]

#[cfg(has_iter_map_windows)]
mod has {
    #[test]
    fn has() {
        let strings = "abcd"
            .chars()
            .map_windows(|[x, y]| format!("{}+{}", x, y))
            .collect::<Vec<String>>();

        assert_eq!(strings, vec!["a+b", "b+c", "c+d"]);
    }
}

#[cfg(not(has_iter_map_windows))]
mod has_not {
    /// ```compile_fail
    /// let strings = "abcd"
    ///     .chars()
    ///     .map_windows(|[x, y]| format!("{}+{}", x, y))
    ///     .collect::<Vec<String>>();
    ///
    /// assert_eq!(strings, vec!["a+b", "b+c", "c+d"]);
    /// ```
    fn doctest() {}
}
