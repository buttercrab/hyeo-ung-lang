use crate::code;
use crate::code::{Area, Code};

fn make_indent(value: usize) -> String {
    std::iter::repeat(' ').take(value * 4).collect::<String>()
}

fn fn_print(indent: usize, s: String) -> String {
    format!("\n{}print!({:?});", make_indent(indent), s)
}

fn fn_eprint(indent: usize, s: String) -> String {
    format!("\n{}eprint!({:?});", make_indent(indent), s)
}

fn fn_exit(indent: usize, code: i32) -> String {
    format!("\n{}std::process::exit({});", make_indent(indent), code)
}

fn command(indent: usize, c: &impl Code) -> String {
    "\n".to_string()
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
        res.push_str(&*fn_eprint(
            indent,
            state
                .get_stack(2)
                .iter()
                .map(|num| num.floor().to_int() as u8 as char)
                .collect(),
        ));
        state.get_stack(2).clear();
    }

    let mut codes: Vec<Vec<T::CodeType>> = Vec::new();
    codes.push(Vec::new());

    for c in code {
        match c.get_area() {
            Area::Val {
                type_: _,
                left: _,
                right: _,
            } => {
                codes.push(vec![c.clone()]);
                codes.push(Vec::new());
            }
            Area::Nil => {
                codes.last_mut().unwrap().push(c.clone());
            }
        }
    }

    res.push_str(&*format!(
        "
    while state < {} {{",
        codes.len()
    ));
    indent += 1;

    let mut stack = vec![(codes.len(), false)];

    for i in 0..codes.len() {
        while stack.last().unwrap().0 > 1 {
            stack.push((stack.last().unwrap().0 / 2, false));
            res.push_str(&*format!(
                "\n{}if state < {} {{",
                make_indent(indent),
                stack.last().unwrap().0 + i
            ));
            indent += 1;
        }

        for item in &codes[i] {
            res.push_str(&*command(indent, item));
        }

        while stack.len() > 1 && stack.last().unwrap().1 {
            stack.pop();
            indent -= 1;
            res.push_str(&*format!("\n{}}}", make_indent(indent)));
        }

        if i != codes.len() - 1 {
            let last = stack.pop().unwrap().0;
            stack.push((stack.last().unwrap().0 - last, true));
            res.push_str(&*format!("\n{}}} else {{", make_indent(indent - 1)));
        }
    }

    res.push_str(
        "
        state += 1;
    }
}",
    );
    res
}
