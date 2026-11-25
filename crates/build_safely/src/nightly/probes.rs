use super::{AutoCfg, UnstableFeature};

pub mod const_bool;
pub mod const_cmp;
pub mod const_convert;
pub mod const_destruct;
pub mod const_option_ops;
pub mod const_result_trait_fn;
pub mod const_try;
pub mod error_generic_member_access;
pub mod error_iter;
pub mod error_reporter;
pub mod fn_traits;
pub mod offset_of_slice;
pub mod ptr_metadata;
pub mod raw_os_error_ty;
pub mod structural_match;
pub mod trait_alias;

/// Prefix with:
/// - #![allow(stable_features)] (only if allowed)
/// - #![feature()] (only if allowed)
/// - #![allow(unused)] (always)
pub fn make_probe(feature: &UnstableFeature, allowed: bool, probe: &str) -> String {
    let mut _probe = String::with_capacity(256);
    if allowed {
        _probe.push('\n');
        _probe.push_str("#![allow(stable_features)]");
        _probe.push('\n');

        _probe.push_str("#![feature(");
        _probe.push_str(&feature.to_string());
        _probe.push_str(")]");
        _probe.push('\n');
    };
    _probe.push_str("#![allow(unused)]");
    _probe.push('\n');
    _probe.push_str(probe);
    _probe
}

/// Register `#[cfg(has_feature)]` & set based on the probe
pub fn has(ac: &AutoCfg, feature: &UnstableFeature, allowed: bool, probe: &str) -> bool {
    let cfg = format!("has_{feature}");
    autocfg::emit_possibility(&cfg);
    let code = make_probe(feature, allowed, probe);
    if ac.probe_raw(&code).is_ok() {
        autocfg::emit(&cfg);
        true
    } else {
        false
    }
}

/// Register `#[cfg(has_feature)]` & run a default probe
pub fn unstable(
    ac: &AutoCfg,
    feature: &UnstableFeature,
    allowed: bool,
    extra_lines: Option<&str>,
) -> bool {
    let cfg = format!("unstable_{feature}");
    autocfg::emit_possibility(&cfg);

    let mut code = format!(
        r#"
#![deny(stable_features)]
#![feature({feature})]
#![allow(unused)]
"#
    );
    if let Some(extra_lines) = extra_lines {
        code.push('\n');
        code.push_str(extra_lines);
        code.push('\n');
    }

    if allowed && ac.probe_raw(&code).is_ok() {
        autocfg::emit(&cfg);
        true
    } else {
        false
    }
}

pub mod adt_const_params {
    pub const AVAILABLE: &str = r#"
use std::marker::ConstParamTy;
#[derive(ConstParamTy, PartialEq, Eq)]
struct Increment(i32);
struct Counter<const INC: Increment>(i32);
"#;
}

pub mod assert_matches {
    pub const AVAILABLE: &str = r#"
use std::assert_matches;
"#;
    pub const ROOT: &str = r#"
use std::assert_matches;

fn main() {
    assert_matches!(Some(4), Some(_));
}
"#;
    // was stabilised in root - so no need to remove feature from this probe
    pub const MODULE: &str = r#"
#![allow(stable_features)]
#![feature(assert_matches)]
use std::assert_matches::assert_matches;

fn main() {
    assert_matches!(Some(4), Some(_));
}
"#;
}

pub mod bool_to_result {
    pub const AVAILABLE: &str = r#"
fn main() {
    let _ = true.ok_or(());
}
"#;
}

pub mod can_vector {
    pub const AVAILABLE: &str = r#"
use std::io::Read;
fn main() {
    std::io::empty().is_read_vectored();
}
"#;
}

pub mod const_ops {
    // requires: feature(const_trait_impl)
    // #![allow(stable_features)] may be duplicated by make_probe
    pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]        
#![feature(const_trait_impl)]
use std::ops::Add;
struct Left(u32);
struct Right(u32);

const impl Add<Right> for Left {
    type Output = Self;

    fn add(self, rhs: Right) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
"#;
}

pub mod default_field_values {
    pub const AVAILABLE: &str = r#"
struct Counter {
    inner: usize = 1,
}
"#;
}

pub mod const_trait_impl {
    pub const AVAILABLE: &str = r#"
struct Thing;
const trait Foo {}
const impl Foo for Thing {}
"#;
}

pub mod doc_notable_trait {
    pub const AVAILABLE: &str = r#"
#[doc(notable_trait)]
trait Foo {}
"#;
}

pub mod exact_div {
    pub const AVAILABLE: &str = r#"
fn half(n: usize) -> Option<usize> {
    n.div_exact(2)
}
"#;
}

pub mod exact_size_is_empty {
    pub const AVAILABLE: &str = r#"
struct Empty;

impl Iterator for Empty {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl ExactSizeIterator for Empty {
    fn len(&self) -> usize {
        0
    }
    fn is_empty(&self) -> bool {
        true
    }
}

fn empty() {
    let x = Empty;
    let _ = x.is_empty();
}
"#;
}

pub mod integer_cast_extras {
    pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]
#![feature(integer_casts)]
fn integer_cast_extras() {
    let _: u32 = 0_i32.strict_cast_unsigned();
}
"#;
}

pub mod integer_casts {
    pub const AVAILABLE: &str = r#"
fn integer_casts() {
    let _: u32 = 0_usize.strict_cast();
}
"#;
}

pub mod iter_array_chunks {
    pub const AVAILABLE: &str = r#"
fn iter_array_chunks() {
    let x = [0,1,2,3,4];
    let _ = x.iter().array_chunks::<2>();
}
"#;
}

pub mod iter_next_chunk {
    pub const AVAILABLE: &str = r#"
fn iter_next_chunk() {
    let x = [0,1,2,3,4];
    let _ = x.iter().next_chunk::<2>();
    let _ = x.iter().next_chunk_back::<2>();
}
"#;
}

pub mod iter_map_windows {
    pub const AVAILABLE: &str = r#"
fn iter_map_windows() {
    let _ = "abcd".chars().map_windows(|[x, y]| format!("{}+{}", x, y));
}
"#;
}

pub mod iterator_try_collect {
    // vec! not array: https://internals.rust-lang.org/t/code-compiles-on-playground-but-fails-when-passed-via-stdin-to-rustc/24393
    pub const AVAILABLE: &str = r#"
fn try_collect() {
    let _: Option<Vec<_>> = std::iter::Iterator::try_collect(&mut vec![Some(1)].into_iter());
}
"#;
}

pub mod negative_impls {
    pub const AVAILABLE: &str = r#"
struct Prisoner;
impl !Send for Prisoner {}
"#;
}

pub mod never_type {
    pub const AVAILABLE: &str = r#"
type Bang = !;
"#;
}

pub mod os_str_split_at {
    pub const AVAILABLE: &str = r#"
fn os_str_split_at() {
    use std::ffi::OsStr;
    let os_str = OsStr::new("foo");
    let (_f, _oo) = os_str.split_at(1);
}
"#;
}

pub mod path_absolute_method {
    pub const AVAILABLE: &str = r#"
use std::path::PathBuf;
fn path_absolute_method() {
    let _ = PathBuf::new().absolute();
}
"#;
}

pub mod proc_macro_diagnostic {
    pub const AVAILABLE: &str = r#"
extern crate proc_macro;
use proc_macro::Diagnostic;      
"#;
}

pub mod strip_circumfix {
    pub const AVAILABLE: &str = r#"
fn main() {
    let s = "foo";
    let _ = s.strip_circumfix("f", "o");
}
"#;
}

pub mod try_blocks {
    pub const AVAILABLE: &str = r#"
fn try_blocks() {
    let _ = try {
        let _ = Err(5_u8)?;
        let _ = Ok(6_u16)?;
    };
}
"#;
}

pub mod try_blocks_heterogeneous {
    pub const AVAILABLE: &str = r#"
fn try_blocks_heterogeneous() {
    let _ = try bikeshed Result<_, u16> {
        let _ = Err(5_u8)?;
        let _ = Err(6_u16)?;
    };
}
"#;
}

pub mod try_trait_v2 {
    pub const AVAILABLE: &str = r#"
use std::ops::Try;
"#;
}

pub mod try_trait_v2_residual {
    pub const AVAILABLE: &str = r#"
use std::ops::Residual;
"#;
}

pub mod unsized_const_params {
    // requires: feature(adt_const_params), allow(incomplete_features)
    // #![allow(stable_features)] may be duplicated by make_probe
    pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
struct Foo<const N: &'static str>;
"#;
}

pub mod write_all_vectored {
    pub const AVAILABLE: &str = r#"
use std::io::{empty, Write, IoSlice};
fn main() {
    let buf: [u8;_] = [0];
    let slice = IoSlice::new(&buf);
    empty().write_all_vectored(&mut [slice]);
}
"#;
}
