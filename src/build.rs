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
        "{}{}{}{}{}{}{}{}{}",
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
        if opt {
            format!("vec![Vec::new(); {}]", state.stack_size())
        } else {
            "HashMap::new()".to_string()
        },
        ",
        }
    }

    fn pop(&mut self, idx: usize) -> Num {
        ",
        if opt {
            "if idx < self.data.len() {
            match self.data[idx].pop() {
                Some(n) => n,
                None => {
                    if idx == 0 {
                        let mut s = String::new();
                        std::io::stdin().read_line(&mut s).unwrap();
                        for c in s.chars().rev() {
                            self.data[0].push(Num::from_num(c as isize));
                        }
                        match self.data[0].pop() {
                            Some(n) => n,
                            None => Num::nan(),
                        }
                    } else {
                        Num::nan()
                    }
                },
            }
        } else {
            Num::nan()
        }"
        } else {
            "match self.data.entry(idx).or_insert(Vec::new()).pop() {
            Some(n) => n,
            None => {
                if idx == 0 {
                    let mut s = String::new();
                    std::io::stdin().read_line(&mut s).unwrap();
                    for c in s.chars().rev() {
                        self.data.get_mut(&0).unwrap().push(Num::from_num(c as isize));
                    }
                    match self.data.get_mut(&0).unwrap().pop() {
                        Some(n) => n,
                        None => Num::nan(),
                    }
                } else {
                    Num::nan()
                }
            },
        }"
        },
        "
    }

    fn push(&mut self, idx: usize, num: Num) {
        ",
        if opt {
            "if idx < self.data.len() {
            if !self.data[idx].is_empty() || !num.is_nan() {
                self.data[idx].push(num);
            }
         }"
        } else {
            "let st = self.data.entry(idx).or_insert(Vec::new());
        if !st.is_empty() || !num.is_nan() {
            st.push(num);
        }"
        },
        "
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
