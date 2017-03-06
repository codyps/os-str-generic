extern crate os_str_generic;
use os_str_generic::OsStrGenericExt;
use std::ffi::{OsStr,OsString};

#[test]
fn eeq_osstr() {
    let mut v = OsStr::new("hi").elements();
    assert_eq!(v.next().unwrap(), OsStr::new("h"));
    assert_eq!(v.next().unwrap(), OsStr::new("i"));
    assert_eq!(v.next(), None);
}

#[test]
fn eeq_char() {
    let mut v = OsStr::new("hi").elements();
    assert_eq!(v.next().unwrap(), 'h');
    assert_eq!(v.next().unwrap(), 'i');
    assert_eq!(v.next(), None)
}

#[test]
fn eeq_u8() {
    let mut v = OsStr::new("hi").elements();
    assert_eq!(v.next().unwrap(), b'h');
    assert_eq!(v.next().unwrap(), b'i');
    assert_eq!(v.next(), None)
}

#[test]
fn starts_with() {
    assert!(OsStr::new("hello world").starts_with("hello"));
    assert!(!OsStr::new("hello world").starts_with("helli"));
    assert!(OsStr::new("hello world").starts_with("hello world"));
    assert!(!OsStr::new("hello world").starts_with("hello world "));
}

#[test]
fn without_prefix() {
    assert_eq!(OsStr::new("hello world").without_prefix("hello "), Some(OsString::from("world")))
}
