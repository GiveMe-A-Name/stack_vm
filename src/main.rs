use std::fs::File;
use std::io::{self, Read};

use bytecode_vm::label::{find_label, Labels};
use bytecode_vm::procedure::{find_precedures, Procedures};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut contents = String::new();
    let mut file = File::open(&args[1])?;
    file.read_to_string(&mut contents)?;

    let line_slice: Vec<_> = contents
        .split("\n")
        .filter_map(|line| filter_map_line(line))
        .collect();

    let labels: Labels = line_slice
        .iter()
        .enumerate()
        .filter_map(|(index, slice)| find_label(index, slice))
        .collect();
    let procedures: Procedures = find_precedures(&line_slice);

    Ok(())
}

/// 过滤空行和注释行，将筛选后的行转化为vec
///
fn filter_map_line(line: &str) -> Option<Vec<&str>> {
    let v = line.split_whitespace().collect::<Vec<_>>();
    if matches!(v.as_slice(), [] | ["--", ..]) {
        return None;
    }
    Some(v)
}
