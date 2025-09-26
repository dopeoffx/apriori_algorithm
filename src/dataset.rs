use std::collections::HashMap;
use std::hash::Hash;
use std::vec;
use std::cmp::{min, max};

#[derive(Debug)]
struct Row {
    data: Box<[i32]>,
    len: usize,
    min: i32,
    max: i32,
}

#[derive(Debug)]
pub struct Dataset {
    rows: Vec<Row>
}

impl Dataset {
        pub fn from_lines(lines: &[&str]) -> Self {
            let mut rows = Vec::with_capacity(lines.len());
            for &line in lines {
                let mut v: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>()
                    .expect("číslo"))
                    .collect();
                let min = *v.first().unwrap();
                let max = *v.last().unwrap();
                let len = v.len();
                rows.push(Row{data: v.into_boxed_slice(), len, min, max});
                rows.sort_by_key(|r| r.len);
            }
            Self {rows}
        }

}