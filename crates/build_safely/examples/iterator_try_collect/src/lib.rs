#![cfg_attr(unstable_iterator_try_collect, feature(iterator_try_collect))]

#[cfg(test)]
#[cfg(has_iterator_try_collect)]
mod tests {
    #[test]
    fn has() {
        let _: Option<Vec<_>> = std::iter::Iterator::try_collect(&mut vec![Some(1)].into_iter());
    }
}

#[cfg(test)]
#[cfg(not(has_iterator_try_collect))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!(vec![1], vec![1]);
    }
}
