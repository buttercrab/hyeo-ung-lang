use crate::code;
use std::any::Any;

fn fn_print(s: String) -> String {
    format!("\nprint!({:?});", s)
}

pub fn build_source<T>(mut state: T, code: &Vec<T::CodeType>) -> String
where
    T: code::State + 'static,
{
    let opt = state.type_id() == code::OptCode::new(0, 0, 0, 0, code::Area::Nil).type_id();

    let mut res = String::from(format!(
        "{}{}{}{}{}",
        "\
use hyeong_build::big_number;
use hyeong_build::number;

struct Stack {
    data: ",
        if opt { "Vec<" } else { "HashMap<usize, " },
        "Vec<Num>>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            data: ",
        if opt { "Vec<" } else { "HashMap<usize, " },
        "Vec<Num>>::new(),
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
    let mut point = HashMap<u128, usize>::new();
    let mut state = 0usize;
    let mut last = 0usize;
",
    ));

    if !state.get_stack(1).is_empty() {
        res.push_str(&*fn_print(
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
