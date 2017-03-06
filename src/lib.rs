use std::ffi::OsStr;

#[cfg(windows)]
type Elem = u16;

#[cfg(not(windows))]
type Elem = u8;

/// A single "element" of an OsStr. On windows, this corresponds to a `u16`. On unix-like systems, it
/// corresponds to a `u8`
/// 
/// Can be compared to another `OsStrElement` or a `&OsStr`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsStrElement {
    inner: Elem 
}

impl OsStrElement {
    pub fn raw(&self) -> Elem {
        self.inner
    }
}

impl<'a> ::std::cmp::PartialEq<&'a OsStr> for OsStrElement {
    fn eq(&self, rhs: & &'a OsStr) -> bool { 
        rhs.len() == 1 && rhs.elements().next().unwrap().inner == self.inner
    }
}

impl ::std::cmp::PartialEq<char> for OsStrElement {
    fn eq(&self, rhs: &char) -> bool {
        self.inner as u8 == *rhs as u8
    }
}

impl ::std::cmp::PartialEq<u8> for OsStrElement {
    fn eq(&self, rhs: &u8) -> bool {
        self.inner as u8 == *rhs
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

    // fn starts_with(&self, prefix) -> bool
    // fn without_prefix(&self, prefix) -> Option<OsString> {}
    //
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
