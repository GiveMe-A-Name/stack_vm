use std::collections::HashMap;

use crate::Pointer;

// type Procedure<'a> = (&'a str, (Pointer, Pointer));
pub type Procedures<'a> = HashMap<&'a str, (Pointer, Pointer)>;

pub fn find_precedures<'a>(line_slice: &'a [Vec<&str>]) -> Procedures<'a> {
    todo!()
}
