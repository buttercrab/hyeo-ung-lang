use crate::number::Num;
use std::cmp::Ordering;
use std::fmt;

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

/// Calculates Area when value and stack is given.
///
/// # Examples
/// ```
/// use hyeong::number::Num;
/// use hyeong::area::{Area, calc};
///
/// let a = Area::new(10);
/// assert_eq!(10, calc(&a, 1, || Option::Some(Num::one())).unwrap());
/// ```
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
                    .partial_cmp(&Num::from_num(area_value as isize))
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
                    .partial_cmp(&Num::from_num(area_value as isize))
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

/// `Area` to string in debug mode
/// it builds the string as it iterates post-order
pub fn area_to_string_debug(s: &mut String, area: &Area) {
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
    /// `Area` to string in debug mode
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::area::Area;
    ///
    /// let a = Area::Val {
    ///     type_: 0,
    ///     left: Box::new(Area::new(2)),
    ///     right: Box::new(Area::Nil),
    /// };
    ///
    /// assert_eq!("?♥_", format!("{:?}", a));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        area_to_string_debug(&mut s, self);
        write!(f, "{}", s)
    }
}

/// `Area` to string in formatting
/// it builds the string as it iterates infix-order.
pub fn area_to_string_display(s: &mut String, area: &Area) {
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
    /// `Area` to string in formatting
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::area::Area;
    ///
    /// let a = Area::Val {
    ///     type_: 0,
    ///     left: Box::new(Area::new(2)),
    ///     right: Box::new(Area::Nil),
    /// };
    ///
    /// assert_eq!("[♥]?[_]", format!("{}", a));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        area_to_string_display(&mut s, self);
        write!(f, "{}", s)
    }
}
