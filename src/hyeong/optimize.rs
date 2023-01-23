use crate::hyeong::code::{Code, CodeType, OptCode, UnOptCode};
use crate::hyeong::state::OptState;
use anyhow::Result;
use std::collections::{BTreeSet, HashMap};

/// Optimization helper function for level 2 optimization
// fn opt_execute<T>(
//     ipt: &mut impl ReadLine,
//     out: &mut impl Write,
//     err: &mut impl Write,
//     mut state: T,
//     code: &T::CodeType,
// ) -> Result<(T, bool)>
// where
//     T: State + Clone,
// {
//     let state_clone = state.clone();
//     let mut cur_loc = state.push_code((*code).clone());
//     let length = cur_loc + 1;
//     let mut exec_count = 0;
//     while cur_loc < length {
//         if exec_count >= 100 {
//             return Ok((state_clone, false));
//         }
//
//         let code = (*state.get_code(cur_loc)).clone();
//         let mut cur_stack = state.current_stack();
//
//         match code.type_() {
//             CodeType::Hyeong => {
//                 state.push_stack(
//                     out,
//                     err,
//                     cur_stack,
//                     &Num::from_num(code.hangul_count() as isize)
//                         * &Num::from_num(code.dot_count() as isize),
//                 )?;
//             }
//             CodeType::Hang => {
//                 let mut n = Num::zero();
//                 for _ in 0..code.hangul_count() {
//                     if cur_stack <= 2 {
//                         return Ok((state_clone, false));
//                     }
//                     n += &pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?;
//                 }
//                 push_stack_wrap(out, err, &mut state, code.dot_count(), n)?;
//             }
//             CodeType::Hat => {
//                 let mut n = Num::one();
//                 for _ in 0..code.hangul_count() {
//                     if cur_stack <= 2 {
//                         return Ok((state_clone, false));
//                     }
//                     n *= &pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?;
//                 }
//                 push_stack_wrap(out, err, &mut state, code.dot_count(), n)?;
//             }
//             CodeType::Heut => {
//                 let mut n = Num::zero();
//                 let mut v = Vec::with_capacity(code.hangul_count());
//
//                 for _ in 0..code.hangul_count() {
//                     if cur_stack <= 2 {
//                         return Ok((state_clone, false));
//                     }
//                     v.push(pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?);
//                 }
//
//                 for mut x in v {
//                     x.minus();
//                     n += &x;
//                     push_stack_wrap(out, err, &mut state, cur_stack, x)?;
//                 }
//
//                 push_stack_wrap(out, err, &mut state, code.dot_count(), n)?;
//             }
//             CodeType::Heup => {
//                 let mut n = Num::one();
//                 let mut v = Vec::with_capacity(code.hangul_count());
//
//                 for _ in 0..code.hangul_count() {
//                     if cur_stack <= 2 {
//                         return Ok((state_clone, false));
//                     }
//                     v.push(pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?);
//                 }
//
//                 for mut x in v {
//                     x.flip();
//                     n *= &x;
//                     push_stack_wrap(out, err, &mut state, cur_stack, x)?;
//                 }
//
//                 push_stack_wrap(out, err, &mut state, code.dot_count(), n)?;
//             }
//             CodeType::Heuk => {
//                 if cur_stack <= 2 {
//                     return Ok((state_clone, false));
//                 }
//                 let n = pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?;
//                 for _ in 0..code.hangul_count() {
//                     push_stack_wrap(out, err, &mut state, code.dot_count(), n.clone())?;
//                 }
//                 push_stack_wrap(out, err, &mut state, cur_stack, n)?;
//                 state.set_current_stack(code.dot_count());
//             }
//         }
//
//         cur_stack = state.current_stack();
//         let area_type = match code.area().calc(code.area_count(), || {
//             if cur_stack <= 2 {
//                 Err(anyhow!("Stack underflow"))
//             } else {
//                 pop_stack_wrap(ipt, out, err, &mut state, cur_stack)
//             }
//         }) {
//             Ok(value) => value,
//             Err(_) => return Ok((state_clone, false)),
//         };
//
//         if area_type != 0 {
//             if area_type != 13 {
//                 let id = ((code.area_count() as u128) << 4) + area_type as u128;
//                 match state.get_point(id) {
//                     Some(value) => {
//                         if cur_loc != value {
//                             state.set_latest_loc(cur_loc);
//                             cur_loc = value;
//                             exec_count += 1;
//                             continue;
//                         }
//                     }
//                     None => state.set_point(id, cur_loc),
//                 }
//             } else if let Some(loc) = state.get_latest_loc() {
//                 cur_loc = loc;
//                 exec_count += 1;
//                 continue;
//             }
//         }
//
//         cur_loc += 1;
//     }
//
//     Ok((state, true))
// }

fn optimize1(code: Vec<UnOptCode>) -> Result<(OptState, Vec<OptCode>)> {
    let mut now = 3;
    let mut chk = BTreeSet::new();

    for c in &code {
        if matches!(c.type_(), CodeType::Hyeong) {
            continue;
        }
        let _ = chk.insert(now);
        if matches!(c.type_(), CodeType::Heuk) {
            now = c.dot_count();
        }
    }

    let dot_map = chk
        .into_iter()
        .skip_while(|x| *x <= 2)
        .enumerate()
        .map(|(i, x)| (x, i + 3))
        .collect::<HashMap<_, _>>();

    let opt_code = code
        .into_iter()
        .map(|c| {
            let type_ = c.type_();
            let hangul_count = c.hangul_count();
            let dot_count = c.dot_count();
            let area = c.area().clone();
            let area_count = c.area_count();

            if matches!(c.type_(), CodeType::Hyeong) || c.dot_count() <= 2 {
                OptCode::new(type_, hangul_count, dot_count, area_count, area)
            } else {
                let dot_count = dot_map.get(&dot_count).copied().unwrap_or(dot_map.len() + 3);
                OptCode::new(type_, hangul_count, dot_count, area_count, area)
            }
        })
        .collect::<Vec<_>>();

    Ok((OptState::new(dot_map.len() + 3), opt_code))
}

// fn optimize2(mut state: OptState, code: Vec<OptCode>) -> Result<(OptState, Vec<OptCode>)> {
//     let mut out = io::CustomWriter::new(|_| Ok(()));
//     let mut err = io::CustomWriter::new(|_| Ok(()));
//
//     let mut idx = code.len();
//     for (i, opt_code) in code.iter().enumerate() {
//         let (new_state, next) = opt_execute(&mut stdin(), &mut out, &mut err, state, opt_code)?;
//         state = new_state;
//         if !next {
//             idx = i;
//             break;
//         }
//     }
//     let code = code[idx..].to_vec();
//
//     state
//         .get_stack(1)
//         .extend(out.to_string()?.chars().map(|x| Num::from_num(x as isize)));
//     state
//         .get_stack(2)
//         .extend(err.to_string()?.chars().map(|x| Num::from_num(x as isize)));
//     Ok((state, code))
// }

/// Optimization function
///
/// ## Level 1
///
/// In level 1, it analyzes the hyeong code and collect only used index of stack.
/// Then, re-number the indices not to make `HashMap`; using `Vec`
///
/// ## Level 2
///
/// In level 2, it runs code until it gets input, terminates, or too much loop.
/// So, `Hello, World!` code compiles to `print!("Hello, World!")` when unused code is removed.
///
/// # Examples
///
/// ```
/// use hyeong::core::{parse, optimize};
/// use hyeong::core::state::State;
///
/// let a = parse::parse(String::from("형... 항."));
/// let (mut s, c) = optimize::optimize(a, 2).unwrap();
///
/// assert_eq!("3", s.get_stack(1).iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""))
/// ```
pub fn optimize(code: Vec<UnOptCode>, level: u8) -> Result<(OptState, Vec<OptCode>)> {
    let (state, code) = if level >= 1 {
        optimize1(code)?
    } else {
        (OptState::new(1), Vec::new())
    };

    // todo: refactor optimize 2 code
    // if level >= 2 {
    //     optimize2(state, code)
    // } else {
    //     Ok((state, code))
    // }
    Ok((state, code))
}
