use crate::code;

fn fn_print(s: String) -> String {
    format!("\nprint!({:?});", s)
}

pub fn build_source<T>(mut state: T, code: &Vec<T::CodeType>) -> String
    where
        T: code::State,
{
    let mut res = String::from(
        "\
use hyeong_build::big_number;
use hyeong_build::number;

fn main() {",
    );

    if !state.get_stack(1).is_empty() {
        res.push_str(&*fn_print(state.get_stack(1).iter().map(|num| num.floor().to_int() as u8 as char).collect()));
        state.get_stack(1).clear();
    }

    if !state.get_stack(2).is_empty() {
        res.push_str(&*fn_print(state.get_stack(2).iter().map(|num| num.floor().to_int() as u8 as char).collect()));
        state.get_stack(2).clear();
    }

    // do something

    res.push_str(
        "
}",
    );
    res
}
