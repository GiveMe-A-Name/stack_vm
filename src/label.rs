use crate::Pointer;
use std::collections::HashMap;

type Label<'a> = (&'a str, Pointer);
pub type Labels<'a> = HashMap<&'a str, Pointer>;

pub fn find_label<'a>(index: Pointer, slice: &Vec<&str>) -> Label<'a> {
    todo!()
}
