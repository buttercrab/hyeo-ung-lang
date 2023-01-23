use number::num::Num;
#[cfg(not(feature = "opt"))]
use std::collections::HashMap;

struct Stack {
    #[cfg(feature = "opt")]
    data: Vec<Vec<Num>>,
    #[cfg(not(feature = "opt"))]
    data: HashMap<usize, Vec<Num>>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            #[cfg(feature = "opt")]
            data: vec![Vec::new(); env!("STACK_SIZE").parse().unwrap()],
            #[cfg(not(feature = "opt"))]
            data: HashMap::new(),
        }
    }

    fn pop(&mut self, index: usize) -> Num {
        if index == 1 {
            std::process::exit(0);
        } else if index == 2 {
            std::process::exit(1);
        } else if cfg!(feature = "opt") {
            self.data[index].pop().unwrap_or_else(|| {
                if index == 0 {
                    let mut s = String::new();
                    std::io::stdin().read_line(&mut s).unwrap();
                    for c in s.chars().rev() {
                        self.data[0].push(Num::from_num(c as isize));
                    }
                    self.data[0].pop().unwrap_or_else(Num::nan)
                } else {
                    Num::nan()
                }
            })
        } else {
            self.data
                .entry(index)
                .or_insert_with(Vec::new)
                .pop()
                .unwrap_or_else(|| {
                    if index == 0 {
                        let mut s = String::new();
                        std::io::stdin().read_line(&mut s).unwrap();
                        let v = self.data.get_mut(&0).unwrap();
                        for c in s.chars().rev() {
                            v.push(Num::from_num(c as isize));
                        }
                        v.pop().unwrap_or_else(Num::nan)
                    } else {
                        Num::nan()
                    }
                })
        }
    }

    fn push(&mut self, index: usize, value: Num) {
        if index == 1 {
            if value.is_pos() {
                print!("{}", std::char::from_u32(value.floor().to_int()).unwrap());
            } else {
                print!("{}", -&value);
            }
        } else if index == 2 {
            if value.is_pos() {
                eprint!("{}", std::char::from_u32(value.floor().to_int()).unwrap());
            } else {
                eprint!("{}", -&value);
            }
        }

        let v = if cfg!(feature = "opt") {
            self.data.get_mut(index).unwrap()
        } else {
            self.data.entry(index).or_insert_with(Vec::new)
        };

        if !v.is_empty() || !value.is_nan() {
            v.push(value);
        }
    }
}
