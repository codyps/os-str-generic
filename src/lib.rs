use std::ffi::{OsString, OsStr};

#[cfg(windows)]
type Elem = u16;

#[cfg(not(windows))]
type Elem = u8;

#[derive(Debug, Clone)]
struct OsStrElement {
    inner: Elem 
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
struct OsStrElementsInner<'a>(std::slice::Iter<'a, u8>);

impl<'a> OsStrElementsInner<'a> {
    #[cfg(windows)]
    fn from_osstr(i: &'a OsStr) -> OsStrElementsInner<'a> {
        use std::os::windows::ffi::OsStrExt;
        OsStrElementsInner(i.encode_wide())
    }

    #[cfg(not(windows))]
    fn from_osstr(i: &'a OsStr) -> OsStrElementsInner<'a> {
        use std::os::unix::ffi::OsStrExt;
        OsStrElementsInner(i.as_bytes().iter())
    }
}

#[derive(Debug, Clone)]
struct OsStrElements<'a> {
    inner: OsStrElementsInner<'a>,
}

impl<'a> Iterator for OsStrElements<'a> {
    type Item = OsStrElement;

    fn next(&mut self) -> Option<OsStrElement> {
        None     
    }
}

trait OsStrGenericExt {
    fn elements(&self) -> OsStrElements;
}

impl OsStrGenericExt for OsStr {
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
    #[test]
    fn it_works() {
    }
}
