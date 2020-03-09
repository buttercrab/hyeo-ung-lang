use crate::code;

fn make_indent(value: usize) -> String {
    std::iter::repeat(' ').take(value * 4).collect::<String>()
}

fn fn_print(indent: usize, s: String) -> String {
    format!("\n{}print!({:?});", make_indent(indent), s)
}

pub fn build_source<T>(mut state: T, code: &Vec<T::CodeType>, level: usize) -> String
where
    T: code::State,
{
    let opt = level != 0;
    let mut res = String::from(format!(
        "{}{}{}{}{}",
        "\
use hyeong_build::number::Num;
use std::collections::HashMap;

struct Stack {
    data: ",
        if opt { "Vec<" } else { "HashMap<usize, " },
        "Vec<Num>>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            data: ",
        if opt { "Vec" } else { "HashMap" },
        "::new(),
        }
    }

    fn pop(&mut self, idx: usize) -> Num {
        unimplemented!()
    }

    fn push(&mut self, idx: usize, num: Num) {
        unimplemented!()
    }
}

fn main() {
    let mut stack = Stack::new();
    let mut point: HashMap<u128, usize> = HashMap::new();
    let mut state = 0usize;
    let mut last = 0usize;
",
    ));

    let mut indent = 1usize;

    if !state.get_stack(1).is_empty() {
        res.push_str(&*fn_print(
            indent,
            state
                .get_stack(1)
                .iter()
                .map(|num| num.floor().to_int() as u8 as char)
                .collect(),
        ));
        state.get_stack(1).clear();
    }

    if !state.get_stack(2).is_empty() {
        res.push_str(&*fn_print(
            indent,
            state
                .get_stack(2)
                .iter()
                .map(|num| num.floor().to_int() as u8 as char)
                .collect(),
        ));
        state.get_stack(2).clear();
    }

    // do something

    res.push_str(
        "
}",
    );
    res
}
