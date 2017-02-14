extern crate os_str_generic;
use os_str_generic::OsStrGenericExt;
use std::ffi::{OsStr};

#[test]
fn elements_eq() {
    let mut v = OsStr::new("hi").elements();
    assert_eq!(v.next().unwrap(), OsStr::new("h"));
    assert_eq!(v.next().unwrap(), OsStr::new("i"));
    assert_eq!(v.next(), None);
}
