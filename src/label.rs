use crate::Pointer;
use std::collections::HashMap;

type Label<'a> = (&'a str, Pointer);
pub type Labels<'a> = HashMap<&'a str, Pointer>;

/// find label from line's
///
/// Example
/// ["label", "loop"] => (loop, index);
pub fn find_label<'a>(index: Pointer, slice: &'a Vec<&str>) -> Option<Label<'a>> {
    if let &["label", label_name] = slice.as_slice() {
        Some((label_name, index))
    } else {
        None
    }
}
