use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use colored::Colorize;

use crate::number::Num;
use crate::{number, parse};

/// Area Part of each code
/// Since the area has binary operator,
/// It is saved as binary tree(ast).
///
/// # Type
///
/// Each value of `type_` that is representing
///
/// - ` 0: ?`
/// - ` 1: !`
/// - ` 2: ♥`
/// - ` 3: ❤`
/// - ` 4: 💕`
/// - ` 5: 💖`
/// - ` 6: 💗`
/// - ` 7: 💘`
/// - ` 8: 💙`
/// - ` 9: 💚`
/// - `10: 💛`
/// - `11: 💜`
/// - `12: 💝`
/// - `13: ♡`
///
/// # Examples
///
/// ```
/// use hyeong::code;
///
/// let a = code::Area::Val {
///     type_: 0,
///     left: Box::new(code::Area::new(2)),
///     right: Box::new(code::Area::Nil),
/// };
///
/// assert_eq!("[♥]?[_]", format!("{}", a));
/// ```
#[derive(Clone)]
pub enum Area {
    Val {
        type_: u8,
        left: Box<Area>,
        right: Box<Area>,
    },
    Nil,
}

impl Area {
    /// New `Area` that is leaf node
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::code;
    ///
    /// let a = code::Area::new(10);
    /// ```
    pub fn new(type_: u8) -> Area {
        Area::Val {
            type_,
            left: Box::new(Area::Nil),
            right: Box::new(Area::Nil),
        }
    }
}

pub fn calc<T>(area: &Area, area_value: usize, mut pop: T) -> Option<u8>
where
    T: FnMut() -> Option<Num>,
{
    let mut area = area;

    loop {
        match area {
            Area::Val { type_, left, right } => {
                if *type_ == 0 {
                    let v = pop();
                    area = match match v {
                        Some(value) => value,
                        None => return Option::None,
                    }
                    .partial_cmp(&number::Num::from_num(area_value as isize))
                    {
                        Some(Ordering::Less) => left,
                        _ => right,
                    }
                } else if *type_ == 1 {
                    let v = pop();
                    area = match match v {
                        Some(value) => value,
                        None => return Option::None,
                    }
                    .partial_cmp(&number::Num::from_num(area_value as isize))
                    {
                        Some(Ordering::Equal) => left,
                        _ => right,
                    }
                } else {
                    break Option::Some(*type_);
                }
            }
            Area::Nil => {
                break Option::Some(0);
            }
        }
    }
}

fn area_to_string_debug(s: &mut String, area: &Area) {
    match area {
        Area::Val {
            ref type_,
            ref left,
            ref right,
        } => {
            let c = "?!♥❤💕💖💗💘💙💚💛💜💝♡".chars().collect::<Vec<char>>()[*type_ as usize];
            s.push(c);
            if *type_ <= 1 {
                area_to_string_debug(s, left);
                area_to_string_debug(s, right);
            }
        }
        Area::Nil => {
            s.push('_');
        }
    }
}

impl fmt::Debug for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        area_to_string_debug(&mut s, self);
        write!(f, "{}", s)
    }
}

fn area_to_string_display(s: &mut String, area: &Area) {
    match area {
        Area::Val {
            ref type_,
            ref left,
            ref right,
        } => {
            let c = "?!♥❤💕💖💗💘💙💚💛💜💝♡".chars().collect::<Vec<char>>()[*type_ as usize];
            if *type_ <= 1 {
                s.push('[');
                area_to_string_display(s, left);
                s.push(']');
                s.push(c);
                s.push('[');
                area_to_string_display(s, right);
                s.push(']');
            } else {
                s.push(c);
            }
        }
        Area::Nil => {
            s.push('_');
        }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        area_to_string_display(&mut s, self);
        write!(f, "{}", s)
    }
}

pub trait Code {
    fn get_type(&self) -> u8;

    fn get_hangul_count(&self) -> usize;

    fn get_dot_count(&self) -> usize;

    fn get_area(&self) -> &Area;

    fn get_area_count(&self) -> usize;

    fn clone(&self) -> Self;
}

#[derive(Clone)]
pub struct OptCode {
    type_: u8,
    hangul_count: usize,
    dot_count: usize,
    area_count: usize,
    area: Area,
}

impl OptCode {
    pub fn new(
        type_: u8,
        hangul_count: usize,
        dot_count: usize,
        area_count: usize,
        area: Area,
    ) -> OptCode {
        OptCode {
            type_,
            hangul_count,
            dot_count,
            area_count,
            area,
        }
    }
}

impl Code for OptCode {
    fn get_type(&self) -> u8 {
        self.type_
    }

    fn get_hangul_count(&self) -> usize {
        self.hangul_count
    }

    fn get_dot_count(&self) -> usize {
        self.dot_count
    }

    fn get_area(&self) -> &Area {
        &self.area
    }

    fn get_area_count(&self) -> usize {
        self.area_count
    }

    fn clone(&self) -> OptCode {
        OptCode {
            type_: self.type_,
            hangul_count: self.hangul_count,
            dot_count: self.dot_count,
            area_count: self.area_count,
            area: self.area.clone(),
        }
    }
}

#[derive(Clone)]
pub struct UnOptCode {
    // 0: 형, 혀엉, 혀어엉, 혀어어엉 ...
    // 1: 항, 하앙, 하아앙, 하아아앙 ...
    // 2: 핫, 하앗, 하아앗, 하아아앗 ...
    // 3: 흣, 흐읏, 흐으읏, 흐으으읏 ...
    // 4: 흡, 흐읍, 흐으읍, 흐으으읍 ...
    // 5: 흑, 흐윽, 흐으윽, 흐으으윽 ...
    type_: u8,
    hangul_count: usize,
    dot_count: usize,
    loc: (usize, usize),
    area: Area,
    code: String,
}

impl UnOptCode {
    pub fn new(
        type_: u8,
        hangul_count: usize,
        dot_count: usize,
        loc: (usize, usize),
        area: Area,
        code: String,
    ) -> UnOptCode {
        UnOptCode {
            type_,
            hangul_count,
            dot_count,
            loc,
            area,
            code,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {}_{}_{} : {}",
            (&*format!("{}:{}", self.loc.0, self.loc.1)).yellow(),
            parse::COMMANDS[self.type_ as usize],
            self.hangul_count,
            self.dot_count,
            self.area
        )
    }

    pub fn get_location(&self) -> (usize, usize) {
        self.loc
    }

    pub fn get_raw(&self) -> String {
        self.code.clone()
    }
}

impl fmt::Debug for UnOptCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut area = String::new();
        area_to_string_debug(&mut area, &self.area);
        write!(
            f,
            "type: {}, cnt1: {}, cnt2: {}, area: {:?}",
            self.type_, self.hangul_count, self.dot_count, area
        )
    }
}

impl Code for UnOptCode {
    fn get_type(&self) -> u8 {
        self.type_
    }

    fn get_hangul_count(&self) -> usize {
        self.hangul_count
    }

    fn get_dot_count(&self) -> usize {
        self.dot_count
    }

    fn get_area(&self) -> &Area {
        &self.area
    }

    fn get_area_count(&self) -> usize {
        self.hangul_count * self.dot_count
    }

    fn clone(&self) -> UnOptCode {
        UnOptCode {
            type_: self.type_,
            hangul_count: self.hangul_count,
            dot_count: self.dot_count,
            loc: self.loc.clone(),
            area: self.area.clone(),
            code: self.code.clone(),
        }
    }
}

pub trait State {
    type CodeType: Code;

    fn get_all_stack_index(&self) -> Vec<usize>;

    fn stack_size(&self) -> usize;

    fn current_stack(&self) -> usize;

    fn set_current_stack(&mut self, cur: usize);

    fn get_stack(&mut self, idx: usize) -> &mut Vec<number::Num>;

    fn push_stack(&mut self, idx: usize, num: number::Num) {
        let st = self.get_stack(idx);
        if !st.is_empty() || !num.is_nan() {
            st.push(num);
        }
    }

    fn pop_stack(&mut self, idx: usize) -> number::Num {
        match self.get_stack(idx).pop() {
            Some(t) => t,
            None => number::Num::nan(),
        }
    }

    fn get_code(&self, loc: usize) -> &Self::CodeType;

    fn push_code(&mut self, code: Self::CodeType) -> usize;

    fn get_all_code(&self) -> Vec<Self::CodeType>;

    fn set_point(&mut self, id: u128, loc: usize);

    fn get_point(&self, id: u128) -> Option<usize>;

    fn get_all_point(&self) -> Vec<(u128, usize)>;

    fn set_latest_loc(&mut self, loc: usize);

    fn get_latest_loc(&self) -> Option<usize>;
}

#[derive(Clone)]
pub struct OptState {
    stack: Vec<Vec<number::Num>>,
    code: Vec<OptCode>,
    point: HashMap<u128, usize>,
    cur: usize,
    latest: Option<usize>,
}

impl OptState {
    pub fn new(size: usize) -> OptState {
        OptState {
            stack: vec![Vec::new(); size],
            code: vec![],
            point: HashMap::new(),
            cur: 3,
            latest: None,
        }
    }
}

impl State for OptState {
    type CodeType = OptCode;

    fn get_all_stack_index(&self) -> Vec<usize> {
        (0..self.stack.len()).collect()
    }

    fn stack_size(&self) -> usize {
        self.stack.len()
    }

    fn current_stack(&self) -> usize {
        self.cur
    }

    fn set_current_stack(&mut self, cur: usize) {
        self.cur = cur;
    }

    fn get_stack(&mut self, idx: usize) -> &mut Vec<number::Num> {
        self.stack[idx].as_mut()
    }

    fn push_stack(&mut self, idx: usize, num: number::Num) {
        if idx < self.stack.len() {
            if !self.stack[idx].is_empty() || !num.is_nan() {
                self.get_stack(idx).push(num);
            }
        }
    }

    fn pop_stack(&mut self, idx: usize) -> number::Num {
        if idx < self.stack.len() {
            match self.get_stack(idx).pop() {
                Some(t) => t,
                None => number::Num::nan(),
            }
        } else {
            number::Num::nan()
        }
    }

    fn get_code(&self, loc: usize) -> &Self::CodeType {
        &self.code[loc]
    }

    fn push_code(&mut self, code: Self::CodeType) -> usize {
        self.code.push(code);
        self.code.len() - 1
    }

    fn get_all_code(&self) -> Vec<Self::CodeType> {
        self.code.clone()
    }

    fn set_point(&mut self, id: u128, loc: usize) {
        self.point.insert(id, loc);
    }

    fn get_point(&self, id: u128) -> Option<usize> {
        self.point.get(&id).map(|&x| x)
    }

    fn get_all_point(&self) -> Vec<(u128, usize)> {
        let mut v = Vec::with_capacity(self.point.len());
        for (a, b) in &self.point {
            v.push((*a, *b));
        }
        v
    }

    fn set_latest_loc(&mut self, loc: usize) {
        self.latest = Option::Some(loc);
    }

    fn get_latest_loc(&self) -> Option<usize> {
        self.latest
    }
}

#[derive(Clone)]
pub struct UnOptState {
    stack: HashMap<usize, Vec<number::Num>>,
    code: Vec<UnOptCode>,
    point: HashMap<u128, usize>,
    cur: usize,
    latest: Option<usize>,
}

impl UnOptState {
    pub fn new() -> UnOptState {
        UnOptState {
            stack: HashMap::new(),
            code: vec![],
            point: HashMap::new(),
            cur: 3,
            latest: None,
        }
    }
}

impl State for UnOptState {
    type CodeType = UnOptCode;

    fn get_all_stack_index(&self) -> Vec<usize> {
        let mut v = Vec::with_capacity(self.stack.len());
        for (i, _) in &self.stack {
            v.push(*i);
        }
        v
    }

    fn stack_size(&self) -> usize {
        self.stack.len()
    }

    fn current_stack(&self) -> usize {
        self.cur
    }

    fn set_current_stack(&mut self, cur: usize) {
        self.cur = cur;
    }

    fn get_stack(&mut self, idx: usize) -> &mut Vec<number::Num> {
        self.stack.entry(idx).or_insert(Vec::new())
    }

    fn get_code(&self, loc: usize) -> &Self::CodeType {
        &self.code[loc]
    }

    fn push_code(&mut self, code: Self::CodeType) -> usize {
        self.code.push(code);
        self.code.len() - 1
    }

    fn get_all_code(&self) -> Vec<Self::CodeType> {
        self.code.clone()
    }

    fn set_point(&mut self, id: u128, loc: usize) {
        self.point.insert(id, loc);
    }

    fn get_point(&self, id: u128) -> Option<usize> {
        self.point.get(&id).map(|&x| x)
    }

    fn get_all_point(&self) -> Vec<(u128, usize)> {
        let mut v = Vec::with_capacity(self.point.len());
        for (a, b) in &self.point {
            v.push((*a, *b));
        }
        v
    }

    fn set_latest_loc(&mut self, loc: usize) {
        self.latest = Option::Some(loc);
    }

    fn get_latest_loc(&self) -> Option<usize> {
        self.latest
    }
}

impl fmt::Debug for UnOptState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("current stack: {}\n", self.cur);
        let mut v = self.stack.iter().collect::<Vec<_>>();
        v.sort_by(|x, y| x.0.cmp(&y.0));
        for (a, b) in v {
            s.push_str(&*format!("stack {}: {:?}\n", a, b));
        }
        write!(f, "{}", s)
    }
}
