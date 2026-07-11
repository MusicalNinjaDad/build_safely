#![cfg_attr(unstable_bool_to_result, feature(bool_to_result))]

#[cfg(test)]
#[cfg(has_bool_to_result)]
mod tests {
    #[test]
    fn has() {
        let _ = true.ok_or(());
        let _ = false.ok_or_else(|| ());
    }
}

#[cfg(test)]
#[cfg(not(has_bool_to_result))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!(true, true);
    }
}
