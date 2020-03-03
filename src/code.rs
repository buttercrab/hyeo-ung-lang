use std::fmt;

use colored::Colorize;

use crate::parse;

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

fn area_to_string_debug(s: &mut String, area: &Area) {
    match area {
        Area::Val {
            ref type_,
            ref left,
            ref right
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
            ref right
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

    fn set_hangul_count(&mut self, count: usize);

    fn get_dot_count(&self) -> usize;

    fn set_dot_count(&mut self, count: usize);

    fn get_area(&self) -> &Area;

    fn set_area(&mut self, area: Area);

    fn get_area_count(&self) -> usize;
}

pub struct UnOptCode {
    // 0: 형, 혀엉, 혀어엉, 혀어어엉 ...
    // 1: 항, 하앙, 하아앙, 하아아앙 ...
    // 2: 핫, 하앗, 하아앗, 하아아앗 ...
    // 3: 흣, 흐읏, 흐으읏, 흐으으읏 ...
    // 4: 흡, 흐읍, 흐으읍, 흐으으읍 ...
    // 5: 흑, 흐윽, 흐으윽, 흐으으윽 ...
    type_: u8,
    cnt1: usize,
    cnt2: usize,
    loc: (usize, usize),
    area: Area,
}

impl UnOptCode {
    pub fn new(type_: u8, loc: (usize, usize)) -> Self {
        UnOptCode {
            type_,
            cnt1: 1,
            cnt2: 0,
            loc,
            area: Area::Nil,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}_{}_{} : {}",
                (&*format!("{}:{}", self.loc.0, self.loc.1)).yellow()
                , parse::COMMANDS[self.type_ as usize], self.cnt1, self.cnt2, self.area)
    }

    pub fn get_location(&self) -> (usize, usize) { self.loc }
}

impl fmt::Debug for UnOptCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut area = String::new();
        area_to_string_debug(&mut area, &self.area);
        write!(f, "type: {}, cnt1: {}, cnt2: {}, area: {:?}", self.type_, self.cnt1, self.cnt2, area)
    }
}

impl Code for UnOptCode {
    fn get_type(&self) -> u8 { self.type_ }

    fn get_hangul_count(&self) -> usize { self.cnt1 }

    fn set_hangul_count(&mut self, count: usize) { self.cnt1 = count; }

    fn get_dot_count(&self) -> usize { self.cnt2 }

    fn set_dot_count(&mut self, count: usize) { self.cnt2 = count; }

    fn get_area(&self) -> &Area { &self.area }

    fn set_area(&mut self, area: Area) { self.area = area; }

    fn get_area_count(&self) -> usize { self.cnt1 * self.cnt2 }
}

pub struct OptCode {
    type_: u8,
    cnt1: usize,
    cnt2: usize,
    cnt3: usize,
    area: Area,
}

impl OptCode {
    pub fn new(type_: u8) -> Self {
        OptCode {
            type_,
            cnt1: 0,
            cnt2: 0,
            cnt3: 0,
            area: Area::Nil,
        }
    }
}

impl OptCode {
    pub fn set_area_count(&mut self, count: usize) { self.cnt3 = count; }
}

impl Code for OptCode {
    fn get_type(&self) -> u8 { self.type_ }

    fn get_hangul_count(&self) -> usize { self.cnt1 }

    fn set_hangul_count(&mut self, count: usize) { self.cnt1 = count; }

    fn get_dot_count(&self) -> usize { self.cnt2 }

    fn set_dot_count(&mut self, count: usize) { self.cnt2 = count; }

    fn get_area(&self) -> &Area { &self.area }

    fn set_area(&mut self, area: Area) { self.area = area; }

    fn get_area_count(&self) -> usize { self.cnt3 }
}