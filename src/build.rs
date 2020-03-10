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

fn command(indent: usize, c: &impl Code) -> String {
    String::from(format!(
        "{}{}",
        match c.get_type() {
            0 => {
                format!(
                    "\n{}stack.push(cur, Num::from_num({}));",
                    make_indent(indent),
                    c.get_hangul_count() * c.get_dot_count()
                )
            }
            1 => {
                format!(
                    "\n{0}let mut n = Num::zero();\
                     \n{0}for _ in 0..{1} {{\
                     \n{0}    n += &stack.pop(cur);\
                     \n{0}}}\
                     \n{0}stack.push({2}, n);",
                    make_indent(indent),
                    c.get_hangul_count(),
                    c.get_dot_count()
                )
            }
            2 => {
                format!(
                    "\n{0}let mut n = Num::one();\
                     \n{0}for _ in 0..{1} {{\
                     \n{0}    n *= &stack.pop(cur);\
                     \n{0}}}\
                     \n{0}stack.push({2}, n);",
                    make_indent(indent),
                    c.get_hangul_count(),
                    c.get_dot_count()
                )
            }
            3 => {
                format!(
                    "\n{0}let mut n = Num::zero();\
                     \n{0}let mut v = Vec::with_capacity({1});\
                     \n{0}for _ in 0..{1} {{\
                     \n{0}    v.push(stack.pop(cur));\
                     \n{0}}}\
                     \n{0}for mut x in v {{\
                     \n{0}    x.minus();\
                     \n{0}    n += &x;\
                     \n{0}    stack.push(cur, x);\
                     \n{0}}}\
                     \n{0}stack.push({2}, n);",
                    make_indent(indent),
                    c.get_hangul_count(),
                    c.get_dot_count()
                )
            }
            4 => {
                format!(
                    "\n{0}let mut n = Num::one();\
                     \n{0}let mut v = Vec::with_capacity({1});\
                     \n{0}for _ in 0..{1} {{\
                     \n{0}    v.push(stack.pop(cur));\
                     \n{0}}}\
                     \n{0}for mut x in v {{\
                     \n{0}    x.flip();\
                     \n{0}    n *= &x;\
                     \n{0}    stack.push(cur, x);\
                     \n{0}}}\
                     \n{0}stack.push({2}, n);",
                    make_indent(indent),
                    c.get_hangul_count(),
                    c.get_dot_count()
                )
            }
            _ => {
                format!(
                    "\n{0}let n = stack.pop(cur);\
                     \n{0}for _ in 0..{1} {{\
                     \n{0}    stack.push({2}, n.clone());\
                     \n{0}}}\
                     \n{0}stack.push(cur, n);\
                     \n{0}cur = {2};",
                    make_indent(indent),
                    c.get_hangul_count(),
                    c.get_dot_count()
                )
            }
        },
        area(indent, c.get_area(), c.get_area_count())
    ))
}

fn area(mut indent: usize, a: &Area, cnt: usize) -> String {
    let mut st = vec![(a, &Area::Nil, false)];
    let mut res = String::new();
    loop {
        loop {
            if let Area::Val { type_, left, right } = st.last().unwrap().0 {
                if *type_ <= 1 {
                    st.push((left, right, false));
                    res.push_str(&*format!(
                        "\n{0}match stack.pop(cur).partial_cmp(&Num::from_num({1})) {{\
                         \n{0}    Some(std::cmp::Ordering::{2}) => {{",
                        make_indent(indent),
                        cnt,
                        if *type_ == 0 { "Less" } else { "Equal" }
                    ));
                    indent += 2;
                    continue;
                } else {
                    if *type_ < 13 {
                        res.push_str(&*format!(
                            "\n{0}let v = *point.entry({1}u128).or_insert(state);\
                             \n{0}if v != state {{\
                             \n{0}    last = Option::Some(state);\
                             \n{0}    state = v;\
                             \n{0}    continue;\
                             \n{0}}}",
                            make_indent(indent),
                            ((cnt as u128) << 4) + *type_ as u128
                        ));
                    } else {
                        res.push_str(&*format!(
                            "\n{0}if let Option::Some(v) = last {{\
                             \n{0}    state = v;\
                             \n{0}    continue;\
                             \n{0}}}",
                            make_indent(indent)
                        ));
                    }
                    break;
                }
            } else {
                break;
            }
        }

        while st.len() > 1 && st.last().unwrap().2 {
            st.pop();
            indent -= 2;
            res.push_str(&*format!(
                "\n{0}    }}\
                 \n{0}}}",
                make_indent(indent)
            ));
        }

        if st.len() > 1 {
            let (left, right, _) = st.pop().unwrap();
            st.push((right, left, true));
            indent -= 1;
            res.push_str(&*format!(
                "\n{0}}}\
                 \n{0}_ => {{",
                make_indent(indent)
            ));
            indent += 1;
        } else {
            break res;
        }
    }
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

    #[allow(dead_code)]
    fn pop(&mut self, idx: usize) -> Num {
        if idx == 1 {
            std::process::exit(0);
        }
        if idx == 2 {
            std::process::exit(1);
        }
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

    #[allow(dead_code)]
    fn push(&mut self, idx: usize, num: Num) {
        if idx == 1 {
            if num.is_pos() {
                print!(\"{}\", num.floor().to_int() as u8 as char);
            } else {
                print!(\"{}\", -&num);
            }
            return;
        }
        if idx == 2 {
            if num.is_pos() {
                eprint!(\"{}\", num.floor().to_int() as u8 as char);
            } else {
                eprint!(\"{}\", -&num);
            }
            return;
        }
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
    #[allow(unused_mut, unused_variables)]
    let mut stack = Stack::new();
    #[allow(unused_mut, unused_variables)]
    let mut point: HashMap<u128, usize> = HashMap::new();
    let mut state = 0usize;
    #[allow(unused_mut, unused_variables, unused_assignments)]
    let mut last = Option::None;
    #[allow(unused_mut, unused_variables, unused_assignments)]
    let mut cur = 3usize;
",
    ));

    if opt {
        for i in state.get_all_stack_index() {
            if state.get_stack(i).is_empty() {
                continue;
            }
            res.push_str(&*format!(
                "
    stack.data[{}] = vec!{:?}.iter().map(|x| Num::from_num(*x)).collect();",
                i,
                state.get_stack(i)
            ));
        }

        for (a, b) in state.get_all_point() {
            res.push_str(&*format!(
                "
    point.insert({}u128, {});",
                a, b
            ));
        }

        res.push_str(&*format!(
            "
    cur = {};",
            state.current_stack()
        ));

        res.push_str(&*format!(
            "
    last = Option::{};",
            match state.get_latest_loc() {
                Some(v) => format!("Some({})", v),
                None => "None".to_string(),
            }
        ))
    }

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
                if !codes.last().unwrap().is_empty() {
                    codes.push(vec![c.clone()]);
                } else {
                    codes.last_mut().unwrap().push(c.clone());
                }
                codes.push(Vec::new());
            }
            Area::Nil => {
                codes.last_mut().unwrap().push(c.clone());
            }
        }
    }

    if codes.last().unwrap().is_empty() {
        codes.pop().unwrap();
    }

    if codes.len() > 0 {
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
    }",
        );
    }
    res.push_str(
        "
}",
    );
    res
}
