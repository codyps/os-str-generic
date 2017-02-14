#[deny(unused_imports)]

use std::ffi::OsStr;

#[cfg(windows)]
type Elem = u16;

#[cfg(not(windows))]
type Elem = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
struct OsStrElement {
    inner: Elem 
}

impl<'a> ::std::cmp::PartialEq<&'a OsStr> for OsStrElement {
    fn eq(&self, rhs: & &'a OsStr) -> bool { 
        rhs.len() == 1 && rhs.elements().next().unwrap().inner == self.inner
    }
}

#[cfg(windows)]
#[derive(Clone)]
struct OsStrElementsInner<'a>(::std::os::windows::ffi::EncodeWide<'a>);

#[cfg(windows)]
impl<'a> ::std::fmt::Debug for OsStrElementsInner<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("OsStrElementsInner").finish()
    }
}

#[cfg(not(windows))]
#[derive(Debug, Clone)]
struct OsStrElementsInner<'a>(std::iter::Cloned<std::slice::Iter<'a, u8>>);

impl<'a> OsStrElementsInner<'a> {
    #[cfg(windows)]
    fn from_osstr(i: &'a OsStr) -> OsStrElementsInner<'a> {
        use std::os::windows::ffi::OsStrExt;
        OsStrElementsInner(i.encode_wide())
    }

    #[cfg(not(windows))]
    fn from_osstr(i: &'a OsStr) -> OsStrElementsInner<'a> {
        use std::os::unix::ffi::OsStrExt;
        OsStrElementsInner(i.as_bytes().iter().cloned())
    }
}

#[derive(Debug, Clone)]
struct OsStrElements<'a> {
    inner: OsStrElementsInner<'a>,
}

impl<'a> Iterator for OsStrElements<'a> {
    type Item = OsStrElement;

    fn next(&mut self) -> Option<OsStrElement> {
        self.inner.0.next().map(|x| OsStrElement { inner: x })
    }
}

trait OsStrGenericExt {
    // type Element;
    fn elements(&self) -> OsStrElements;
}

impl OsStrGenericExt for OsStr {
    /*
    /* consider this pattern, perhaps in another trait */
    #[cfg(windows)]
    type Element = u16;
    #[cfg(not(windows))]
    type Element = u8;
    */

    fn elements(&self) -> OsStrElements {
        OsStrElements {
            inner: OsStrElementsInner::from_osstr(self),
        }
    }
}

/*
#[derive(Debug)]
struct OsStrSplit {
}

impl Iterator for OsStrSplit {
    type Item = &OsStr
}

fn split_flags(flags: &OsStr) -> OsStrSplit
{

    for i in flags.elements() {
        if i == ' ' {
            
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::OsStrGenericExt;
    use std::ffi::{OsStr};

    #[test]
    fn elements_eq() {
        let mut v = OsStr::new("hi").elements();
        assert_eq!(v.next().unwrap(), OsStr::new("h"));
        assert_eq!(v.next().unwrap(), OsStr::new("i"));
        assert_eq!(v.next(), None);
    }
}
