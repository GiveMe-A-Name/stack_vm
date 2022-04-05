use std::collections::HashMap;

use crate::Pointer;

// type Procedure<'a> = (&'a str, (Pointer, Pointer));
pub type Procedures<'a> = HashMap<&'a str, (Pointer, Pointer)>;

pub fn find_precedures<'a>(line_slice: &'a [Vec<&str>]) -> Procedures<'a> {
    let mut cursor = 0;
    let mut procedures = Procedures::new();
    while cursor < line_slice.len() {
        if let &["Proc", proc_name] = line_slice[cursor].as_slice() {
            let start_cursor = cursor;
            while line_slice[cursor].as_slice() != ["End"] {
                cursor += 1;
            }
            procedures.insert(proc_name, (start_cursor, cursor));
        } else {
            cursor += 1;
        }
    }
    procedures
}
