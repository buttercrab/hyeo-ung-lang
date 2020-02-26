use hyeo_ung_lang::big_number::Num;

fn main() {
    let mut a = Num::one();
    let b = Num::new(2);
    a += &b;

    print!("{}", a.to_string())
}
