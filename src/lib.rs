use std::ffi::{OsStr, OsString};
use std::iter::FromIterator;
use std::ascii::AsciiExt;

#[cfg(windows)]
type Elem = u16;

#[cfg(not(windows))]
type Elem = u8;

/// A single "element" of an OsStr. On windows, this corresponds to a `u16`. On unix-like systems, it
/// corresponds to a `u8`
///
/// Can be compared to another `OsStrElement` or a `&OsStr`.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct OsStrElement {
    inner: Elem
}

impl OsStrElement {
    pub fn raw(&self) -> Elem {
        self.inner
    }

    #[cfg(windows)]
    fn as_os_string_inner(&self) -> OsString {
        use std::os::windows::ffi::OsStringExt;
        let v = [self.inner];
        OsString::from_wide(&v[..])
    }

    #[cfg(not(windows))]
    fn as_os_string_inner(&self) -> OsString {
        use std::os::unix::ffi::OsStringExt;
        let v = vec![self.inner];
        OsString::from_vec(v)
    }

    pub fn as_os_string(&self) -> OsString {
        self.as_os_string_inner()
    }
}

impl<'a> ::std::cmp::PartialEq<&'a OsStr> for OsStrElement {
    fn eq(&self, rhs: & &'a OsStr) -> bool {
        rhs.len() == 1 && rhs.elements().next().unwrap().inner == self.inner
    }
}

impl ::std::cmp::PartialEq<char> for OsStrElement {
    fn eq(&self, rhs: &char) -> bool {
        rhs.is_ascii() && self.inner as u8 == *rhs as u8
    }
}

impl ::std::cmp::PartialEq<u8> for OsStrElement {
    fn eq(&self, rhs: &u8) -> bool {
        self.inner == self.inner as u8 as Elem && self.inner as u8 == *rhs
    }
}

impl FromIterator<OsStrElement> for OsString {
    fn from_iter<T>(iter: T) -> OsString
        where T: IntoIterator<Item=OsStrElement>
    {
        let mut s = OsString::new();
        for i in iter.into_iter() {
            s.push(i.as_os_string())
        }

        s
    }
}

/*
impl<'a> ::std::cmp::PartialEq<&'a str> for OsStrElement {
    fn eq(&self, rhs: & &'a str) -> bool {
        rhs.len() == 1 && rhs
    }
}
*/

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

/// Iterator over the elements of a `&OsStr`
#[derive(Debug, Clone)]
pub struct OsStrElements<'a> {
    inner: OsStrElementsInner<'a>,
}

impl<'a> Iterator for OsStrElements<'a> {
    type Item = OsStrElement;

    fn next(&mut self) -> Option<OsStrElement> {
        self.inner.0.next().map(|x| OsStrElement { inner: x })
    }
}

/// Extentions to OsStr that allow working with them without filling code with `#[cfg(...)]`
pub trait OsStrGenericExt {
    // type Element;

    /// Iterate over the smallest elements of an OsStr. Element meaning is dependent on specific
    /// OS.
    fn elements(&self) -> OsStrElements;

    fn starts_with<T: AsRef<OsStr>>(&self, prefix: T) -> bool {
        let mut ei = self.elements();
        let mut pi = prefix.as_ref().elements();
        loop {
            let e = ei.next();
            let p = pi.next();
            match (e, p) {
                (Some(_), None) => return true,
                (None, None) => return true,
                (None, Some(_)) => return false,
                (Some(c1), Some(c2)) => if c1 != c2 { return false }
            }
        }
    }

    fn without_prefix<T: AsRef<OsStr>>(&self, prefix: T) -> Option<OsString> {
        let mut ei = self.elements().peekable();
        let mut pi = prefix.as_ref().elements();
        loop {
            {
                let e = ei.peek();
                let p = pi.next();
                match (e, p) {
                    (Some(_), None) => break,
                    (None, None) => break,
                    (None, Some(_)) => return None,
                    (Some(c1), Some(c2)) => if c1.clone() != c2 { return None }
                }
            }

            ei.next();
        }

        Some(ei.collect())
    }
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
