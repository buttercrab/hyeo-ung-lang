use crate::code;

pub fn build_source<T>(mut state: T, code: &Vec<T::CodeType>) -> String
where
    T: code::State,
{
    let mut res = String::from(
        "\
use hyeong_build::big_number;
use hyeong_build::number;

fn main() {
",
    );

    if !state.get_stack(1).is_empty() {
        for num in state.get_stack(1).iter() {
            res.push_str(&*format!(
                "\
    print!(\"{}\");
",
                num.floor().to_int() as u8 as char
            ));
        }
        state.get_stack(1).clear();
    }

    if !state.get_stack(2).is_empty() {
        for num in state.get_stack(2).iter() {
            res.push_str(&*format!(
                "\
    eprint!(\"{}\");
",
                num.floor().to_int() as u8 as char
            ));
        }
        state.get_stack(2).clear();
    }

    // do something

    res.push_str(
        "\
}
",
    );
    res
}
